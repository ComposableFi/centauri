use anchor_client::{
	solana_client::{
		nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
	},
	solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
};
use guestchain::{BlockHeader, Signature as SignatureTrait};
use lib::hash::CryptoHash;
use serde::{Deserialize, Serialize};
use solana_ibc::events::Epoch;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use std::str::FromStr;
use tokio::runtime::Runtime;

use crate::{error::Error, utils::skip_fail};
use base64::Engine;
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientType,
			events::{
				Attributes as ClientAttributes, ClientMisbehaviour, CreateClient, UpdateClient,
				UpgradeClient,
			},
		},
		ics03_connection::events::{
			Attributes as ConnAttributes, OpenAck as ConnOpenAck, OpenConfirm as ConnOpenConfirm,
			OpenInit as ConnOpenInit, OpenTry as ConnOpenTry,
		},
		ics04_channel::{
			events::{
				AcknowledgePacket, CloseConfirm as ChanCloseConfirm, CloseInit as ChanCloseInit,
				OpenAck as ChanOpenAck, OpenConfirm as ChanOpenConfirm, OpenInit as ChanOpenInit,
				OpenTry as ChanOpenTry, ReceivePacket, SendPacket, TimeoutPacket,
				WriteAcknowledgement,
			},
			packet::{Packet, Sequence},
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ModuleId,
	},
	events::{IbcEvent, ModuleEvent, ModuleEventAttribute},
	timestamp::Timestamp,
	Height,
};
use pallet_ibc::light_clients::Signature;

