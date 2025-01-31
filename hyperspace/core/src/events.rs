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
use crate::Mode;
use codec::Encode;
use core::time::Duration;
use ibc::{
	core::{
		ics02_client::client_state::ClientState as ClientStateT,
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty},
			msgs::{
				conn_open_ack::MsgConnectionOpenAck, conn_open_confirm::MsgConnectionOpenConfirm,
				conn_open_try::MsgConnectionOpenTry,
			},
		},
		ics04_channel::{
			channel::{ChannelEnd, Counterparty as ChannelCounterparty, State},
			msgs::{
				acknowledgement::MsgAcknowledgement, chan_close_confirm::MsgChannelCloseConfirm,
				chan_open_ack::MsgChannelOpenAck, chan_open_confirm::MsgChannelOpenConfirm,
				chan_open_try::MsgChannelOpenTry, recv_packet::MsgRecvPacket,
			},
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
		ics24_host::identifier::ConnectionId,
	},
	events::{IbcEvent, IbcEventType},
	proofs::{ConsensusProof, Proofs},
	tx_msg::Msg,
	Height,
};
use ibc_proto::google::protobuf::Any;
use pallet_ibc::light_clients::AnyClientState;
use primitives::{error::Error, mock::LocalClientTypes, Chain};
use std::str::FromStr;
use tendermint_proto::Protobuf;

/// Connection proof type
#[derive(Encode)]
pub struct ConnectionProof {
	pub host_proof: Vec<u8>,
	pub connection_proof: Vec<u8>,
}

