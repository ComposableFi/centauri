// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(unused_variables)]

pub mod chain;
pub mod command;
pub mod events;
pub mod logging;
mod macros;
pub mod packets;
pub mod queue;
pub mod substrate;
mod utils;

use crate::utils::RecentStream;
use anyhow::anyhow;
use events::{has_packet_events, parse_events};
use futures::{future::ready, StreamExt, TryFutureExt};
use ibc::{events::IbcEvent, Height};
use ibc_proto::google::protobuf::Any;
use itertools::Itertools;
use metrics::handler::MetricsHandler;
use primitives::{Chain, IbcProvider, UndeliveredType, UpdateType};
use std::collections::HashSet;

#[derive(Copy, Debug, Clone)]
pub enum Mode {
	/// Run without trying to relay packets or query channel state
	Light,
}

/// Core relayer loop, waits for new finality events and forwards any new [`ibc::IbcEvents`]
/// to the counter party chain.
pub async fn relay<A, B>(
	mut chain_a: A,
	mut chain_b: B,
	mut chain_a_metrics: Option<MetricsHandler>,
	mut chain_b_metrics: Option<MetricsHandler>,
	mode: Option<Mode>,
) -> Result<(), anyhow::Error>
where
	A: Chain,
	B: Chain,
{
	let stream_a = RecentStream::new(chain_a.finality_notifications().await?);
	let stream_b = RecentStream::new(chain_b.finality_notifications().await?);
	let (mut chain_a_finality, mut chain_b_finality) = (stream_a, stream_b);

	// Introduce altering between branches so that each branch gets a chance to execute first after
	// another one
	let mut first_executed = false;

	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			result = chain_a_finality.next(), if !first_executed => {
				log::info!("Got {} finality", chain_a.name());
				first_executed = true;
				process_finality_event(&mut chain_a, &mut chain_b, &mut chain_a_metrics, mode, result, &mut chain_a_finality, &mut chain_b_finality).await?;
			}
			// new finality event from chain B
			result = chain_b_finality.next() => {
				log::info!("Got {} finality", chain_b.name());
				first_executed = false;
				process_finality_event(&mut chain_b, &mut chain_a, &mut chain_b_metrics, mode, result, &mut chain_b_finality, &mut chain_a_finality).await?;
			}
			else => {
				first_executed = false;
			}
		}
	}
}

pub async fn fish<A, B>(chain_a: A, chain_b: B) -> Result<(), anyhow::Error>
where
	A: Chain,
	A::Error: From<B::Error>,
	B: Chain,
	B::Error: From<A::Error>,
{
	// we only care about events where the counterparty light client is updated.
	let (mut chain_a_client_updates, mut chain_b_client_updates) = (
		chain_a.ibc_events().await.filter_map(|ev| {
			ready(match ev {
				IbcEvent::UpdateClient(update) if chain_b.client_id() == *update.client_id() =>
					Some(update),
				_ => None,
			})
		}),
		chain_b.ibc_events().await.filter_map(|ev| {
			ready(match ev {
				IbcEvent::UpdateClient(update) if chain_a.client_id() == *update.client_id() =>
					Some(update),
				_ => None,
			})
		}),
	);

	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			update = chain_a_client_updates.next() => {
				let update = match update {
					Some(update) => update,
					None => break,
				};
				// The corresponding transaction on tendermint may not be indexed yet, so we wait for a bit
				if chain_a.client_type() == "07-tendermint" {
					tokio::time::sleep(chain_a.expected_block_time()).await;
				}
				let message = chain_a.query_client_message(update).await.map_err(|e| { log::info!("error: {}", e); e })?;
				chain_b.check_for_misbehaviour(&chain_a, message).await.map_err(|e| { log::info!("error: {}", e); e })?;
			}
			// new finality event from chain B
			update = chain_b_client_updates.next() => {
				let update = match update {
					Some(update) => update,
					None => break,
				};
				// The corresponding transaction on tendermint may not be indexed yet, so we wait for a bit
				if chain_a.client_type() == "07-tendermint" {
					tokio::time::sleep(chain_a.expected_block_time()).await;
				}
				let message = chain_b.query_client_message(update).await.map_err(|e| { log::info!("error: {}", e); e })?;
				chain_a.check_for_misbehaviour(&chain_b, message).await.map_err(|e| { log::info!("error: {}", e); e })?;
			}
		}
	}

	Ok(())
}