pub fn convert_new_event_to_old(
	event: ibc_core_handler_types::events::IbcEvent,
	height: Height,
) -> Option<IbcEvent> {
	match event {
		ibc_core_handler_types::events::IbcEvent::CreateClient(e) => {
			let eve = CreateClient(ClientAttributes {
				height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
				client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
				client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
				consensus_height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
			});
			Some(IbcEvent::CreateClient(eve))
		},
		ibc_core_handler_types::events::IbcEvent::UpdateClient(e) => {
			let eve = UpdateClient {
				common: ClientAttributes {
					height: Height {
						revision_number: e.consensus_height().revision_number(),
						revision_height: e.consensus_height().revision_height(),
					},
					client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
					client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
					consensus_height: Height {
						revision_number: e.consensus_height().revision_number(),
						revision_height: e.consensus_height().revision_height(),
					},
				},
				header: Some(e.header().clone()),
			};
			Some(IbcEvent::UpdateClient(eve))
		},
		ibc_core_handler_types::events::IbcEvent::UpgradeClient(e) => {
			let eve = UpgradeClient(ClientAttributes {
				height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
				client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
				client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
				consensus_height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
			});
			Some(IbcEvent::UpgradeClient(eve))
		},
		ibc_core_handler_types::events::IbcEvent::ClientMisbehaviour(e) => {
			let eve = ClientMisbehaviour(ClientAttributes {
				height,
				client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
				client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
				consensus_height: height,
			});
			Some(IbcEvent::ClientMisbehaviour(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenInitConnection(e) => {
			let eve = ConnOpenInit(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_a().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: e
					.conn_id_on_b()
					.and_then(|conn| Some(ConnectionId::from_str(conn.as_str()).unwrap())),
				connection_id: Some(ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap()),
			});
			Some(IbcEvent::OpenInitConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenTryConnection(e) => {
			let eve = ConnOpenTry(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: e.conn_id_on_a().and_then(|conn_id| {
					Some(ConnectionId::from_str(conn_id.clone().as_str()).unwrap())
				}),
				connection_id: Some(
					ConnectionId::from_str(e.conn_id_on_b().clone().as_str()).unwrap(),
				),
			});
			Some(IbcEvent::OpenTryConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenAckConnection(e) => {
			let eve = ConnOpenAck(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_a().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: e
					.conn_id_on_b()
					.and_then(|conn| Some(ConnectionId::from_str(conn.as_str()).unwrap())),
				connection_id: Some(ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap()),
			});
			Some(IbcEvent::OpenAckConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenConfirmConnection(e) => {
			let eve = ConnOpenConfirm(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_a().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: Some(
					ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				),
				connection_id: e
					.conn_id_on_a()
					.and_then(|conn| Some(ConnectionId::from_str(conn.as_str()).unwrap())),
			});
			Some(IbcEvent::OpenConfirmConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenInitChannel(e) => {
			let eve = ChanOpenInit {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenInitChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenTryChannel(e) => {
			let eve = ChanOpenTry {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenTryChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenAckChannel(e) => {
			let eve = ChanOpenAck {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenAckChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenConfirmChannel(e) => {
			let eve = ChanOpenConfirm {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenConfirmChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::CloseInitChannel(e) => {
			let eve = ChanCloseInit {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
				connection_id: ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::CloseInitChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::CloseConfirmChannel(e) => {
			let eve = ChanCloseConfirm {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::CloseConfirmChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::SendPacket(e) => {
			let eve = SendPacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: e.packet_data().to_vec(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::SendPacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::ReceivePacket(e) => {
			let eve = ReceivePacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_b().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: e.packet_data().to_vec(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::ReceivePacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(e) => {
			let eve = WriteAcknowledgement {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: e.packet_data().to_vec(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
				ack: e.acknowledgement().as_bytes().to_vec(),
			};
			Some(IbcEvent::WriteAcknowledgement(eve))
		},
		ibc_core_handler_types::events::IbcEvent::AcknowledgePacket(e) => {
			let eve = AcknowledgePacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: Vec::new(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::AcknowledgePacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::TimeoutPacket(e) => {
			let eve = TimeoutPacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: Vec::new(), // Not sure about this
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::TimeoutPacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::ChannelClosed(_) => None,
		ibc_core_handler_types::events::IbcEvent::Module(e) => {
			let attributes: Vec<ModuleEventAttribute> = e
				.attributes
				.iter()
				.map(|attr| ModuleEventAttribute {
					key: attr.clone().key,
					value: attr.clone().value,
				})
				.collect();
			let eve = ModuleEvent {
				kind: e.kind,
				module_name: ModuleId::from_str("transfer").unwrap(),
				attributes,
			};
			Some(IbcEvent::AppModule(eve))
		},
		ibc_core_handler_types::events::IbcEvent::Message(_) => None,
	}
}

pub fn get_ibc_events_from_logs(
	logs: Vec<String>,
) -> (Vec<ibc_core_handler_types::events::IbcEvent>, u64) {
	let (events, proof_height) = get_events_from_logs(logs);
	let events: Vec<ibc_core_handler_types::events::IbcEvent> = events
		.iter()
		.filter_map(|event| match event {
			solana_ibc::events::Event::IbcEvent(e) => Some(e.clone()),
			_ => None,
		})
		.collect();
	(events, proof_height)
}

pub async fn get_client_state_at_height(
	rpc: RpcClient,
	client_id: ClientId,
	program_id: Pubkey,
	upto_height: u64,
) -> Option<solana_ibc::client_state::AnyClientState> {
	log::info!("Getting client states at height {:?}", upto_height);
	let mut before_hash = None;
	let mut current_height = upto_height;
	while current_height >= upto_height {
		let (transactions, last_searched_hash) =
			get_previous_transactions(&rpc, program_id, before_hash, SearchIn::IBC)
				.await
				.ok()?;
		if transactions.is_empty() {
			break;
		}
		before_hash = Some(
			anchor_client::solana_sdk::signature::Signature::from_str(&last_searched_hash).ok()?,
		);
		for tx in transactions {
			let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
				solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
				_ => Vec::new(),
			};
			let (events, height) = get_events_from_logs(logs);
			let client_state_events: Vec<solana_ibc::events::ClientStateUpdate> = events
				.iter()
				.filter_map(|event| match event {
					solana_ibc::events::Event::ClientStateUpdate(e) => {
						if e.client_id.as_str() == client_id.as_str() {
							Some(e.clone())
						} else {
							None
						}
					},
					_ => None,
				})
				.collect();
			current_height = height;
			if height == 0 || client_state_events.is_empty() {
				continue;
			}
			log::info!("Found height {:?}", height);
			if height < upto_height {
				break;
			}
			// There can be only one client state event in a tx
			let current_client_state = &client_state_events[0];
			let any_client_state: solana_ibc::client_state::AnyClientState =
				borsh::BorshDeserialize::try_from_slice(current_client_state.state.as_ref())
					.unwrap();
			log::info!("This is any client state {:?}", any_client_state);
			return Some(any_client_state);
		}
	}
	None
}

pub fn get_events_from_logs(logs: Vec<String>) -> (Vec<solana_ibc::events::Event<'static>>, u64) {
	let serialized_events: Vec<&str> = logs
		.iter()
		.filter_map(|log| {
			if log.starts_with("Program data: ") {
				Some(log.strip_prefix("Program data: ").unwrap())
			} else {
				None
			}
		})
		.collect();
	let height_str = logs
		.iter()
		.find_map(|log| {
			if log.starts_with("Program log: Current Block height ") {
				Some(log.strip_prefix("Program log: Current Block height ").unwrap())
			} else {
				None
			}
		})
		.map_or("0", |height| height);
	let height = height_str.parse::<u64>().unwrap();
	let events: Vec<solana_ibc::events::Event> = serialized_events
		.iter()
		.filter_map(|event| {
			let decoded_event = base64::prelude::BASE64_STANDARD.decode(event);
			if let Ok(decoded_event) = decoded_event {
				let decoded_event: Result<solana_ibc::events::Event, String> =
					borsh::BorshDeserialize::try_from_slice(&decoded_event).map_err(|e| {
						// log::error!("These are logs {:?}", logs);
						// log::error!("This is decoded event {:?}", decoded_event);
						e.to_string()
					});
				if let Ok(decoded_event) = decoded_event {
					return Some(decoded_event);
				}
			}
			None
		})
		.collect();
	(events, height)
}

pub async fn _get_signatures_for_blockhash(
	rpc: RpcClient,
	program_id: Pubkey,
	blockhash: CryptoHash,
) -> Result<(Vec<(Pubkey, Signature)>, BlockHeader), String> {
	let (transactions, _) = get_previous_transactions(&rpc, program_id, None, SearchIn::GuestChain)
		.await
		.map_err(|e| e.to_string())?;

	let mut signatures = Vec::new();
	// let mut index = 0;
	for tx in transactions {
		let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
			_ => Vec::new(),
		};
		let (events, _proof_height) = get_events_from_logs(logs);
		// Find block signed events with blockhash
		let block_header: Vec<Option<BlockHeader>> = events
			.iter()
			.map(|event| match event {
				solana_ibc::events::Event::NewBlock(e) => {
					println!("This is new block event {:?}", e.block_header.0.block_height);
					let new_blockhash = e.block_header.0.calc_hash();
					if blockhash == new_blockhash {
						println!("New block event where it is true");
						return Some(e.block_header.0.clone());
					}
					None
				},
				solana_ibc::events::Event::BlockSigned(e) => {
					println!("This is block signed event {:?}", e.block_height);
					if e.block_hash == blockhash {
						println!("This is block signed in side blockhash");
						signatures.push((
							Pubkey::new_from_array(e.pubkey.clone().into()),
							Signature::from_bytes(&e.signature.to_vec()).unwrap(),
						))
					};
					None
				},
				_ => None,
			})
			.collect();
		if let Some(header) = block_header.iter().find(|b| b.is_some()) {
			return Ok((signatures, header.clone().unwrap()));
		}
	}
	Err("Couldnt find blocks".to_string())
}

pub async fn get_header_from_height(
	rpc: RpcClient,
	program_id: Pubkey,
	height: u64,
) -> Option<BlockHeader> {
	log::info!("Getting header for height {}", height);
	let mut before_hash = None;
	let mut block_header = None;
	while block_header.is_none() {
		let (transactions, last_searched_hash) =
			get_previous_transactions(&rpc, program_id, before_hash, SearchIn::GuestChain)
				.await
				.ok()?;
		if transactions.is_empty() {
			break;
		}
		before_hash = Some(
			anchor_client::solana_sdk::signature::Signature::from_str(&last_searched_hash).ok()?,
		);
		log::info!("THis is before hash {:?} {:?}", before_hash, last_searched_hash);
		for tx in transactions {
			let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
				solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
				_ => Vec::new(),
			};
			let (events, _proof_height) = get_events_from_logs(logs);
			// Find block signed events with blockhash
			block_header = events.iter().find_map(|event| match event {
				solana_ibc::events::Event::NewBlock(e) => {
					println!(
						"This is new block event when fetching for height {:?}",
						e.block_header.0.block_height
					);
					let block_height = u64::from(e.block_header.0.block_height);
					if block_height == height {
						println!("New block event where it is true for height {:?}", height);
						return Some(e.block_header.0.clone());
					}
					None
				},
				_ => None,
			});
			if block_header.is_some() {
				return block_header;
			}
		}
	}
	block_header
}

pub async fn get_signatures_upto_height(
	rpc: RpcClient,
	program_id: Pubkey,
	upto_height: u64,
) -> Result<
	(
		Vec<(
			Vec<(Pubkey, Signature)>,
			BlockHeader,
			Option<guestchain::Epoch<sigverify::ed25519::PubKey>>,
		)>,
		Vec<ibc_core_handler_types::events::IbcEvent>,
	),
	Error,
> {
	let mut current_height = upto_height;
	let mut before_hash = None;
	let mut all_signatures = Vec::new();
	let mut all_block_headers = Vec::new();
	let mut all_ibc_events = Vec::new();
	// let mut finalized_heights = Vec::new();
	log::info!("This is upto height {:?}", upto_height);
	while current_height >= upto_height {
		let (transactions, last_searched_hash) =
			get_previous_transactions(&rpc, program_id, before_hash, SearchIn::GuestChain).await?;
		if transactions.is_empty() {
			break;
		}
		before_hash =
			Some(anchor_client::solana_sdk::signature::Signature::from_str(&last_searched_hash)?);
		for tx in transactions {
			let transaction_err = tx.result.transaction.meta.clone().unwrap().err;
			if transaction_err.is_some() {
				// match tx.result.transaction.transaction {
				// 	solana_transaction_status::EncodedTransaction::Json(e) => {
				// 		println!("Error in transaction {:?}", e.signatures);
				// 	},
				//   _ => panic!("WTF")
				// }
				continue;
			}
			let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
				solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
				_ => Vec::new(),
			};
			let (events, _proof_height) = get_events_from_logs(logs.clone());
			let (ibc_events, _) = get_ibc_events_from_logs(logs);
			all_ibc_events.extend(ibc_events);
			for event in events {
				match event {
					solana_ibc::events::Event::NewBlock(e) => {
						println!(
							"This is new block event when fetching for height {:?}",
							e.block_header.0.block_height
						);
						let block_height = u64::from(e.block_header.0.block_height);
						current_height = block_height;
						if block_height >= upto_height {
							all_block_headers.push((e.block_header.0.clone(), e.epoch));
						} else {
							log::info!("breaking out of upto height");
						}
					},
					solana_ibc::events::Event::BlockSigned(e) => {
						all_signatures.push(e);
					},
					// solana_ibc::events::Event::BlockFinalised(e) => {
					// 	let block_height = u64::from(e.block_height);
					// 	finalized_heights.push(block_height);
					// },
					_ => (),
				}
			}
			if current_height < upto_height {
				break;
			}
		}
	}
	Ok((
		all_block_headers
			.iter()
			.filter_map(|(b, epoch)| {
				let signatures_for_header: Vec<_> = all_signatures
					.iter()
					.filter_map(|s| {
						if s.block_height == b.block_height {
							Some((
								Pubkey::new_from_array(s.pubkey.clone().into()),
								Signature::from_bytes(&s.signature.to_vec()).unwrap(),
							))
						} else {
							None
						}
					})
					.collect();
				if signatures_for_header.is_empty() {
					return None;
				}
				let epoch = epoch.as_ref().and_then(|e| Some(e.0.clone()));
				Some((signatures_for_header, b.clone(), epoch))
			})
			.collect(),
		all_ibc_events,
	))
}

pub async fn get_previous_transactions(
	rpc: &RpcClient,
	program_id: Pubkey,
	before_hash: Option<anchor_client::solana_sdk::signature::Signature>,
	search_in: SearchIn,
) -> Result<(Vec<Response>, String), Error> {
	let search_address = match search_in {
		SearchIn::IBC => {
			let storage_seeds = &[solana_ibc::SOLANA_IBC_STORAGE_SEED];
			Pubkey::find_program_address(storage_seeds, &program_id).0
		},
		SearchIn::GuestChain => program_id,
	};
	let transaction_signatures = rpc
		.get_signatures_for_address_with_config(
			&search_address, // Since ibc storage is only used for ibc and not for guest chain
			GetConfirmedSignaturesForAddress2Config {
				limit: Some(150),
				before: before_hash,
				commitment: Some(CommitmentConfig::finalized()),
				..Default::default()
			},
		)
		.await?;
	if transaction_signatures.is_empty() {
		return Ok((vec![], before_hash.map_or("".to_string(), |sig| sig.to_string())));
	}
	let last_searched_hash = transaction_signatures
		.last()
		.map_or("".to_string(), |sig| sig.signature.clone());
	let tx_signatures = transaction_signatures.clone();
	let mut body = vec![];
	for sig in transaction_signatures {
		let signature = sig.signature.clone();
		let payload = Payload {
			jsonrpc: "2.0".to_string(),
			id: 1 as u64,
			method: "getTransaction".to_string(),
			params: (
				signature,
				Param { commitment: "finalized".to_string(), maxSupportedTransactionVersion: 0 },
			),
		};
		body.push(payload);
	}
	let url = rpc.url();
	let ret = tokio::task::spawn_blocking(move || {
		for _ in 0..5 {
			let response = reqwest::blocking::Client::new().post(url.clone()).json(&body).send();
			let response = skip_fail!(response);
			let response: std::result::Result<Vec<Response>, reqwest::Error> = response.json();
			let transactions = skip_fail!(response);
			return (transactions, last_searched_hash);
		}
		log::error!("Couldnt get transactions after 5 retries");
		log::info!("These are signatures {:?}", tx_signatures);
		(vec![], "".to_string())
	})
	.await
	.map_err(|e| Error::Custom(e.to_string()))?;
	Ok(ret)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	jsonrpc: String,
	id: u64,
	method: String,
	params: (String, Param),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
	commitment: String,
	maxSupportedTransactionVersion: u16,
}

pub enum SearchIn {
	IBC,
	GuestChain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub jsonrpc: String,
	pub id: u64,
	pub result: EncodedConfirmedTransactionWithStatusMeta,
}

#[test]
pub fn testing_signatures() {
	println!("I am testing signatures");
	let rpc = RpcClient::new(
		"https://mainnet.helius-rpc.com/?api-key=65520d87-04b2-43a5-b5d5-35d5db0601b3".to_string(),
	);
	let program_id = Pubkey::from_str("2HLLVco5HvwWriNbUhmVwA2pCetRkpgrqwnjcsZdyTKT").unwrap();
	let upto_height = 116806;
	println!("I am testing signatures");
	let signatures =
		Runtime::new()
			.unwrap()
			.block_on(get_signatures_upto_height(rpc, program_id, upto_height));
	signatures.0.iter().for_each(|sig| {
		println!("Height {}", sig.1.block_height);
	})
}

#[tokio::test]
pub async fn testing_events_final() {
	let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());
	let mut last_hash = None;
	loop {
		let (events, prev) = get_previous_transactions(
			&rpc,
			Pubkey::from_str("9FeHRJLHJSEw4dYZrABHWTRKruFjxDmkLtPmhM5WFYL7").unwrap(),
			last_hash,
			SearchIn::IBC,
		)
		.await;
		if events.is_empty() {
			println!("No events found");
			break;
		}
		println!("Received events {}", events.len());
		last_hash = Some(anchor_client::solana_sdk::signature::Signature::from_str(&prev).unwrap());
	}
}

#[test]
pub fn testing_events() {
	let events = vec![
	"Program data: ABUC".to_string(),
	"Program data: AA+kAAAAeyJhbW91bnQiOiIxNjAwMDA4Mzk5NDAxNjg3NjgwIiwiZGVub20iOiJwcGljYSIsInJlY2VpdmVyIjoib3h5ekVzVWo5Q1Y2SHNxUENVWnFWd3JGSkp2cGQ5aUNCclBkelRCV0xCYiIsInNlbmRlciI6ImNlbnRhdXJpMWYwcm1kZnVmM2s4c3FubXJmYTBwbHQzeGdtM2xmeDR2cXU0dHA2In0BAQAAAAAAAAAGCAAAAAAAAAAaVjgvdcMXAQAAAAAAAAAIAAAAdHJhbnNmZXIKAAAAY2hhbm5lbC0xNQgAAAB0cmFuc2ZlcgkAAABjaGFubmVsLTABDAAAAGNvbm5lY3Rpb24tMA==".to_string(),
	"Program data: ABUC".to_string(),
	"Program data: ABCkAAAAeyJhbW91bnQiOiIxNjAwMDA4Mzk5NDAxNjg3NjgwIiwiZGVub20iOiJwcGljYSIsInJlY2VpdmVyIjoib3h5ekVzVWo5Q1Y2SHNxUENVWnFWd3JGSkp2cGQ5aUNCclBkelRCV0xCYiIsInNlbmRlciI6ImNlbnRhdXJpMWYwcm1kZnVmM2s4c3FubXJmYTBwbHQzeGdtM2xmeDR2cXU0dHA2In0BAQAAAAAAAAAGCAAAAAAAAAAaVjgvdcMXAQAAAAAAAAAIAAAAdHJhbnNmZXIKAAAAY2hhbm5lbC0xNQgAAAB0cmFuc2ZlcgkAAABjaGFubmVsLTARAAAAeyJyZXN1bHQiOiJBUT09In0MAAAAY29ubmVjdGlvbi0w".to_string(),
	"Program data: ABQSAAAAZGVub21pbmF0aW9uX3RyYWNlAQAAAAUAAABkZW5vbRgAAAB0cmFuc2Zlci9jaGFubmVsLTAvcHBpY2E=".to_string(),
	"Program data: ABQVAAAAZnVuZ2libGVfdG9rZW5fcGFja2V0BwAAAAYAAABtb2R1bGUIAAAAdHJhbnNmZXIGAAAAc2VuZGVyLwAAAGNlbnRhdXJpMWYwcm1kZnVmM2s4c3FubXJmYTBwbHQzeGdtM2xmeDR2cXU0dHA2CAAAAHJlY2VpdmVyKwAAAG94eXpFc1VqOUNWNkhzcVBDVVpxVndyRkpKdnBkOWlDQnJQZHpUQldMQmIFAAAAZGVub20FAAAAcHBpY2EGAAAAYW1vdW50EwAAADE2MDAwMDgzOTk0MDE2ODc2ODAEAAAAbWVtbwAAAAAHAAAAc3VjY2VzcwQAAAB0cnVl".to_string(),
	];
	let (eves, height) = get_events_from_logs(events);
	eves.iter().for_each(|event| println!("{:?}", event));
	let seqs = vec![1];
	let port_id = PortId::transfer();
	let channel_id = ChannelId::new(15);
	let recv_packet_events: Vec<_> = eves
		.iter()
		.filter_map(|tx| match tx {
			solana_ibc::events::Event::IbcEvent(e) => match e {
				ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(packet) =>
					if packet.chan_id_on_a().as_str() == &channel_id.to_string() &&
						packet.port_id_on_a().as_str() == port_id.as_str() &&
						seqs.iter().find(|&&seq| packet.seq_on_a().value() == seq).is_some()
					{
						println!("We found packet");
						Some(packet)
					} else {
						None
					},
				_ => None,
			},
			_ => None,
		})
		.collect();
	// let client_state_logs: Vec<&str> = events
	// 	.iter()
	// 	.filter_map(|log| {
	// 		if log.starts_with("Program logged: This is updated client state ") {
	// 			Some(log.strip_prefix("Program logged: This is updated client state ").unwrap())
	// 		} else {
	// 			None
	// 		}
	// 	})
	// 	.collect();
	// // There can be only one client state event in a tx
	// let client_state_log = client_state_logs[0];
	// // Remove the square brackets and whitespace, then split the string into an iterator of &str,
	// // each representing a byte. Then parse each &str to a u8 and collect into a Vec<u8>
	// let bytes: Vec<u8> = client_state_log
	// 	.trim_matches(|c: char| c == '[' || c == ']') // Trim the square brackets
	// 	.split(", ") // Split the string into individual numbers
	// 	.map(|s| s.parse::<u8>().unwrap()) // Convert each number from &str to u8
	// 	.collect(); // Collect into a Vec<u8>
	// let any_client_state: solana_ibc::client_state::AnyClientState =
	// 	borsh::BorshDeserialize::try_from_slice(bytes.as_slice()).unwrap();
	// println!("This is any client state {:?}", any_client_state);
}
