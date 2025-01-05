use ibc::core::ics26_routing::msgs::Ics26Envelope;
use ibc_core_channel_types::{
	channel::Order,
	msgs::{
		ChannelMsg, MsgAcknowledgement, MsgChannelCloseConfirm, MsgChannelCloseInit,
		MsgChannelOpenAck, MsgChannelOpenConfirm, MsgChannelOpenInit, MsgChannelOpenTry,
		MsgRecvPacket, MsgTimeout, MsgTimeoutOnClose, PacketMsg,
	},
	packet::Packet,
	timeout::TimeoutHeight,
	Version as ChanVersion,
};
use ibc_core_client_types::{
	msgs::{ClientMsg, MsgCreateClient, MsgUpdateClient, MsgUpgradeClient},
	Height,
};
use ibc_core_commitment_types::commitment::{CommitmentPrefix, CommitmentProofBytes};
use ibc_core_connection_types::{
	msgs::{
		ConnectionMsg, MsgConnectionOpenAck, MsgConnectionOpenConfirm, MsgConnectionOpenInit,
		MsgConnectionOpenTry,
	},
	Counterparty,
};
use ibc_core_handler_types::msgs::MsgEnvelope;
use ibc_core_host_types::identifiers::{ChannelId, ClientId, ConnectionId, PortId, Sequence};
use ibc_new_primitives::{Signer, Timestamp};
use ibc_proto_new::{google::protobuf::Any, ibc::core::connection::v1::Version};
use primitives::mock::LocalClientTypes;
use prost::Message;
use std::str::FromStr;

const ROLLUP_CLIENT_STATE_TYPE_URL: &'static str = cf_solana::proto::ClientState::IBC_TYPE_URL;

use crate::{
	client_state::convert_old_client_state_to_new,
	consensus_state::convert_old_consensus_state_to_new,
};