async fn process_finality_event<A: Chain, B: Chain>(
	source: &mut A,
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	mode: Option<Mode>,
	result: Option<A::FinalityEvent>,
	stream_source: &mut RecentStream<A::FinalityEvent>,
	stream_sink: &mut RecentStream<B::FinalityEvent>,
) -> anyhow::Result<()> {
	match result {
		// stream closed
		None => {
			log::info!("Stream closed for {}", source.name());
			*stream_source = loop {
				match source.finality_notifications().await {
					Ok(stream) => break RecentStream::new(stream),
					Err(e) => {
						log::error!("Failed to get finality notifications for {} {:?}. Trying again in 30 seconds...", source.name(), e);
						tokio::time::sleep(std::time::Duration::from_secs(30)).await;
						let _ = source.reconnect().await;
					},
				};
			};
			*stream_sink = loop {
				match sink.finality_notifications().await {
					Ok(stream) => break RecentStream::new(stream),
					Err(e) => {
						log::error!("Failed to get finality notifications for {} {:?}. Trying again in 30 seconds...", sink.name(), e);
						tokio::time::sleep(std::time::Duration::from_secs(30)).await;
						let _ = sink.reconnect().await;
					},
				};
			};
		},
		Some(finality_event) => {
			log::info!("=======================================================");
			log::info!("Received finality notification from {}", source.name(),);

			let result =
				process_some_finality_event(source, sink, metrics, mode, finality_event).await;

			match result {
				Ok(()) => {
					let sink_initial_rpc_call_delay = sink.initial_rpc_call_delay();
					let source_initial_rpc_call_delay = source.initial_rpc_call_delay();
					sink.set_rpc_call_delay(sink_initial_rpc_call_delay);
					source.set_rpc_call_delay(source_initial_rpc_call_delay);
				},
				Err(e) => {
					log::error!("{}", e);
					match sink.handle_error(&e).and_then(|_| source.handle_error(&e)).await {
						Ok(_) => (),
						Err(e) => {
							log::error!("Failed to handle error {:?}", e)
						},
					}
				},
			}
		},
	}
	Ok(())
}

async fn process_some_finality_event<A: Chain, B: Chain>(
	source: &mut A,
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	mode: Option<Mode>,
	finality_event: <A as IbcProvider>::FinalityEvent,
) -> anyhow::Result<()> {
	let updates = source
		.query_latest_ibc_events(finality_event, &*sink)
		.await
		.map_err(|e| anyhow!("Failed to fetch IBC events for finality event {e}"))?;
	log::trace!(target: "hyperspace", "Received updates count: {}", updates.len());
	// query packets that can now be sent, at this sink height because of connection
	// delay.
	let (ready_packets, mut timeout_msgs) =
		packets::query_ready_and_timed_out_packets(&*source, &*sink)
			.await
			.map_err(|e| anyhow!("Failed to parse events: {:?}", e))?;

	let mut msgs = Vec::new();

	log::trace!(
		target: "hyperspace",
		"{} has undelivered seqs: [{:?}:{}, {:?}:{}, {:?}:{}]", source.name(),
		UndeliveredType::Acks, source.has_undelivered_sequences(UndeliveredType::Acks),
		UndeliveredType::Timeouts, source.has_undelivered_sequences(UndeliveredType::Timeouts),
		UndeliveredType::Recvs, source.has_undelivered_sequences(UndeliveredType::Recvs),
	);

	log::trace!(
		target: "hyperspace",
		"{} has undelivered seqs: [{:?}:{}, {:?}:{}, {:?}:{}]", sink.name(),
		UndeliveredType::Acks, sink.has_undelivered_sequences(UndeliveredType::Acks),
		UndeliveredType::Timeouts, sink.has_undelivered_sequences(UndeliveredType::Timeouts),
		UndeliveredType::Recvs, sink.has_undelivered_sequences(UndeliveredType::Recvs),
	);

	log::trace!(
		target: "hyperspace",
		"Received timeouts count: {}",
		timeout_msgs.len()
	);

	process_updates(source, sink, metrics, mode, updates, &mut msgs, ready_packets.clone()).await?;

	msgs.extend(ready_packets);

	process_messages(sink, metrics, msgs).await?;

	if sink.name() == "solana" && timeout_msgs.len() > 0 {
		log::info!("Inside timeout msgs fetching height");
		let mut timeout_heights = Vec::new();
		if timeout_msgs.len() > 0 {
			for msg in timeout_msgs.iter() {
				let my_message = ibc::core::ics26_routing::msgs::Ics26Envelope::<
					primitives::mock::LocalClientTypes,
				>::try_from(msg.clone())
				.unwrap();
				let timeout_msg = match my_message {
					ibc::core::ics26_routing::msgs::Ics26Envelope::Ics4PacketMsg(packet_msg) =>
						match packet_msg {
							ibc::core::ics04_channel::msgs::PacketMsg::ToPacket(msg) => msg,
							_ => continue,
						},
					_ => continue,
				};
				timeout_heights.push(timeout_msg.proofs.height().revision_height);
			}
			loop {
				let largest_height = timeout_heights.iter().max().unwrap();
				let latest_height_on_solana = sink.latest_height_and_timestamp().await.unwrap().0;
				if latest_height_on_solana.revision_height >= *largest_height {
					log::info!("Latest height is finalized");
					break;
				}
				log::info!("Waiting for next block {:?} to be finalized", latest_height_on_solana);
				core::time::Duration::from_secs(1);
			}
			let (updates, heights) = sink.fetch_mandatory_updates(source).await.unwrap();
			let updates_to_be_sent: Vec<Any> = heights
				.iter()
				.enumerate()
				.filter_map(|(index, event)| {
					let height = match event.clone() {
						ibc::events::IbcEvent::NewBlock(
							ibc::core::ics02_client::events::NewBlock { height },
						) => height,
						_ => panic!("Only expected new block event"),
					};
					if timeout_heights.contains(&height.revision_height) {
						return Some(updates[index].clone());
					}
					None
				})
				.collect();
			// Reverse the updates so that the latest update is sent at end
			let mut reversed_updates = updates_to_be_sent.iter().rev().cloned().collect::<Vec<_>>();
			reversed_updates
				.iter()
				.for_each(|update| timeout_msgs.insert(0, update.clone()));
			// timeout_msgs = (reversed_updates.as_slice(), timeout_msgs.as_slice()).concat();
			// timeout_msgs.append(&mut reversed_updates);
		}
	}

	process_timeouts(source, metrics, timeout_msgs).await?;
	Ok(())
}