/// This parses events coming from a source chain
/// Returns a tuple of messages, with the first item being packets that are ready to be sent to the
/// sink chain. And the second item being packet timeouts that should be sent to the source.
pub async fn parse_events(
	source: &mut impl Chain,
	sink: &mut impl Chain,
	events: Vec<IbcEvent>,
	mode: Option<Mode>,
) -> Result<Vec<Any>, anyhow::Error> {
	let mut messages = vec![];
	// 1. translate events to messages
	let mut is_connection_delay = true;
	for event in events {
		// if (matches!(event, IbcEvent::SendPacket(_))
		// 	|| matches!(event, IbcEvent::WriteAcknowledgement(_)))
		// 	&& is_connection_delay
		// {
		// 	log::info!("event {:?} {:?}", is_connection_delay, source.name());
		// 	log::info!("Skipping due to connection delay {:?}", event);
		// 	continue;
		// }
		match event {
			IbcEvent::OpenInitConnection(open_init) => {
				if let Some(connection_id) = open_init.connection_id() {
					log::info!("I am in open init connection with open init {:?}", open_init);
					let connection_id = connection_id.clone();
					// Get connection end with proof
					let connection_response = source
						.query_connection_end(open_init.height(), connection_id.clone())
						.await?;
					let connection_end = ConnectionEnd::try_from(
						connection_response.connection.ok_or_else(|| {
							Error::Custom(format!(
								"[get_messages_for_events - open_conn_init] Connection end not found for {:?}",
								open_init.attributes().connection_id
							))
						})?,
					)?;
					let counterparty = connection_end.counterparty();

					let connection_proof =
						CommitmentProofBytes::try_from(connection_response.proof)?;
					let prefix: CommitmentPrefix = source.connection_prefix();
					let client_state_response = source
						.query_client_state(
							open_init.height(),
							open_init.attributes().client_id.clone(),
						)
						.await?;

					let proof_height = connection_response.proof_height.ok_or_else(|| Error::Custom("[get_messages_for_events - open_conn_init] Proof height not found in response".to_string()))?;
					let proof_height = Height::new(1, proof_height.revision_height);
					// client_state.latest_height();
					let client_state_proof =
						CommitmentProofBytes::try_from(client_state_response.proof).ok();
					let client_state = client_state_response
						.client_state
						.clone()
						.map(AnyClientState::try_from)
						.ok_or_else(|| Error::Custom("Client state is empty".to_string()))??;

					log::info!(
						"This is client state {:?} and in any {:?}",
						client_state,
						client_state_response.client_state
					);
					let consensus_proof = source
						.query_client_consensus(
							open_init.height(),
							open_init.attributes().client_id.clone(),
							client_state.latest_height(),
							true,
						)
						.await?;
					let host_consensus_state_proof =
						query_host_consensus_state_proof(sink, client_state.clone()).await?;

					// Construct OpenTry
					log::info!(
						"This was the delay period {:?} in nanos: {:?}",
						connection_end.delay_period(),
						connection_end.delay_period().as_nanos()
					);
					let msg = MsgConnectionOpenTry::<LocalClientTypes> {
						client_id: counterparty.client_id().clone(),
						// client state proof is mandatory in conn_open_try
						client_state: Some(client_state.clone()),
						counterparty: Counterparty::new(
							open_init.attributes().client_id.clone(),
							Some(connection_id),
							prefix,
						),
						counterparty_versions: connection_end.versions().to_vec(),
						proofs: Proofs::new(
							connection_proof,
							client_state_proof,
							Some(ConsensusProof::new(
								CommitmentProofBytes::try_from(consensus_proof.proof)?,
								client_state.latest_height(),
							)?),
							None,
							proof_height,
						)?,
						delay_period: connection_end.delay_period(),
						signer: sink.account_id(),
						host_consensus_state_proof,
					};
					log::info!(
						"Constructed open try {:?} with client height {:?}",
						proof_height,
						client_state.latest_height()
					);
					let value = msg.encode_vec()?;
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				}
			},
			IbcEvent::OpenTryConnection(open_try) => {
				if let Some(connection_id) = open_try.connection_id() {
					log::info!(
						"THis is open try {:?} and connection id {:?}",
						open_try,
						connection_id
					);
					let connection_id = connection_id.clone();
					// Get connection end with proof
					let connection_response = source
						.query_connection_end(open_try.height(), connection_id.clone())
						.await?;
					log::info!("This is connection open ack from cosmos {:?}", connection_response);
					let connection_end = ConnectionEnd::try_from(
						connection_response.connection.ok_or_else(|| {
							Error::Custom(format!(
								"[get_messages_for_events - open_conn_try] Connection end not found for {:?}",
								open_try.attributes().connection_id
							))
						})?,
					)?;
					log::info!("This is the connection end in open ack {:?}", connection_end);
					log::info!(
						"Encoded connection end in open ack {:?}",
						connection_end.encode_vec()?
					);
					let counterparty = connection_end.counterparty();

					let connection_proof =
						CommitmentProofBytes::try_from(connection_response.proof)?;
					let client_state_response = source
						.query_client_state(
							open_try.height(),
							open_try.attributes().client_id.clone(),
						)
						.await?;

					let proof_height = connection_response.proof_height.ok_or_else(|| Error::Custom("[get_messages_for_events - open_conn_try] Proof height not found in response".to_string()))?;
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);
					let client_state_proof =
						CommitmentProofBytes::try_from(client_state_response.clone().proof).ok();
					log::info!(
						"This is from cosmos {:?}",
						client_state_response.clone().client_state
					);
					let client_state: AnyClientState = client_state_response
						.client_state
						.map(AnyClientState::try_from)
						.ok_or_else(|| Error::Custom("Client state is empty".to_string()))??;
					log::info!("THis is the client state of solana on cosmos {:?}", client_state);
					log::info!(
						"This is encoded client state in open ack {:?}",
						client_state.encode_vec()
					);
					let consensus_proof = source
						.query_client_consensus(
							open_try.height(),
							open_try.attributes().client_id.clone(),
							client_state.latest_height(),
							true,
						)
						.await?;
					let host_consensus_state_proof =
						query_host_consensus_state_proof(sink, client_state.clone()).await?;
					// Construct OpenAck
					println!("-----------------");
					println!("OpenAck consensus height {:?}", client_state.latest_height());
					println!("-----------------");
					let msg = MsgConnectionOpenAck::<LocalClientTypes> {
						connection_id: counterparty
							.connection_id()
							.ok_or_else(|| {
								Error::Custom("[get_messages_for_events - open_conn_try] Connection Id not found".to_string())
							})?
							.clone(),
						counterparty_connection_id: connection_id,
						client_state: Some(client_state.clone()),
						proofs: Proofs::new(
							connection_proof,
							client_state_proof,
							Some(ConsensusProof::new(
								CommitmentProofBytes::try_from(consensus_proof.proof)?,
								client_state.latest_height(),
							)?),
							None,
							proof_height,
						)?,
						host_consensus_state_proof,
						version: connection_end
							.versions()
							.get(0)
							.ok_or_else(|| {
								Error::Custom(format!(
									"[get_messages_for_events - open_conn_try] Connection version is missing for  {:?}",
									open_try.attributes().connection_id
								))
							})?
							.clone(),
						signer: sink.account_id(),
					};

					let value = msg.encode_vec()?;
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				}
			},
			IbcEvent::OpenAckConnection(open_ack) => {
				if let Some(connection_id) = open_ack.connection_id() {
					let connection_id = connection_id.clone();
					// Get connection end with proof
					let connection_response = source
						.query_connection_end(open_ack.height(), connection_id.clone())
						.await?;
					let connection_end = ConnectionEnd::try_from(
						connection_response.connection.ok_or_else(|| {
							Error::Custom(format!(
								"[get_messages_for_events - open_conn_ack] Connection end not found for {:?}",
								open_ack.attributes().connection_id
							))
						})?,
					)?;
					let counterparty = connection_end.counterparty();

					let connection_proof =
						CommitmentProofBytes::try_from(connection_response.proof)?;

					let proof_height = connection_response.proof_height.ok_or_else(|| {
						Error::Custom("[get_messages_for_events - open_conn_ack] Proof height not found in response".to_string())
					})?;
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);

					println!("-----------------");
					println!(
						"Proof Height for OpenConfirm {:?} {:?}",
						proof_height,
						open_ack.height()
					);
					println!("-----------------");

					// Construct OpenConfirm
					let msg = MsgConnectionOpenConfirm {
						connection_id: counterparty
							.connection_id()
							.ok_or_else(|| {
								Error::Custom("[get_messages_for_events - open_conn_ack] Connection Id not found".to_string())
							})?
							.clone(),
						proofs: Proofs::new(connection_proof, None, None, None, proof_height)?,
						signer: sink.account_id(),
					};

					let value = msg.encode_vec()?;
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				}
			},
			IbcEvent::OpenInitChannel(open_init) => {
				if let Some(channel_id) = open_init.channel_id {
					let channel_response = source
						.query_channel_end(
							open_init.height(),
							channel_id,
							open_init.port_id.clone(),
						)
						.await?;
					let channel_end =
						ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
							Error::Custom(format!(
								"[get_messages_for_events - open_chan_init] ChannelEnd not found for {:?}/{:?}",
								channel_id,
								open_init.port_id.clone()
							))
						})?)
						.expect("Channel end decoding should not fail");
					let counterparty = channel_end.counterparty();

					let connection_response = source
						.query_connection_end(open_init.height(), open_init.connection_id.clone())
						.await?;
					let connection_end = connection_response.connection.ok_or_else(|| {
						Error::Custom(format!(
							"[get_messages_for_events - open_chan_init] Connection end not found for {:?}",
							open_init.connection_id.clone()
						))
					})?;
					let counterparty_connection = connection_end.counterparty.ok_or_else(|| {
						Error::Custom(format!(
							"[get_messages_for_events - open_chan_init] Connection counterparty not found for {:?}",
							open_init.connection_id.clone()
						))
					})?;

					// Construct the channel end as we expect it to be constructed on the
					// receiving chain
					let channel = ChannelEnd::new(
						State::TryOpen,
						channel_end.ordering,
						ChannelCounterparty::new(open_init.port_id, Some(channel_id)),
						vec![ConnectionId::from_str(&counterparty_connection.connection_id)?],
						channel_end.version.clone(),
					);

					let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

					let proof_height = channel_response.proof_height.expect(
						"[get_messages_for_events - open_chan_init]Proof height should be present",
					);
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);

					let msg = MsgChannelOpenTry {
						port_id: counterparty.port_id.clone(),
						channel,
						counterparty_version: channel_end.version,
						proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
						signer: sink.account_id(),
					};

					let value = msg.encode_vec()?;
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				}
			},
			IbcEvent::OpenTryChannel(open_try) =>
				if let Some(channel_id) = open_try.channel_id {
					let channel_response = source
						.query_channel_end(open_try.height(), channel_id, open_try.port_id.clone())
						.await?;
					let channel_end =
						ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
							Error::Custom(format!(
								"[get_messages_for_events - open_chan_try] ChannelEnd not found for {:?}/{:?}",
								channel_id, open_try.port_id
							))
						})?)
						.expect("Channel end decoding should not fail");
					let counterparty = channel_end.counterparty();
					let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

					let proof_height = channel_response.proof_height.expect(
						"[get_messages_for_events - open_chan_try] Proof height should be present",
					);
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);

					let msg = MsgChannelOpenAck {
						port_id: counterparty.port_id.clone(),
						counterparty_version: channel_end.version.clone(),
						proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
						channel_id: counterparty.channel_id.expect("Expect channel id to be set"),
						counterparty_channel_id: channel_id,

						signer: sink.account_id(),
					};

					let value = msg.encode_vec()?;
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				},
			IbcEvent::OpenAckChannel(open_ack) =>
				if let Some(channel_id) = open_ack.channel_id {
					let channel_response = source
						.query_channel_end(open_ack.height(), channel_id, open_ack.port_id.clone())
						.await?;
					let channel_end =
						ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
							Error::Custom(format!(
								"[get_messages_for_events - open_chan_ack] ChannelEnd not found for {:?}/{:?}",
								channel_id, open_ack.port_id
							))
						})?)?;
					let counterparty = channel_end.counterparty();
					let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

					let proof_height =
						channel_response.proof_height.expect("Proof height should be present");
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);

					let msg = MsgChannelOpenConfirm {
						port_id: counterparty.port_id.clone(),
						proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
						channel_id: counterparty.channel_id.expect("Expect channel id to be set"),

						signer: sink.account_id(),
					};

					let value = msg.encode_vec()?;
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				},
			IbcEvent::CloseInitChannel(close_init) => {
				let channel_id = close_init.channel_id;
				let channel_response = source
					.query_channel_end(close_init.height(), channel_id, close_init.port_id.clone())
					.await?;
				let channel_end =
					ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
						Error::Custom(format!(
							"[get_messages_for_events - close_chan_init] ChannelEnd not found for {:?}/{:?}",
							channel_id, close_init.port_id
						))
					})?)?;
				let counterparty = channel_end.counterparty();
				let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

				let proof_height =
					channel_response.proof_height.expect("Proof height should be present");
				let proof_height =
					Height::new(proof_height.revision_number, proof_height.revision_height);

				let msg = MsgChannelCloseConfirm {
					port_id: counterparty.port_id.clone(),
					proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
					channel_id: counterparty.channel_id.expect("Expect channel id to be set"),

					signer: sink.account_id(),
				};

				let value = msg.encode_vec()?;
				let msg = Any { value, type_url: msg.type_url() };
				messages.push(msg)
			},
			IbcEvent::SendPacket(send_packet) => {
				#[cfg(feature = "testing")]
				if !packet_relay_status() {
					log::info!("Skipping packet relay status");
					continue;
				}
				if is_connection_delay && source.name() != "solana" {
					log::info!("Skipping send packet relay because of connection delay");
					continue;
				}
				log::info!("Found send packet {:?}", send_packet);
				// can we send this packet?
				// 1. query the connection and get the connection delay.
				// 2. if none, send message immediately
				// 3. otherwise skip.
				let port_id = send_packet.packet.source_port.clone();
				let channel_id = send_packet.packet.source_channel;
				// let channel_response = source
				// 	.query_channel_end(send_packet.height, channel_id, port_id.clone())
				// 	.await?;
				// let channel_end =
				// 	ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
				// 		Error::Custom(
				// 			"Failed to convert to concrete channel end from raw channel end"
				// 				.to_string(),
				// 		)
				// 	})?)?;
				// let connection_id = channel_end
				// 	.connection_hops
				// 	.get(0)
				// 	.ok_or_else(|| Error::Custom("Channel end missing connection id".to_string()))?
				// 	.clone();
				// let connection_response =
				// 	source.query_connection_end(send_packet.height, connection_id.clone()).await?;
				// let connection_end =
				// 	ConnectionEnd::try_from(connection_response.connection.ok_or_else(|| {
				// 		Error::Custom(format!("ConnectionEnd not found for {connection_id:?}"))
				// 	})?)?;
				// if !connection_end.delay_period().is_zero() {
				// 	// We can't send this packet immediately because of connection delays
				// 	log::info!(
				// 		target: "hyperspace",
				// 		"Skipping packet relay because of connection delays {:?}",
				// 		connection_end.delay_period()
				// 	);
				// 	is_connection_delay = true;
				// 	continue
				// }
				let seq = u64::from(send_packet.packet.sequence);
				let packet = send_packet.packet;

				if packet.timeout_height.is_zero() && packet.timeout_timestamp.nanoseconds() == 0 {
					log::info!(
						target: "hyperspace",
						"Skipping packet relay because packet timeout is zero: {}",
						packet.sequence
					);
					continue;
				}

				let (sink_height, sink_timestamp) = sink.latest_height_and_timestamp().await?;

				if packet.timed_out(&sink_timestamp, sink_height) {
					log::info!("Packet timed out, so skipping it inside parse events");
					continue;
				}

				let packet_commitment_response = source
					.query_packet_commitment(send_packet.height, &port_id, &channel_id, seq)
					.await?;
				let commitment_proof =
					CommitmentProofBytes::try_from(packet_commitment_response.proof)?;

				let proof_height = packet_commitment_response
					.proof_height
					.expect("Proof height should be present");
				let proof_height =
					Height::new(proof_height.revision_number, proof_height.revision_height);
				log::info!("Proof height while sending receive packet {:?}", proof_height);
				let msg = MsgRecvPacket {
					packet: packet.clone(),
					proofs: Proofs::new(commitment_proof, None, None, None, proof_height)?,
					signer: sink.account_id(),
				};

				let value = msg.encode_vec()?;
				let msg = Any { value, type_url: msg.type_url() };
				messages.push(msg);
				log::info!(target: "hyperspace", "Sending packet {:?}", packet);
			},
			IbcEvent::WriteAcknowledgement(write_ack) => {
				let port_id = &write_ack.packet.destination_port.clone();
				let channel_id = &write_ack.packet.destination_channel.clone();
				if is_connection_delay && source.name() == "solana" {
					log::info!("Skipping acknowledgment packet relay because of connection delay");
					continue;
				}
				// let channel_response = source
				// 	.query_channel_end(write_ack.height, *channel_id, port_id.clone())
				// 	.await?;
				// let channel_end =
				// 	ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
				// 		Error::Custom(
				// 			"Failed to convert to concrete channel end from raw channel end"
				// 				.to_string(),
				// 		)
				// 	})?)?;
				// let connection_id = channel_end
				// 	.connection_hops
				// 	.get(0)
				// 	.ok_or_else(|| Error::Custom("Channel end missing connection id".to_string()))?
				// 	.clone();
				// let connection_response =
				// 	source.query_connection_end(write_ack.height, connection_id.clone()).await?;
				// let connection_end =
				// 	ConnectionEnd::try_from(connection_response.connection.ok_or_else(|| {
				// 		Error::Custom(format!("ConnectionEnd not found for {connection_id:?}"))
				// 	})?)?;
				// if !connection_end.delay_period().is_zero() {
				// 	log::debug!(target: "hyperspace", "Skipping write acknowledgement because of
				// connection delay {:?}", 		connection_end.delay_period());
				// 	// We can't send this packet immediately because of connection delays
				// 	is_connection_delay = true;
				// 	continue;
				// }
				let seq = u64::from(write_ack.packet.sequence);
				let packet = write_ack.packet;
				let packet_acknowledgement_response = source
					.query_packet_acknowledgement(write_ack.height, port_id, channel_id, seq)
					.await?;
				let acknowledgement = write_ack.ack;
				let commitment_proof =
					CommitmentProofBytes::try_from(packet_acknowledgement_response.proof)?;

				let proof_height = packet_acknowledgement_response
					.proof_height
					.expect("Proof height should be present");
				let proof_height =
					Height::new(proof_height.revision_number, proof_height.revision_height);
				log::info!("Proof height while sending acknowledgement {:?}", proof_height);
				let msg = MsgAcknowledgement {
					packet,
					acknowledgement: acknowledgement.into(),
					proofs: Proofs::new(commitment_proof, None, None, None, proof_height)?,

					signer: sink.account_id(),
				};

				let value = msg.encode_vec()?;
				let msg = Any { value, type_url: msg.type_url() };
				messages.push(msg)
			},
			_ => continue,
		}
	}

	// In light mode do not try to query channel state
	if let Some(Mode::Light) = mode {
		return Ok(messages);
	}

	Ok(messages)
}

/// Fetch the consensus state proof for the sink chain.
async fn query_host_consensus_state_proof(
	sink: &impl Chain,
	client_state: AnyClientState,
) -> Result<Vec<u8>, anyhow::Error> {
	let client_type = sink.client_type();
	let host_consensus_state_proof = if !client_type.contains("tendermint") {
		sink.query_host_consensus_state_proof(&client_state)
			.await?
			.expect("Host chain requires consensus state proof; qed")
	} else {
		vec![]
	};

	Ok(host_consensus_state_proof)
}

pub fn has_packet_events(event_types: &[IbcEventType]) -> bool {
	event_types
		.iter()
		.any(|event_type| matches!(event_type, &IbcEventType::SendPacket | &IbcEventType::WriteAck))
}