pub fn convert_old_msgs_to_new(messages: Vec<Ics26Envelope<LocalClientTypes>>) -> Vec<MsgEnvelope> {
	let new_messages: Vec<MsgEnvelope> = messages
		.iter()
		.map(|message| match message {
			Ics26Envelope::Ics2Msg(msg) => match msg {
				ibc::core::ics02_client::msgs::ClientMsg::CreateClient(e) =>
					MsgEnvelope::Client(ClientMsg::CreateClient(MsgCreateClient::new(
						convert_old_client_state_to_new(e.client_state.clone()).into(),
						convert_old_consensus_state_to_new(e.consensus_state.clone()).into(),
						Signer::from(e.signer.as_ref().to_string()),
					))),
				ibc::core::ics02_client::msgs::ClientMsg::UpdateClient(e) => {
					let header = match &e.client_message {
						pallet_ibc::light_clients::AnyClientMessage::Tendermint(msg) =>
							ibc_proto::google::protobuf::Any::from(msg.clone()),
						pallet_ibc::light_clients::AnyClientMessage::Guest(msg) =>
							ibc_proto::google::protobuf::Any::from(msg.clone()),
						_ => panic!("Not supported"),
					};
					let new_any_header = Any { type_url: header.type_url, value: header.value };
					MsgEnvelope::Client(ClientMsg::UpdateClient(MsgUpdateClient {
						client_id: ClientId::from_str(e.client_id.as_str()).unwrap(),
						client_message: new_any_header,
						signer: Signer::from(e.signer.as_ref().to_string()),
					}))
				},
				ibc::core::ics02_client::msgs::ClientMsg::UpgradeClient(e) =>
					MsgEnvelope::Client(ClientMsg::UpgradeClient(MsgUpgradeClient {
						client_id: ClientId::from_str(e.client_id.as_str()).unwrap(),
						upgraded_client_state: convert_old_client_state_to_new(
							e.client_state.clone(),
						)
						.into(),
						upgraded_consensus_state: convert_old_consensus_state_to_new(
							e.consensus_state.clone(),
						)
						.into(),
						proof_upgrade_client: CommitmentProofBytes::try_from(
							e.proof_upgrade_client.clone(),
						)
						.unwrap(),
						proof_upgrade_consensus_state: CommitmentProofBytes::try_from(
							e.proof_upgrade_consensus_state.clone(),
						)
						.unwrap(),
						signer: Signer::from(e.signer.as_ref().to_string()),
					})),
			},
			Ics26Envelope::Ics3Msg(msg) => match msg {
				ibc::core::ics03_connection::msgs::ConnectionMsg::ConnectionOpenInit(e) =>
					MsgEnvelope::Connection(ConnectionMsg::OpenInit(MsgConnectionOpenInit {
						client_id_on_a: ClientId::from_str(e.client_id.as_str()).unwrap(),
						counterparty: Counterparty {
							client_id: ClientId::from_str(e.counterparty.client_id().as_str())
								.unwrap(),
							connection_id: e.counterparty.connection_id.clone().and_then(
								|conn_id| Some(ConnectionId::from_str(conn_id.as_str()).unwrap()),
							),
							prefix: CommitmentPrefix::try_from(
								e.counterparty.prefix().as_bytes().to_vec(),
							)
							.unwrap(),
						},
						version: e.version.clone().and_then(|ver| {
							let raw_version =
								ibc_proto::ibc::core::connection::v1::Version::from(ver.clone());
							Some(
								Version {
									identifier: raw_version.identifier,
									features: raw_version.features,
								}
								.try_into()
								.unwrap(),
							)
						}),
						delay_period: e.delay_period,
						signer: Signer::from(e.signer.as_ref().to_string()),
					})),
				#[allow(deprecated)]
				ibc::core::ics03_connection::msgs::ConnectionMsg::ConnectionOpenTry(e) => {
					let encoded_cs = ibc_proto::google::protobuf::Any::from(
						e.client_state.as_ref().unwrap().clone(),
					);
					MsgEnvelope::Connection(ConnectionMsg::OpenTry(MsgConnectionOpenTry {
						counterparty: Counterparty {
							client_id: ClientId::from_str(e.counterparty.client_id().as_str())
								.unwrap(),
							connection_id: e.counterparty.connection_id.clone().and_then(
								|conn_id| Some(ConnectionId::from_str(conn_id.as_str()).unwrap()),
							),
							prefix: CommitmentPrefix::try_from(
								e.counterparty.prefix().as_bytes().to_vec(),
							)
							.unwrap(),
						},
						delay_period: e.delay_period,
						signer: Signer::from(e.signer.as_ref().to_string()),
						client_id_on_b: ClientId::from_str(e.client_id.as_str()).unwrap(),
						client_state_of_b_on_a: Any {
							type_url: ROLLUP_CLIENT_STATE_TYPE_URL.to_string(),
							value: encoded_cs.value,
						},
						versions_on_a: e
							.counterparty_versions
							.iter()
							.map(|version| {
								let raw_version =
									ibc_proto::ibc::core::connection::v1::Version::from(
										version.clone(),
									);
								Version {
									identifier: raw_version.identifier,
									features: raw_version.features,
								}
								.try_into()
								.unwrap()
							})
							.collect(),
						proof_conn_end_on_a: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_client_state_of_b_on_a: CommitmentProofBytes::try_from(
							e.proofs.client_proof().clone().unwrap().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_consensus_state_of_b_on_a: CommitmentProofBytes::try_from(
							e.proofs.consensus_proof().unwrap().proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proofs_height_on_a: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						consensus_height_of_b_on_a: Height::new(
							e.proofs.consensus_proof().unwrap().height().revision_number,
							e.proofs.consensus_proof().unwrap().height().revision_height,
						)
						.unwrap(),
						proof_consensus_state_of_b: if e.host_consensus_state_proof.is_empty() {
							None
						} else {
							Some(
								CommitmentProofBytes::try_from(
									e.host_consensus_state_proof.clone(),
								)
								.unwrap(),
							)
						},
						previous_connection_id: String::default(),
					}))
				},
				ibc::core::ics03_connection::msgs::ConnectionMsg::ConnectionOpenAck(e) => {
					let encoded_cs = ibc_proto::google::protobuf::Any::from(
						e.client_state.as_ref().unwrap().clone(),
					);
					log::info!(
						"This is the proof height for consensus state {:?}",
						e.proofs.consensus_proof().unwrap().height()
					);

					MsgEnvelope::Connection(ConnectionMsg::OpenAck(MsgConnectionOpenAck {
						signer: Signer::from(e.signer.as_ref().to_string()),
						conn_id_on_a: ConnectionId::from_str(e.connection_id.as_str()).unwrap(),
						conn_id_on_b: ConnectionId::from_str(e.counterparty_connection_id.as_str())
							.unwrap(),
						client_state_of_a_on_b: Any {
							type_url: ROLLUP_CLIENT_STATE_TYPE_URL.to_string(),
							value: encoded_cs.value,
						},
						proof_conn_end_on_b: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_client_state_of_a_on_b: CommitmentProofBytes::try_from(
							e.proofs.client_proof().clone().unwrap().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_consensus_state_of_a_on_b: CommitmentProofBytes::try_from(
							e.proofs.consensus_proof().unwrap().proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proofs_height_on_b: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						consensus_height_of_a_on_b: Height::new(
							e.proofs.consensus_proof().unwrap().height().revision_number,
							e.proofs.consensus_proof().unwrap().height().revision_height,
						)
						.unwrap(),
						version: {
							let raw_version = ibc_proto::ibc::core::connection::v1::Version::from(
								e.version.clone(),
							);
							Version {
								identifier: raw_version.identifier,
								features: raw_version.features,
							}
							.try_into()
							.unwrap()
						},
						proof_consensus_state_of_a: if e.host_consensus_state_proof.is_empty() {
							None
						} else {
							Some(
								CommitmentProofBytes::try_from(
									e.host_consensus_state_proof.clone(),
								)
								.unwrap(),
							)
						},
					}))
				},
				ibc::core::ics03_connection::msgs::ConnectionMsg::ConnectionOpenConfirm(e) =>
					MsgEnvelope::Connection(ConnectionMsg::OpenConfirm(MsgConnectionOpenConfirm {
						signer: Signer::from(e.signer.as_ref().to_string()),
						conn_id_on_b: ConnectionId::from_str(e.connection_id.as_str()).unwrap(),
						proof_conn_end_on_a: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_a: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
					})),
			},
			Ics26Envelope::Ics4ChannelMsg(msg) => match msg {
				ibc::core::ics04_channel::msgs::ChannelMsg::ChannelOpenInit(e) =>
					MsgEnvelope::Channel(ChannelMsg::OpenInit(MsgChannelOpenInit {
						port_id_on_a: PortId::from_str(e.port_id.as_str()).unwrap(),
						connection_hops_on_a: e
							.channel
							.connection_hops
							.iter()
							.map(|hops| ConnectionId::from_str(hops.as_str()).unwrap())
							.collect(),
						port_id_on_b: PortId::from_str(e.port_id.as_str()).unwrap(),
						ordering: match e.channel.ordering {
							ibc::core::ics04_channel::channel::Order::Unordered => Order::Unordered,
							ibc::core::ics04_channel::channel::Order::Ordered => Order::Ordered,
						},
						signer: Signer::from(e.signer.as_ref().to_string()),
						version_proposal: ChanVersion::from_str(&e.channel.version.to_string())
							.unwrap(),
					})),
				#[allow(deprecated)]
				ibc::core::ics04_channel::msgs::ChannelMsg::ChannelOpenTry(e) =>
					MsgEnvelope::Channel(ChannelMsg::OpenTry(MsgChannelOpenTry {
						port_id_on_b: PortId::from_str(e.port_id.as_str()).unwrap(),
						connection_hops_on_b: e
							.channel
							.connection_hops
							.iter()
							.map(|hops| ConnectionId::from_str(hops.as_str()).unwrap())
							.collect(),
						port_id_on_a: PortId::from_str(e.channel.remote.port_id.as_str()).unwrap(),
						chan_id_on_a: ChannelId::new(
							e.channel.remote.channel_id.unwrap().sequence(),
						),
						version_supported_on_a: ChanVersion::from_str(
							&e.channel.version.to_string(),
						)
						.unwrap(),
						proof_chan_end_on_a: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_a: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						ordering: match e.channel.ordering {
							ibc::core::ics04_channel::channel::Order::Unordered => Order::Unordered,
							ibc::core::ics04_channel::channel::Order::Ordered => Order::Ordered,
						},
						signer: Signer::from(e.signer.as_ref().to_string()),
						version_proposal: ChanVersion::from_str(&e.channel.version.to_string())
							.unwrap(),
					})),
				ibc::core::ics04_channel::msgs::ChannelMsg::ChannelOpenAck(e) =>
					MsgEnvelope::Channel(ChannelMsg::OpenAck(MsgChannelOpenAck {
						port_id_on_a: PortId::from_str(e.port_id.as_str()).unwrap(),
						chan_id_on_a: ChannelId::new(e.channel_id.sequence()),
						proof_chan_end_on_b: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_b: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						signer: Signer::from(e.signer.as_ref().to_string()),
						chan_id_on_b: ChannelId::new(e.counterparty_channel_id.sequence()),
						version_on_b: ChanVersion::from_str(&e.counterparty_version.to_string())
							.unwrap(),
					})),
				ibc::core::ics04_channel::msgs::ChannelMsg::ChannelOpenConfirm(e) =>
					MsgEnvelope::Channel(ChannelMsg::OpenConfirm(MsgChannelOpenConfirm {
						port_id_on_b: PortId::from_str(e.port_id.as_str()).unwrap(),
						chan_id_on_b: ChannelId::new(e.channel_id.sequence()),
						proof_chan_end_on_a: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_a: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						signer: Signer::from(e.signer.as_ref().to_string()),
					})),
				ibc::core::ics04_channel::msgs::ChannelMsg::ChannelCloseInit(e) =>
					MsgEnvelope::Channel(ChannelMsg::CloseInit(MsgChannelCloseInit {
						port_id_on_a: PortId::from_str(e.port_id.as_str()).unwrap(),
						chan_id_on_a: ChannelId::new(e.channel_id.sequence()),
						signer: Signer::from(e.signer.as_ref().to_string()),
					})),
				ibc::core::ics04_channel::msgs::ChannelMsg::ChannelCloseConfirm(e) =>
					MsgEnvelope::Channel(ChannelMsg::CloseConfirm(MsgChannelCloseConfirm {
						port_id_on_b: PortId::from_str(e.port_id.as_str()).unwrap(),
						chan_id_on_b: ChannelId::new(e.channel_id.sequence()),
						proof_chan_end_on_a: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_a: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						signer: Signer::from(e.signer.as_ref().to_string()),
					})),
			},
			Ics26Envelope::Ics4PacketMsg(msg) => match msg {
				ibc::core::ics04_channel::msgs::PacketMsg::RecvPacket(e) =>
					MsgEnvelope::Packet(PacketMsg::Recv(MsgRecvPacket {
						packet: Packet {
							seq_on_a: Sequence::from(e.packet.sequence.0),
							port_id_on_a: PortId::from_str(e.packet.source_port.as_str()).unwrap(),
							chan_id_on_a: ChannelId::new(e.packet.source_channel.sequence()),
							port_id_on_b: PortId::from_str(e.packet.destination_port.as_str())
								.unwrap(),
							chan_id_on_b: ChannelId::new(e.packet.destination_channel.sequence()),
							data: e.packet.data.clone(),
							timeout_height_on_b: if e.packet.timeout_height.revision_number > 0 {
								TimeoutHeight::At(
									Height::new(
										e.packet.timeout_height.revision_number,
										e.packet.timeout_height.revision_height,
									)
									.unwrap(),
								)
							} else {
								TimeoutHeight::Never
							},
							timeout_timestamp_on_b: Timestamp::from_nanoseconds(
								e.packet.timeout_timestamp.nanoseconds(),
							)
							.unwrap(),
						},
						proof_commitment_on_a: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_a: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						signer: Signer::from(e.signer.as_ref().to_string()),
					})),
				ibc::core::ics04_channel::msgs::PacketMsg::AckPacket(e) =>
					MsgEnvelope::Packet(PacketMsg::Ack(MsgAcknowledgement {
						packet: Packet {
							seq_on_a: Sequence::from(e.packet.sequence.0),
							port_id_on_a: PortId::from_str(e.packet.source_port.as_str()).unwrap(),
							chan_id_on_a: ChannelId::new(e.packet.source_channel.sequence()),
							port_id_on_b: PortId::from_str(e.packet.destination_port.as_str())
								.unwrap(),
							chan_id_on_b: ChannelId::new(e.packet.destination_channel.sequence()),
							data: e.packet.data.clone(),
							timeout_height_on_b: if e.packet.timeout_height.revision_number > 0 {
								TimeoutHeight::At(
									Height::new(
										e.packet.timeout_height.revision_number,
										e.packet.timeout_height.revision_height,
									)
									.unwrap(),
								)
							} else {
								TimeoutHeight::Never
							},
							timeout_timestamp_on_b: Timestamp::from_nanoseconds(
								e.packet.timeout_timestamp.nanoseconds(),
							)
							.unwrap(),
						},
						signer: Signer::from(e.signer.as_ref().to_string()),
						acknowledgement: e
							.acknowledgement()
							.clone()
							.into_bytes()
							.try_into()
							.unwrap(),
						proof_acked_on_b: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_height_on_b: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
					})),
				ibc::core::ics04_channel::msgs::PacketMsg::ToPacket(e) =>
					MsgEnvelope::Packet(PacketMsg::Timeout(MsgTimeout {
						packet: Packet {
							seq_on_a: Sequence::from(e.packet.sequence.0),
							port_id_on_a: PortId::from_str(e.packet.source_port.as_str()).unwrap(),
							chan_id_on_a: ChannelId::new(e.packet.source_channel.sequence()),
							port_id_on_b: PortId::from_str(e.packet.destination_port.as_str())
								.unwrap(),
							chan_id_on_b: ChannelId::new(e.packet.destination_channel.sequence()),
							data: e.packet.data.clone(),
							timeout_height_on_b: if e.packet.timeout_height.revision_number > 0 {
								TimeoutHeight::At(
									Height::new(
										e.packet.timeout_height.revision_number,
										e.packet.timeout_height.revision_height,
									)
									.unwrap(),
								)
							} else {
								TimeoutHeight::Never
							},
							timeout_timestamp_on_b: Timestamp::from_nanoseconds(
								e.packet.timeout_timestamp.nanoseconds(),
							)
							.unwrap(),
						},
						signer: Signer::from(e.signer.as_ref().to_string()),
						proof_height_on_b: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						next_seq_recv_on_b: Sequence::from(e.next_sequence_recv.0),
						proof_unreceived_on_b: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
					})),
				ibc::core::ics04_channel::msgs::PacketMsg::ToClosePacket(e) =>
					MsgEnvelope::Packet(PacketMsg::TimeoutOnClose(MsgTimeoutOnClose {
						packet: Packet {
							seq_on_a: Sequence::from(e.packet.sequence.0),
							port_id_on_a: PortId::from_str(e.packet.source_port.as_str()).unwrap(),
							chan_id_on_a: ChannelId::new(e.packet.source_channel.sequence()),
							port_id_on_b: PortId::from_str(e.packet.destination_port.as_str())
								.unwrap(),
							chan_id_on_b: ChannelId::new(e.packet.destination_channel.sequence()),
							data: e.packet.data.clone(),
							timeout_height_on_b: if e.packet.timeout_height.revision_number > 0 {
								TimeoutHeight::At(
									Height::new(
										e.packet.timeout_height.revision_number,
										e.packet.timeout_height.revision_height,
									)
									.unwrap(),
								)
							} else {
								TimeoutHeight::Never
							},
							timeout_timestamp_on_b: Timestamp::from_nanoseconds(
								e.packet.timeout_timestamp.nanoseconds(),
							)
							.unwrap(),
						},
						signer: Signer::from(e.signer.as_ref().to_string()),
						proof_height_on_b: Height::new(
							e.proofs.height().revision_number,
							e.proofs.height().revision_height,
						)
						.unwrap(),
						next_seq_recv_on_b: Sequence::from(e.next_sequence_recv.0),
						proof_unreceived_on_b: CommitmentProofBytes::try_from(
							e.proofs.object_proof().as_bytes().to_vec(),
						)
						.unwrap(),
						proof_close_on_b: CommitmentProofBytes::try_from(
							e.proofs.other_proof().clone().unwrap().as_bytes().to_vec(),
						)
						.unwrap(),
					})),
			},
		})
		.collect();
	new_messages
}