async fn process_updates<A: Chain, B: Chain>(
	source: &mut A,
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	mode: Option<Mode>,
	updates: Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>,
	msgs: &mut Vec<Any>,
	timeout_msgs: Vec<Any>,
) -> anyhow::Result<()> {
	// for timeouts we need both chains to be up to date
	let sink_has_undelivered_acks = sink.has_undelivered_sequences(UndeliveredType::Recvs) ||
		sink.has_undelivered_sequences(UndeliveredType::Acks) ||
		sink.has_undelivered_sequences(UndeliveredType::Timeouts);
	let source_has_undelivered_acks = source.has_undelivered_sequences(UndeliveredType::Timeouts);

	let mandatory_heights_for_undelivered_seqs =
		if (sink_has_undelivered_acks || source_has_undelivered_acks) && !updates.is_empty() {
			find_mandatory_heights_for_undelivered_sequences(source, &updates).await
		} else {
			HashSet::new()
		};

	log::info!("Updates on {} are {}", source.name(), updates.len());

	let mut timeout_heights = Vec::new();
	let mut updates_to_be_added = Vec::new();
	// if timeout_msgs.len() > 0 && source.name() == "solana" {
	// 	log::info!("Inside sending updates in fetching height");
	// 	for msg in timeout_msgs.iter() {
	// 		let my_message = ibc::core::ics26_routing::msgs::Ics26Envelope::<
	// 			primitives::mock::LocalClientTypes,
	// 		>::try_from(msg.clone())
	// 		.unwrap();
	// 		let height = match my_message {
	// 			ibc::core::ics26_routing::msgs::Ics26Envelope::Ics4PacketMsg(packet_msg) => {
	// 				match packet_msg {
	// 					ibc::core::ics04_channel::msgs::PacketMsg::ToPacket(msg) => {
	// 						msg.proofs.height()
	// 					},
	// 					ibc::core::ics04_channel::msgs::PacketMsg::AckPacket(msg) => {
	// 						msg.proofs.height()
	// 					},
	// 					ibc::core::ics04_channel::msgs::PacketMsg::RecvPacket(msg) => {
	// 						msg.proofs.height()
	// 					},
	// 					ibc::core::ics04_channel::msgs::PacketMsg::ToClosePacket(msg) => {
	// 						msg.proofs.height()
	// 					},
	// 					_ => continue,
	// 				}
	// 			},
	// 			_ => continue,
	// 		};
	// 		timeout_heights.push(height);
	// 	}
	// 	let latest_update_height =
	// 		updates.last().map_or(0, |(_, height, _, _)| height.revision_height);
	// 	let height_is_greater = timeout_heights
	// 		.iter()
	// 		.any(|height| height.revision_height > latest_update_height);

	// 	if height_is_greater {
	// 		loop {
	// 			let largest_height = timeout_heights.iter().max().unwrap();
	// 			let latest_height_on_solana = source.latest_height_and_timestamp().await?.0;
	// 			log::info!(
	// 				"This is the largest height {:?} {:?}",
	// 				largest_height,
	// 				latest_height_on_solana
	// 			);
	// 			if latest_height_on_solana.revision_height > largest_height.revision_height {
	// 				log::info!("Latest height is finalized and sleeping for 5 seconds");
	// 				std::thread::sleep(core::time::Duration::from_secs(5));
	// 				break;
	// 			}
	// 			log::info!("Waiting for next block {:?} to be finalized", latest_height_on_solana);
	// 			std::thread::sleep(core::time::Duration::from_secs(1));
	// 		}
	// 		let (mandatory_updates, heights) = source.fetch_mandatory_updates(sink).await?;
	// 		// log::info!("Height is greater than timeout height {:?}", );
	// 		log::info!("These are heights {:?}", heights);
	// 		let updates_to_be_sent: Vec<Any> = heights
	// 			.iter()
	// 			.enumerate()
	// 			.filter_map(|(index, event)| {
	// 				let height = match event.clone() {
	// 					ibc::events::IbcEvent::NewBlock(
	// 						ibc::core::ics02_client::events::NewBlock { height },
	// 					) => height,
	// 					_ => panic!("Only expected new block event"),
	// 				};
	// 				let temp_height = Height::new(1, height.revision_height);
	// 				if timeout_heights.contains(&temp_height)
	// 					&& height.revision_height > latest_update_height
	// 				{
	// 					return Some(mandatory_updates[index].clone());
	// 				}
	// 				None
	// 			})
	// 			.collect();
	// 		log::info!("Updates to be sent {:?}", updates_to_be_sent);
	// 		updates_to_be_added = updates_to_be_sent;
	// 		updates_to_be_added.reverse();
	// 	}
	// }
	log::info!(
		"Update heights {:?} and timeout heights {:?}",
		updates.iter().map(|(_, height, ..)| height).collect::<Vec<_>>(),
		timeout_heights
	);

	let update_max_height = updates
		.iter()
		.map(|(_, height, ..)| height.clone())
		.max()
		.unwrap_or(Height::new(1, 0));

	for (msg_update_client, height, events, update_type) in updates {
		if let Some(metrics) = metrics.as_mut() {
			if let Err(e) = metrics.handle_events(events.as_slice()).await {
				log::error!("Failed to handle metrics for {} {:?}", source.name(), e);
			}
		}

		// println!("These are events {:?} from chain {:?}", events, source.name());
		let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
		let mut messages = parse_events(source, sink, events, mode)
			.await
			.map_err(|e| anyhow!("Failed to parse events: {:?}", e))?;

		// if let Some(index) = messages
		// 	.iter()
		// 	.position(|value| value.type_url == "/ibc.core.connection.v1.MsgConnectionOpenTry")
		// {
		// 	log::info!("Remvoign open try");
		// 	messages.swap_remove(index);
		// }

		log::trace!(
			target: "hyperspace",
			"Received messages count: {}, is the update optional: {}",
			messages.len(), update_type.is_optional(),
		);

		let need_to_send_proofs_for_sequences = (sink_has_undelivered_acks ||
			source_has_undelivered_acks) &&
			mandatory_heights_for_undelivered_seqs.contains(&height.revision_height);
		let common_state = source.common_state();
		let skip_optional_updates = common_state.skip_optional_client_updates;

		// println!("These are messages len {}", messages.len());
		// println!("update type: {:?}, skip_optional_updates {:?}", update_type,
		// skip_optional_updates);

		// We want to send client update if packet messages exist but where not sent due
		// to a connection delay even if client update message is optional
		match (
			// TODO: we actually may send only when timeout of some packet has reached,
			// not when we have *any* undelivered packets. But this requires rewriting
			// `find_suitable_proof_height_for_client` function, that uses binary
			// search, which won't work in this case
			skip_optional_updates &&
				update_type.is_optional() &&
				!need_to_send_proofs_for_sequences &&
				!timeout_heights.contains(&height),
			has_packet_events(&event_types),
			messages.is_empty(),
		) {
			(true, false, true) => {
				// skip sending ibc messages if no new events
				log::info!("Skipping finality notification for {}", sink.name());
				continue;
			},
			(false, _, true) =>
				if update_type.is_optional() && need_to_send_proofs_for_sequences {
					log::info!("Sending an optional update because source ({}) chain has undelivered sequences at height{}", sink.name(), height.revision_height);
				} else {
					log::info!(
						"Sending mandatory client update message for {} at height {}",
						sink.name(),
						height.revision_height
					)
				},
			_ => log::info!(
				"Received finalized events from: {} at height {} {event_types:#?}",
				source.name(),
				height.revision_height
			),
		};
		log::info!(
			"pushed msg update client for {} with msg {} of len {}",
			source.name(),
			msg_update_client.type_url,
			msg_update_client.value.len()
		);
		if (height.revision_height != update_max_height.revision_height &&
			messages.is_empty() &&
			update_type.is_optional())
		{
			log::info!(
				"Skipping update for {} at height {} because it is not the latest update",
				source.name(),
				height.revision_height
			);
			continue;
		}
		msgs.push(msg_update_client);
		msgs.append(&mut messages);
	}
	msgs.append(&mut updates_to_be_added);
	Ok(())
}

