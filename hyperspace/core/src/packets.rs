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

#[cfg(feature = "testing")]
use crate::send_packet_relay::packet_relay_status;
use rand::Rng;
use sp_runtime::Either::{Left, Right};
use std::{
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
	time::Duration,
};
use tokio::{task::JoinSet, time::sleep};

use crate::packets::utils::{
	construct_ack_message, construct_recv_message, construct_timeout_message,
	get_timeout_proof_height, verify_delay_passed, VerifyDelayOn,
};
use ibc::{
	core::{
		ics02_client::client_state::ClientState as ClientStateT,
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::channel::{ChannelEnd, State},
	},
	Height,
};
use ibc_proto::google::protobuf::Any;
use pallet_ibc::light_clients::AnyClientState;
use primitives::{
	error::Error, find_suitable_proof_height_for_client, packet_info_to_packet,
	query_undelivered_acks, query_undelivered_sequences, Chain, UndeliveredType,
};

pub mod connection_delay;
pub mod utils;

pub const PROCESS_PACKETS_BATCH_SIZE: usize = 10;

/// Returns a tuple of messages, with the first item being packets that are ready to be sent to the
/// sink chain. And the second item being packet timeouts that should be sent to the source.
///
/// The function also flags the packets/timeouts that are ready to be sent. The idea is the
/// following, basically:
/// source -> recv_packet    -> sink   => sink has undelivered recvs
/// source -> ack_packet     -> sink   => sink has undelivered acks
/// source -> timeout_packet -> source => source & sink has undelivered timeouts (since timeouts
/// need both clients to be up to date)
pub async fn query_ready_and_timed_out_packets(
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<(Vec<Any>, Vec<Any>), anyhow::Error> {
	let mut messages = vec![];
	let mut timeout_messages = vec![];
	let (source_height, source_timestamp) = source.latest_height_and_timestamp().await?;
	let (sink_height, sink_timestamp) = sink.latest_height_and_timestamp().await?;
	let channel_whitelist = source.channel_whitelist();

	// TODO: parallelize this
	for (channel_id, port_id) in channel_whitelist {
		let source_channel_response = match source
			.query_channel_end(source_height, channel_id, port_id.clone())
			.await
		{
			Ok(response) => response,
			// this can happen in case the channel is not yet created
			Err(e) => {
				log::warn!(target: "hyperspace", "Failed to query channel end for chain {}, channel {}/{}: {:?}", source.name(), channel_id, port_id, e);
				continue
			},
		};
		let source_channel_end = match source_channel_response.channel.map(ChannelEnd::try_from) {
			Some(Ok(source_channel)) => source_channel,
			_ => {
				log::warn!(target: "hyperspace", "ChannelEnd not found for {:?}/{:?}", channel_id, port_id.clone());
				continue
			},
		};
		// we're only interested in open or closed channels
		if !matches!(source_channel_end.state, State::Open | State::Closed) {
			log::trace!(target: "hyperspace", "Skipping channel {:?}/{:?} because it is not open or closed", channel_id, port_id.clone());
			continue
		}
		let connection_id = source_channel_end
			.connection_hops
			.get(0)
			.ok_or_else(|| Error::Custom("Channel end missing connection id".to_string()))?
			.clone();
		let connection_response =
			source.query_connection_end(source_height, connection_id.clone()).await?;
		let source_connection_end =
			ConnectionEnd::try_from(connection_response.connection.ok_or_else(|| {
				Error::Custom(format!(
					"[query_ready_and_timed_out_packets] ConnectionEnd not found for {connection_id:?}"
				))
			})?)?;

		let sink_channel_id = source_channel_end.counterparty().channel_id.ok_or_else(|| {
			Error::Custom(
				" An Open Channel End should have a valid counterparty channel id".to_string(),
			)
		})?;
		let sink_port_id = source_channel_end.counterparty().port_id.clone();
		let sink_channel_response = match sink
			.query_channel_end(sink_height, sink_channel_id, sink_port_id.clone())
			.await
		{
			Ok(response) => response,
			Err(e) => {
				// this can happen in case the channel is not yet created
				log::warn!(target: "hyperspace", "Failed to query channel end for chain {}, channel {}/{}: {:?}", sink.name(), channel_id, port_id, e);
				continue
			},
		};

		let sink_channel_end = match sink_channel_response.channel.map(ChannelEnd::try_from) {
			Some(Ok(sink_channel)) => sink_channel,
			_ => {
				log::warn!(target: "hyperspace", "ChannelEnd not found for {:?}/{:?}", channel_id, port_id.clone());
				continue
			},
		};

		let next_sequence_recv = sink
			.query_next_sequence_recv(sink_height, &sink_port_id, &sink_channel_id)
			.await?;

		let source_client_state_on_sink =
			sink.query_client_state(sink_height, source.client_id()).await?;
		let source_client_state_on_sink = AnyClientState::try_from(
			source_client_state_on_sink.client_state.ok_or_else(|| {
				Error::Custom(format!(
					"Client state for {} should exist on {}",
					source.name(),
					sink.name()
				))
			})?,
		)
		.map_err(|_| {
			Error::Custom(format!(
				"Invalid Client state for {} should found on {}",
				source.name(),
				sink.name()
			))
		})?;

		let sink_client_state_on_source =
			source.query_client_state(source_height, sink.client_id()).await?;
		let sink_client_state_on_source = AnyClientState::try_from(
			sink_client_state_on_source.client_state.ok_or_else(|| {
				Error::Custom(format!(
					"Client state for {} should exist on {}",
					source.name(),
					sink.name()
				))
			})?,
		)
		.map_err(|_| {
			Error::Custom(format!(
				"Invalid Client state for {} should found on {}",
				source.name(),
				sink.name()
			))
		})?;
		let latest_sink_height_on_source = sink_client_state_on_source.latest_height();
		let latest_source_height_on_sink = source_client_state_on_sink.latest_height();

		let max_packets_to_process = source.common_state().max_packets_to_process;

		// query packets that are waiting for connection delay.
		let seqs = query_undelivered_sequences(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			source,
			sink,
		)
		.await?
		.into_iter()
		.take(max_packets_to_process)
		.collect::<Vec<_>>();

		log::info!(target: "hyperspace", "Found {} undelivered packets for {:?}/{:?} for {seqs:?}", seqs.len(), channel_id, port_id.clone());

		let mut send_packets = source.query_send_packets(channel_id, port_id.clone(), seqs).await?;
		log::trace!(target: "hyperspace", "SendPackets count before deduplication: {}", send_packets.len());
		send_packets.sort();
		send_packets.dedup();
		log::trace!(target: "hyperspace", "SendPackets count after deduplication: {}", send_packets.len());
		let mut recv_packets_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		let source = Arc::new(source.clone());
		let sink = Arc::new(sink.clone());
		let timeout_packets_count = Arc::new(AtomicUsize::new(0));
		let send_packets_count = Arc::new(AtomicUsize::new(0));
		let acks_packets_count = Arc::new(AtomicUsize::new(0));
		for send_packets in send_packets.chunks(PROCESS_PACKETS_BATCH_SIZE) {
			for send_packet in send_packets.iter().cloned() {
				let source_connection_end = source_connection_end.clone();
				let sink_channel_end = sink_channel_end.clone();
				let source_connection_end = source_connection_end.clone();
				let source = source.clone();
				let sink = sink.clone();
				let duration = Duration::from_millis(
					rand::thread_rng().gen_range(1..source.rpc_call_delay().as_millis() as u64),
				);
				let timeout_packets_count = timeout_packets_count.clone();
				let recv_packets_count = send_packets_count.clone();
				recv_packets_join_set.spawn(async move {
					sleep(duration).await;
					let source = &source;
					let sink = &sink;
					let packet = packet_info_to_packet(&send_packet);
					// Check if packet has timed out
					let packet_height = send_packet.height.ok_or_else(|| {
						Error::Custom(format!("Packet height not found for packet {packet:?}"))
					})?;

				    log::info!("sink_height = {sink_height:?}, timeout_height = {:?}, timeout_timestamp = {}, seq = {}", packet.timeout_height, packet.timeout_timestamp, packet.sequence);

					if packet.timed_out(&sink_timestamp, sink_height) {
						source.common_state().ignored_timeouted_sequences.lock().await.insert(
							packet.sequence.0
						);
						// so we know this packet has timed out on the sink, we need to find the maximum
						// consensus state height at which we can generate a non-membership proof of the
						// packet for the sink's client on the source.
						let proof_height =
							if let Some(proof_height) = get_timeout_proof_height(
								&**source,
								&**sink,
								source_height,
								source_timestamp,
								sink_height,
								sink_timestamp,
								latest_sink_height_on_source,
								&packet,
								packet_height,
								source_connection_end.delay_period(),
							)
							.await
						{
							proof_height
						} else {
							timeout_packets_count.fetch_add(1, Ordering::SeqCst);
							log::trace!(target: "hyperspace", "Skipping packet as no timeout proof height could be found: {:?}", packet);
							return Ok(None)
						};

						// given this maximum height, has the connection delay been satisfied?
						if !verify_delay_passed(
							&**source,
							&**sink,
							source_timestamp,
							source_height,
							sink_timestamp,
							sink_height,
							source_connection_end.delay_period(),
							proof_height,
							VerifyDelayOn::Source,
						)
							.await?
						{
							log::trace!(target: "hyperspace", "Skipping packet as connection delay has not passed {:?}", packet);
							return Ok(None)
						}

						// lets construct the timeout message to be sent to the source
						let msg = construct_timeout_message(
							&**source,
							&**sink,
							&sink_channel_end,
							packet,
							next_sequence_recv.next_sequence_receive,
							proof_height,
						)
							.await?;
						return Ok(Some(Left(msg)))
					} else {
						log::info!(target: "hyperspace", "The packet has not timed out yet: {:?}", packet);
					}

					// If packet has not timed out but channel is closed on sink we skip
					// Since we have no reference point for when this channel was closed so we can't
					// calculate connection delays yet
					if sink_channel_end.state == State::Closed {
						log::debug!(target: "hyperspace", "Skipping packet as channel is closed on sink: {:?}", packet);
						return Ok(None)
					}

					#[cfg(feature = "testing")]
					// If packet relay status is paused skip
					if !packet_relay_status() {
						return Ok(None)
					}

					// Check if packet is ready to be sent to sink
					// If sink does not have a client height that is equal to or greater than the packet
					// creation height, we can't send it yet, packet_info.height should represent the packet
					// creation height on source chain
					if source.get_proof_height(Height::new(source_height.revision_number, packet_height)).await.revision_height > latest_source_height_on_sink.revision_height {
						// Sink does not have client update required to prove recv packet message
						log::info!(target: "hyperspace", "Skipping packet as sink does not have client update required to prove recv packet message: {:?}", packet);
						recv_packets_count.fetch_add(1, Ordering::SeqCst);
						return Ok(None)
					}

					let start_height = Height::new(latest_source_height_on_sink.revision_number, packet_height);
					let proof_height = if let Some(proof_height) = find_suitable_proof_height_for_client(
						&**source,
						&**sink,
						sink_height,
						source.client_id(),
						start_height,
						Some(start_height),
						None,
						latest_source_height_on_sink,
						source_connection_end.delay_period(),
					)
						.await
					{
						proof_height
					} else {
						log::info!(target: "hyperspace", "Skipping packet as no proof height could be found: {:?}", packet);
						return Ok(None)
					};

					if !verify_delay_passed(
						&**source,
						&**sink,
						source_timestamp,
						source_height,
						sink_timestamp,
						sink_height,
						source_connection_end.delay_period(),
						proof_height,
						VerifyDelayOn::Sink,
					)
						.await?
					{
						log::info!(target: "hyperspace", "Skipping packet as connection delay has not passed {:?}", packet);
						return Ok(None)
					}

					if packet.timeout_height.is_zero() && packet.timeout_timestamp.nanoseconds() == 0 {
						log::warn!(target: "hyperspace", "Skipping packet as packet timeout is zero: {}", packet.sequence);
						return Ok(None)
					}

					let msg = construct_recv_message(&**source, &**sink, packet, proof_height).await?;
					Ok(Some(Right(msg)))
				});
			}
		}

		while let Some(result) = recv_packets_join_set.join_next().await {
			let Some(either) = result?? else { continue };
			match either {
				Left(msg) => timeout_messages.push(msg),
				Right(msg) => messages.push(msg),
			}
		}

		let timeouts_count = timeout_packets_count.load(Ordering::SeqCst);
		log::debug!(target: "hyperspace", "Found {timeouts_count} packets that have timed out");
		source
			.on_undelivered_sequences(timeouts_count != 0, UndeliveredType::Timeouts)
			.await;

		let sends_count = send_packets_count.load(Ordering::SeqCst);
		log::debug!(target: "hyperspace", "Found {sends_count} sent packets");
		sink.on_undelivered_sequences(sends_count != 0, UndeliveredType::Recvs).await;

		// Get acknowledgement messages
		if source_channel_end.state == State::Closed {
			log::trace!(target: "hyperspace", "Skipping acknowledgements for channel {:?} as channel is closed on source", channel_id);
			continue
		}

		// query acknowledgements that are waiting for connection delay.
		let acks = query_undelivered_acks(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			&*source,
			&*sink,
		)
		.await?
		.into_iter()
		.take(max_packets_to_process)
		.collect::<Vec<_>>();

		let acknowledgements =
			source.query_received_packets(channel_id, port_id.clone(), acks).await?;
		log::trace!(target: "hyperspace", "Got acknowledgements for channel {:?}: {:?}", channel_id, acknowledgements);
		let mut acknowledgements_join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();

		for acknowledgements in acknowledgements.chunks(PROCESS_PACKETS_BATCH_SIZE) {
			for acknowledgement in acknowledgements.iter().cloned() {
				let source_connection_end = source_connection_end.clone();
				let source = source.clone();
				let sink = sink.clone();
				let duration1 = Duration::from_millis(
					rand::thread_rng().gen_range(1..source.rpc_call_delay().as_millis() as u64),
				);
				let acks_packets_count = acks_packets_count.clone();
				acknowledgements_join_set.spawn(async move {
					sleep(duration1).await;
					let source = &source;
					let sink = &sink;
					let packet = packet_info_to_packet(&acknowledgement);
					let ack = if let Some(ack) = acknowledgement.ack {
						ack
					} else {
						// Packet has no valid acknowledgement, skip
						log::trace!(target: "hyperspace", "Skipping acknowledgement for packet {:?} as packet has no valid acknowledgement", packet);
						return Ok(None)
					};

					// Check if ack is ready to be sent to sink
					// If sink does not have a client height that is equal to or greater than the packet
					// creation height, we can't send it yet packet_info.height should represent the
					// acknowledgement creation height on source chain
					let ack_height = acknowledgement.height.ok_or_else(|| {
						Error::Custom(format!("Packet height not found for packet {packet:?}"))
					})?;

					if source.get_proof_height(Height::new(source_height.revision_number, ack_height)).await.revision_height > latest_source_height_on_sink.revision_height {
						// Sink does not have client update required to prove acknowledgement packet message
						log::info!(target: "hyperspace", "Skipping acknowledgement for packet {:?} as sink does not have client update required to prove acknowledgement packet message", packet);
						acks_packets_count.fetch_add(1, Ordering::SeqCst);
						return Ok(None)
					}

					log::info!(target: "hyperspace", "sink_height: {:?}, latest_source_height_on_sink: {:?}, acknowledgement.height: {}", sink_height, latest_source_height_on_sink, ack_height);

					let start_height = Height::new(latest_source_height_on_sink.revision_number, ack_height);
					let proof_height = if let Some(proof_height) = find_suitable_proof_height_for_client(
						&**source,
						&**sink,
						sink_height,
						source.client_id(),
						start_height,
						Some(start_height),
						None,
						latest_source_height_on_sink,
						source_connection_end.delay_period()
					)
						.await
					{
						log::info!(target: "hyperspace", "Using proof height: {}", proof_height);
						proof_height
					} else {
						log::info!(target: "hyperspace", "Skipping acknowledgement for packet {:?} as no proof height could be found", packet); 	return Ok(None)
					};

					if !verify_delay_passed(
						&**source,
						&**sink,
						source_timestamp,
						source_height,
						sink_timestamp,
						sink_height,
						source_connection_end.delay_period(),
						proof_height,
						VerifyDelayOn::Sink,
					)
						.await?
					{
						log::info!(target: "hyperspace", "Skipping acknowledgement for packet as connection delay has not passed {:?}", packet);
						return Ok(None)
					}

					let msg = construct_ack_message(&**source, &**sink, packet, ack, proof_height).await?;
					Ok(Some(msg))
				});
			}
		}

		while let Some(result) = acknowledgements_join_set.join_next().await {
			let Some(msg) = result?? else { continue };
			messages.push(msg)
		}

		sink.on_undelivered_sequences(
			acks_packets_count.load(Ordering::SeqCst) != 0,
			UndeliveredType::Acks,
		)
		.await;
	}

	Ok((messages, timeout_messages))
}