async fn process_messages<B: Chain>(
	sink: &mut B,
	metrics: &mut Option<MetricsHandler>,
	msgs: Vec<Any>,
) -> anyhow::Result<()> {
	if !msgs.is_empty() {
		if let Some(metrics) = metrics.as_ref() {
			metrics.handle_messages(msgs.as_slice()).await;
		}
		let type_urls = msgs
			.iter()
			.filter_map(|msg| {
				let type_url = msg.type_url.as_str();
				if type_url == "" {
					return None;
				};
				Some(type_url)
			})
			.collect::<Vec<_>>();
		log::info!("Submitting messages to {}: {type_urls:#?}", sink.name());

		let filtered_msgs: Vec<_> = msgs
			.iter()
			.filter_map(|msg| {
				if msg.type_url == "" {
					return None;
				}
				Some(msg.clone())
			})
			.collect();

		if !filtered_msgs.is_empty() {
			queue::flush_message_batch(filtered_msgs, metrics.as_ref(), &*sink)
				.await
				.map_err(|e| anyhow!("Failed to submit messages: {:?}", e))?;
		}
		log::debug!(target: "hyperspace", "Successfully submitted messages to {}", sink.name());
	}
	Ok(())
}

async fn process_timeouts<A: Chain>(
	source: &mut A,
	metrics: &mut Option<MetricsHandler>,
	timeout_msgs: Vec<Any>,
) -> anyhow::Result<()> {
	if !timeout_msgs.is_empty() {
		if let Some(metrics) = metrics.as_ref() {
			metrics.handle_timeouts(timeout_msgs.as_slice()).await;
		}
		let type_urls = timeout_msgs.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
		log::info!("Submitting timeout messages to {}: {type_urls:#?}", source.name());
		queue::flush_message_batch(timeout_msgs, metrics.as_ref(), &*source)
			.await
			.map_err(|e| anyhow!("Failed to submit timeout messages: {:?}", e))?;
		log::debug!(target: "hyperspace", "Successfully submitted timeout messages to {}", source.name());
	}
	Ok(())
}

async fn find_mandatory_heights_for_undelivered_sequences<A: Chain>(
	source: &mut A,
	updates: &[(Any, Height, Vec<IbcEvent>, UpdateType)],
) -> HashSet<u64> {
	let mut mandatory_updates_for_undelivered_seqs = HashSet::new();
	let update_heights = updates
		.iter()
		.map(|(_, height, ..)| height.revision_height)
		.collect::<HashSet<_>>();
	let (_, height, ..) = updates.first().unwrap();
	let proof_height = source.get_proof_height(*height).await;
	let block_proof_height_difference = proof_height
		.revision_height
		.checked_sub(height.revision_height)
		.expect("proof height is less than update height");
	let needed_updates_num = if block_proof_height_difference > 0 { 2 } else { 1 };
	for (_, height, ..) in updates.iter().rev() {
		if let Some(prev_height) = height.revision_height.checked_sub(block_proof_height_difference)
		{
			if update_heights.contains(&prev_height) {
				mandatory_updates_for_undelivered_seqs.insert(height.revision_height);
				mandatory_updates_for_undelivered_seqs.insert(prev_height);
			}
		}
		if mandatory_updates_for_undelivered_seqs.len() == needed_updates_num {
			break;
		}
	}
	mandatory_updates_for_undelivered_seqs
}

#[cfg(feature = "testing")]
pub mod send_packet_relay {
	use std::sync::atomic::{AtomicBool, Ordering};
	static RELAY_PACKETS: AtomicBool = AtomicBool::new(true);

	/// Returns status of send packet relay
	pub fn packet_relay_status() -> bool {
		RELAY_PACKETS.load(Ordering::SeqCst)
	}

	/// Sets packet relay status
	pub fn set_relay_status(status: bool) {
		RELAY_PACKETS.store(status, Ordering::SeqCst);
	}
}
