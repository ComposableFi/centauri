// #![feature(more_qualified_paths)]
extern crate alloc;

use anchor_client::{
	solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig},
	solana_sdk::{
		compute_budget::ComputeBudgetInstruction,
		instruction::Instruction,
		system_instruction::transfer,
		transaction::{Transaction, VersionedTransaction},
	},
};
use anchor_spl::associated_token::get_associated_token_address;
use client::FinalityEvent;
use client_state::convert_new_client_state_to_old;
use consensus_state::convert_new_consensus_state_to_old;
use core::{pin::Pin, str::FromStr, time::Duration};
use futures::future::join_all;
use guestchain::{BlockHeader, Epoch, PubKey, Validator};
use ibc_core_channel_types::msgs::PacketMsg;
use ibc_core_client_types::msgs::ClientMsg;
use ibc_core_handler_types::msgs::MsgEnvelope;
use ics07_tendermint::{
	client_message::ClientMessage, client_state::ClientState as TmClientState,
	consensus_state::ConsensusState as TmConsensusState,
};
use msgs::convert_old_msgs_to_new;
use serde::{Deserialize, Serialize};
use solana_transaction_status::UiTransactionEncoding;
use std::{num::NonZeroU128, ops::Deref, thread::sleep, time::Instant};
use tendermint::{Hash, Time};
use tendermint_proto::Protobuf;
use tokio::{
	sync::mpsc::unbounded_channel,
	task::{spawn_blocking, JoinSet},
};

use anchor_client::{
	solana_client::{
		nonblocking::pubsub_client::PubsubClient,
		rpc_client::GetConfirmedSignaturesForAddress2Config,
		rpc_config::{
			RpcBlockSubscribeConfig, RpcBlockSubscribeFilter, RpcTransactionLogsConfig,
			RpcTransactionLogsFilter,
		},
	},
	solana_sdk::{
		commitment_config::CommitmentConfig, signature::Signature, signer::Signer as AnchorSigner,
	},
};
use anchor_lang::prelude::*;
use chrono::{DateTime, Utc};
use error::Error;
use ibc::{
	applications::transfer::{Amount, BaseDenom, PrefixedCoin, PrefixedDenom, TracePath},
	core::{
		ics02_client::{
			events::{NewBlock, UpdateClient},
			msgs::update_client::MsgUpdateAnyClient,
			trust_threshold::TrustThreshold,
		},
		ics23_commitment::specs::ProofSpecs,
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::msgs::Ics26Envelope,
	},
	events::IbcEvent,
	timestamp::Timestamp,
	tx_msg::Msg,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			Channel, Counterparty as ChanCounterparty, IdentifiedChannel, QueryChannelResponse,
			QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
			QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{
			ConnectionEnd, Counterparty as ConnCounterparty, IdentifiedConnection,
			QueryConnectionResponse, Version,
		},
	},
};
use ibc_proto_new::Protobuf as _;
use jito_protos::{
	convert::versioned_tx_from_packet,
	searcher::{
		mempool_subscription, searcher_service_client::SearcherServiceClient,
		ConnectedLeadersRegionedRequest, GetTipAccountsRequest, MempoolSubscription,
		NextScheduledLeaderRequest, PendingTxNotification, ProgramSubscriptionV0,
		SubscribeBundleResultsRequest, WriteLockedAccountSubscriptionV0,
	},
};
use lib::hash::CryptoHash;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
use primitives::{
	mock::LocalClientTypes, Chain, CommonClientState, IbcProvider, KeyProvider, LightClientSync,
	MisbehaviourHandler, UndeliveredType, UpdateType,
};
use solana_ibc::events::Event;
use std::{result::Result, sync::Arc};
use tokio_stream::{Stream, StreamExt};

use crate::{
	client::TransactionSender,
	events::{get_events_from_logs, SearchIn},
};
pub use crate::{
	client::{DeliverIxType, SolanaClient, SolanaClientConfig},
	events::convert_new_event_to_old,
};
use ibc::applications::transfer::packet::PacketData;
use ibc_rpc::PacketInfo;
use solana_ibc::storage::{SequenceKind, Serialised};
use sqlx::Row;
use tokio::time::timeout;
use trie_ids::{ClientIdx, ConnectionIdx, PortChannelPK, Tag, TrieKey};

pub mod client;
mod client_state;
mod consensus_state;
mod error;
mod events;
// mod jito;
mod msgs;
#[cfg(feature = "testing")]
mod test_provider;
mod utils;

const SOLANA_IBC_STORAGE_SEED: &[u8] = b"private";
const TRIE_SEED: &[u8] = b"trie";
const CHAIN_SEED: &[u8] = b"chain";
pub const NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER: u64 = 250;
pub const WRITE_ACCOUNT_SEED: &[u8] = b"write";
pub const SIGNATURE_ACCOUNT_SEED: &[u8] = b"signature";

pub const BLOCK_ENGINE_URL: &str = "https://mainnet.block-engine.jito.wtf";

pub const MIN_TIME_UNTIL_UPDATE: u64 = 30 * 60; // 30 mins

pub struct InnerAny {
	pub type_url: String,
	pub value: Vec<u8>,
}

#[async_trait::async_trait]
impl IbcProvider for SolanaClient {
	type FinalityEvent = FinalityEvent;

	type TransactionId = String;

	type AssetId = String;

	type Error = Error;

	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, primitives::UpdateType)>, anyhow::Error>
	where
		T: Chain,
	{
		log::info!("Came into solana lts events");
		// let (finality_blockhash, finality_height) = match finality_event {
		// 	FinalityEvent::Guest { blockhash, block_height } => (blockhash, block_height),
		// };
		let chain_account = self.get_chain_storage().await?;
		let finality_height = chain_account.head().unwrap().block_height.into();
		log::info!("This is solaan height {:?}", finality_height);
		let client_id = self.client_id();
		let latest_cp_height = counterparty.latest_height_and_timestamp().await?.0;
		log::info!("this is the latest cp height {:?}", latest_cp_height);
		let latest_cp_client_state =
			counterparty.query_client_state(latest_cp_height, client_id.clone()).await?;
		let client_state_response = latest_cp_client_state
			.client_state
			.ok_or_else(|| Error::Custom("counterparty returned empty client state".to_string()))?;
		log::info!("This is the type url in solana {:?}", client_state_response.type_url);
		let AnyClientState::Guest(client_state) =
			AnyClientState::decode_recursive(client_state_response.clone(), |c| {
				matches!(c, AnyClientState::Guest(_))
			})
			.or_else(|| {
				log::info!("This is wasm {:?}", client_state_response);
				let wasm_client_state =
					AnyClientState::decode_recursive(client_state_response, |c| {
						matches!(c, AnyClientState::Wasm(_))
					})
					.unwrap();
				Some(wasm_client_state.unpack_recursive().clone())
			})
			.unwrap()
		else {
			unreachable!()
		};
		log::info!("This is client state {:?}", client_state);
		let latest_cp_client_height = u64::from(client_state.0.latest_height);
		log::info!("This is counterparty client height {:?}", latest_cp_client_height);
		let (all_signatures, new_block_events) = events::get_signatures_upto_height(
			self.rpc_client(),
			self.solana_ibc_program_id,
			latest_cp_client_height + 1,
		)
		.await?;

		let earliest_block_header = all_signatures.last();

		log::info!("This is all events {:?}", new_block_events);
		let mut channel_and_port_ids = self.channel_whitelist();
		channel_and_port_ids.extend(counterparty.channel_whitelist());
		let block_events: Vec<IbcEvent> = new_block_events
			.iter()
			.filter_map(|event| {
				let event = convert_new_event_to_old(
					event.clone(),
					Height::new(1, u64::from(finality_height)),
				);
				if let Some(event) = event {
					let is_filtered = primitives::filter_events_by_ids(
						&event,
						&[self.client_id(), counterparty.client_id()],
						&[self.connection_id(), counterparty.connection_id()]
							.into_iter()
							.flatten()
							.collect::<Vec<_>>(),
						&channel_and_port_ids,
					);
					if is_filtered {
						Some(event)
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect();

		let mut updates = Vec::new();
		let mut rev_all_signatures = all_signatures.clone();
		// Reversing so that updates are sent in ascending order of their height.
		rev_all_signatures.reverse();
		for (signatures, block_header, epoch) in rev_all_signatures {
			if (block_header.next_epoch_commitment.is_none() &&
				u64::from(block_header.block_height) != finality_height) ||
				epoch.is_none()
			{
				log::info!(
					"Skipping for height {} and finality height {}",
					block_header.block_height,
					finality_height
				);
				continue;
			}

			let block_hash = block_header.calc_hash();
			let block_height: u64 = block_header.block_height.into();
			let validators = epoch.unwrap().validators().to_vec();
			let all_validators: Vec<Validator<pallet_ibc::light_clients::PubKey>> = validators
				.iter()
				.map(|validator| {
					let new_validator: Validator<pallet_ibc::light_clients::PubKey> =
						Validator::new(
							PubKey::from_bytes(&validator.pubkey.to_vec()).unwrap(),
							validator.stake,
						);
					new_validator
				})
				.collect();
			let final_signatures: Vec<_> = signatures
				.iter()
				.enumerate()
				.map(|(index, (validator, signature))| {
					let validator_idx = all_validators
						.iter()
						.position(|v| {
							v.pubkey ==
								PubKey::from_bytes(&validator.to_bytes().as_slice()).unwrap()
						})
						.unwrap();
					(validator_idx as u16, signature.clone())
				})
				.collect();
			let current_epoch = Epoch::new_with(all_validators, |total| {
				let quorum = NonZeroU128::new(total.get() / 2 + 1).unwrap();
				// min_quorum_stake may be greater than total_stake so we’re not
				// using .clamp to make sure we never return value higher than
				// total_stake.
				println!("THis is total {:?} and quorum {:?}", total, quorum);
				quorum.max(NonZeroU128::new(1000).unwrap()).min(total)
			})
			.unwrap();
			let guest_header = cf_guest_og::Header {
				genesis_hash: chain_account.genesis().unwrap().clone(),
				block_hash,
				block_header: block_header.clone(),
				epoch_commitment: current_epoch.calc_commitment(),
				epoch: current_epoch,
				signatures: final_signatures,
			};
			log::info!(
				"Height: {:?} signature {:?} and finality height {:?}",
				block_height,
				signatures,
				finality_height
			);
			let msg = MsgUpdateAnyClient::<LocalClientTypes> {
				client_id: self.client_id(),
				client_message: AnyClientMessage::Guest(cf_guest::ClientMessage::from(
					guest_header,
				)),
				signer: counterparty.account_id(),
			};
			let value = msg.clone().encode_vec();
			log::info!("This is wihle update {:?}", value);

			let update = if u64::from(block_header.block_height) == finality_height {
				let time_since_last_update =
					if let Some(earliest_block_header) = earliest_block_header {
						u64::from(block_header.timestamp_ns) -
							u64::from(earliest_block_header.1.timestamp_ns)
					} else {
						0
					};
				log::info!(
					"TIme since last update on solana {}",
					time_since_last_update / 1_000_000_000
				);
				if time_since_last_update > MIN_TIME_UNTIL_UPDATE * 1_000_000_000 {
					log::info!("--------------------------PURPOSE UPDATE------------------------");
				} else {
					log::info!("-----------------------NO UPDATE---------------------------");
				}
				(
					Any { type_url: msg.type_url(), value },
					Height::new(1, finality_height),
					block_events.clone(),
					if !block_events.is_empty() ||
						time_since_last_update > MIN_TIME_UNTIL_UPDATE * 1_000_000_000
					{
						log::info!(
							"--------------------------Updating at {}---------------------------",
							finality_height
						);
						UpdateType::Mandatory
					} else {
						UpdateType::Optional
					},
				)
			} else {
				log::info!("Mandatory update due to change in epoch");
				(
					Any { type_url: msg.type_url(), value },
					Height::new(1, block_height),
					Vec::new(),
					UpdateType::Mandatory,
				)
			};
			updates.push(update);
		}
		Ok(updates)
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		let (tx, rx) = unbounded_channel();
		let ws_url = self.ws_url.clone();
		let program_id = self.solana_ibc_program_id;
		tokio::spawn(async move {
			let client =
				PubsubClient::new(&ws_url).await.expect("Could not connect to solana websocket");
			let (mut logs_subscription, _receiver) = client
				.logs_subscribe(
					RpcTransactionLogsFilter::Mentions(vec![program_id.to_string()]),
					RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::finalized()) },
				)
				.await
				.unwrap();

			loop {
				let response = timeout(Duration::from_secs(10), logs_subscription.next()).await;
				if let Ok(Some(logs)) = &response {
					let (events, proof_height) =
						events::get_ibc_events_from_logs(logs.value.logs.clone());
					// log::info!("These are events {:?} ", events);
					// log::info!("Total {:?} events", events.len());
					let mut broke = false;
					events.iter().for_each(|event| {
						log::info!("Came into ibc events");
						let height = Height::new(1, proof_height);
						let converted_event =
							events::convert_new_event_to_old(event.clone(), height);
						if let Some(event) = converted_event {
							log::info!("Sending message");
							tx.send(event.clone()).unwrap_or_else(|_| {
								log::info!("Broke");
								broke = true;
							});
						}
					});
					if broke {
						break;
					}
				} else {
					log::warn!("ibc_events stream ended");
				}
			}
		});
		let streams = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
		Box::pin(streams)
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
		include_proof: bool,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let consensus_state = if cfg!(feature = "no_indexer") {
			use ibc_proto_new::Protobuf;
			let storage = self.get_ibc_storage().await?;
			let revision_height = consensus_height.revision_height;
			let revision_number = consensus_height.revision_number;
			log::info!("query_client_consensus before search clients");
			let client_store = storage
				.clients
				.iter()
				.find(|&client| client.client_id.as_str() == client_id.as_str())
				.ok_or(
					"Client not found with the given client id while querying client consensus"
						.to_owned(),
				)?;
			log::info!("query_client_consensus before get cs states");
			let serialized_consensus_state = client_store
				.consensus_states
				.get(&ibc_core_client_types::Height::new(revision_number, revision_height).unwrap())
				.ok_or(Error::Custom("No value at given key".to_owned()))?;
			log::info!("query_client_consensus before convert cs states");
			serialized_consensus_state
				.state()
				.map_err(|_| {
					Error::Custom(
						"Could not
			deserialize consensus state"
							.to_owned(),
					)
				})
				.unwrap()
		} else {
			use prost::Message;
			let sql = r#"
				SELECT consensus_state
				FROM consensus_states
				WHERE client_id = $1
				  AND revision_number = $2
				  AND revision_height = $3
				  AND (
					  $4 > client_update_revision_number
					  OR ($4 = client_update_revision_number AND $5 >= client_update_revision_height)
				  )
				LIMIT 1
			"#;

			let query = sqlx::query(sql)
				.bind(client_id.as_str())
				.bind(consensus_height.revision_number as i64)
				.bind(consensus_height.revision_height as i64)
				.bind(at.revision_number as i64)
				.bind(at.revision_height as i64);

			let rows = self.query(query).await?;

			if let Some(row) = rows.into_iter().next() {
				let consensus_state_bytes: Option<Vec<u8>> = row.get("consensus_state");

				if let Some(bytes) = consensus_state_bytes {
					use prost_12::Message;
					let any = ibc_proto_new::google::protobuf::Any::decode(bytes.as_slice())
						.map_err(|e| Error::from(e))?;
					let consensus_state =
						solana_ibc::consensus_state::AnyConsensusState::try_from(any.clone())
							.map_err(|e| {
								Error::Custom("Failed to decode Consensus State".to_string())
							});
					consensus_state.unwrap()
				} else {
					return Err(Self::Error::Custom("Consensus state is NULL".to_string()))
				}
			} else {
				return Err(Self::Error::Custom("No consensus state found for the given client_id and consensus_height at the specified block".to_string()))
			}
		};

		log::info!("query_client_consensus before encode");
		let cs_state = convert_new_consensus_state_to_old(consensus_state.clone());
		let inner_any = consensus_state.clone().encode_vec();
		log::info!("this is consensus state {:?}", consensus_state);
		log::info!("This is inner any consensus state {:?}", inner_any);
		let proof = if include_proof {
			let chain_account = self.get_chain_storage().await?;
			let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
			let block_header = if !self.common_state.handshake_completed && at_height {
				log::info!("Fetching previous block header");
				events::get_header_from_height(
					self.rpc_client(),
					self.solana_ibc_program_id,
					at.revision_height,
				)
				.await
				.expect(&format!("No block header found for height {:?}", at.revision_height))
			} else {
				log::info!("Fetching latest header");
				chain_account.head().unwrap().clone()
			};

			let new_client_id =
				ibc_core_host_types::identifiers::ClientId::from_str(client_id.as_str()).unwrap();
			log::info!("query_client_consensus before trie key");
			let consensus_state_trie_key = TrieKey::for_consensus_state(
				ClientIdx::try_from(new_client_id).unwrap(),
				ibc_core_client_types::Height::new(
					consensus_height.revision_number,
					consensus_height.revision_height,
				)
				.unwrap(),
			);
			log::info!("query_client_consensus before prove trie");
			let (val, consensus_state_proof) = trie
				.prove(&consensus_state_trie_key)
				.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;

			log::info!(
				"proof {:?} state root {:?}, trie key {:?} and value {:?} and trie root {:?}",
				consensus_state_proof,
				block_header.state_root,
				consensus_state_trie_key,
				val,
				trie.hash()
			);
			let result = consensus_state_proof.verify(
				&block_header.state_root,
				&consensus_state_trie_key,
				val.as_ref(),
			);
			log::info!("Result {}", result);
			borsh::to_vec(&(block_header, &consensus_state_proof)).unwrap()
		} else {
			vec![]
		};
		Ok(QueryConsensusStateResponse {
			consensus_state: Some(cs_state.into()),
			proof,
			proof_height: Some(at.into()),
		})
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		log::info!("Quering solana client state at height {:?} {:?}", at, client_id);
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let storage = self.get_ibc_storage().await?;
		let new_client_id =
			ibc_core_host_types::identifiers::ClientId::from_str(client_id.as_str()).unwrap();
		let client_state_trie_key =
			TrieKey::for_client_state(ClientIdx::try_from(new_client_id).unwrap());
		let (val, client_state_proof) = trie
			.prove(&client_state_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let client_state = events::get_client_state_at_height(
			self.rpc_client(),
			client_id.clone(),
			self.solana_ibc_program_id,
			at.revision_height,
		)
		.await
		.unwrap_or_else(|| {
			log::info!("Fetching latest client state");
			let client_store = storage
				.clients
				.iter()
				.find(|&client| client.client_id.as_str() == client_id.as_str())
				.expect("Client not found with the given client id while querying client state");
			let serialized_client_state = &client_store.client_state;
			serialized_client_state
				.get()
				.map_err(|_| {
					Error::Custom(
						"Could not
deserialize client state"
							.to_owned(),
					)
				})
				.unwrap()
		});
		// let inner_any = client_state.clone().encode_vec();
		log::info!("this is client state {:?}", client_state);
		// log::info!("This is inner any client state {:?}", inner_any);
		let any_client_state = convert_new_client_state_to_old(client_state);
		let chain_account = self.get_chain_storage().await?;
		let block_header = if !self.common_state.handshake_completed && at_height {
			log::info!("Fetching previous block header");
			events::get_header_from_height(
				self.rpc_client(),
				self.solana_ibc_program_id,
				at.revision_height,
			)
			.await
			.expect(&format!("No block header found for height {:?}", at.revision_height))
		} else {
			log::info!("Fetching latest header");
			chain_account.head().unwrap().clone()
		};
		// let block_header = chain_account.head().unwrap().clone();
		log::info!(
			"proof {:?} state root {:?}, trie key {:?} and value {:?}",
			client_state_proof,
			block_header.state_root,
			client_state_trie_key,
			val
		);
		let state_any = any_client_state.clone().encode_vec();
		log::info!(
			"This is any client state {:?} and any client state {:?}",
			state_any,
			Into::<Any>::into(any_client_state.clone())
		);
		let hash = cf_guest::digest_with_client_id(&client_id, state_any.as_slice());
		log::info!("hash proof {:?} and derived hash {}", val, hash);
		let result = client_state_proof.verify(
			&block_header.state_root,
			&client_state_trie_key,
			val.as_ref(),
		);
		log::info!("Result {}", result);
		Ok(QueryClientStateResponse {
			client_state: Some(any_client_state.into()),
			proof: borsh::to_vec(&(block_header, &client_state_proof)).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		use ibc_proto_new::Protobuf;
		let chain_account = self.get_chain_storage().await?;
		let proof_height =
			if u64::from(chain_account.head().unwrap().block_height) == at.revision_height {
				log::info!("Using existing proof height {}", at.revision_height);
				at.revision_height
			} else {
				log::info!("Using incremented proof height {}", at.revision_height + 1);
				at.revision_height
			};
		let (trie, at_height) = self.get_trie(proof_height, true).await?;
		let storage = self.get_ibc_storage().await?;
		let connection_idx = ConnectionIdx::try_from(
			ibc_core_host_types::identifiers::ConnectionId::from_str(connection_id.as_str())
				.unwrap(),
		)
		.unwrap();
		log::info!(
			"This is connection ID {:?} and index {:?} while querying connection end",
			connection_id,
			connection_idx
		);
		let connection_end_trie_key = TrieKey::for_connection(connection_idx);
		log::info!("This is connection end trie key {:?}", connection_end_trie_key);
		let (val, connection_end_proof) = trie
			.prove(&connection_end_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		log::info!("This is serialized connection {:?}", storage.connections);
		let serialized_connection_end =
			storage.connections.get(usize::from(connection_idx)).ok_or(
				"Connection not found with the given
		client id"
					.to_owned(),
			)?;
		let inner_connection_end = Serialised::get(serialized_connection_end)
			.map_err(|_| {
				Error::Custom(
					"Could not
	deserialize connection end"
						.to_owned(),
				)
			})
			.unwrap();
		log::info!("This is new connection end {:?}", inner_connection_end);
		log::info!("Borsh serialized connection end {:?}", borsh::to_vec(&inner_connection_end));
		log::info!("This is in any {:?}", inner_connection_end.clone().encode_vec());
		log::info!(
			"What i get hashed value {:?}",
			CryptoHash::digest(inner_connection_end.clone().encode_vec().as_slice())
		);
		log::info!("This is the hashed value {:?}", val);
		let inner_any = inner_connection_end.clone().encode_vec();
		let inner_counterparty = inner_connection_end.counterparty();
		let connection_end = ConnectionEnd {
			client_id: inner_connection_end.client_id().to_string(),
			versions: inner_connection_end
				.versions()
				.to_vec()
				.iter()
				.map(|version| {
					let raw_version =
						ibc_proto_new::ibc::core::connection::v1::Version::from(version.clone());
					Version { identifier: raw_version.identifier, features: raw_version.features }
				})
				.collect(),
			state: inner_connection_end.state.into(),
			counterparty: Some(ConnCounterparty {
				client_id: inner_counterparty.client_id().to_string(),
				connection_id: inner_counterparty
					.connection_id
					.as_ref()
					.map_or_else(|| "".to_string(), |v| v.as_str().to_string()),
				prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
					key_prefix: inner_counterparty.prefix().clone().into_vec(),
				}),
			}),
			delay_period: inner_connection_end.delay_period().as_nanos() as u64,
		};
		log::info!("This is after connection end {:?}", connection_end);
		log::info!(
			"Proof height {:?}, Latest Height {:?}",
			proof_height,
			chain_account.head().unwrap().block_height
		);
		log::info!("at height {:?}", at_height);
		let block_header = if !self.common_state.handshake_completed && at_height {
			log::info!("Fetching previous block header");
			events::get_header_from_height(
				self.rpc_client(),
				self.solana_ibc_program_id,
				proof_height,
			)
			.await
			.expect(&format!("No block header found for height {:?}", at.revision_height))
		} else {
			log::info!("Fetching latest header");
			chain_account.head().unwrap().clone()
		};
		// let block_header = chain_account.head().unwrap().clone();
		log::info!(
			"proof {:?} state root {:?}, trie key {:?} and value {:?}",
			connection_end_proof,
			block_header.state_root,
			connection_end_trie_key,
			val
		);
		log::info!("Block header {:?}", block_header);
		let result = connection_end_proof.verify(
			&block_header.state_root,
			&connection_end_trie_key,
			val.as_ref(),
		);
		log::info!("Result {}", result);
		log::info!("connection end {:?}", connection_end);
		let mut proof = borsh::to_vec(&(block_header.clone(), &connection_end_proof)).unwrap();
		log::info!("This is proof {:?}", proof);
		let (header, proof): (BlockHeader, sealable_trie::proof::Proof) =
			borsh::BorshDeserialize::deserialize_reader(&mut &proof[..]).unwrap();
		log::info!("This is deserialized proof {:?}", proof);
		Ok(QueryConnectionResponse {
			connection: Some(connection_end),
			proof: borsh::to_vec(&(block_header, &connection_end_proof)).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let storage = self.get_ibc_storage().await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let channel_end_path =
			PortChannelPK::try_from(new_port_id.clone(), new_channel_id.clone()).unwrap();
		let channel_end_trie_key = TrieKey::for_channel_end(&channel_end_path);
		let (_, channel_end_proof) = trie
			.prove(&channel_end_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let inner_channel_end = storage
			.port_channel
			.deref()
			.get(&channel_end_path)
			.ok_or(Error::Custom("No value at given key".to_owned()))?
			.channel_end()
			.unwrap()
			.ok_or(Error::Custom("Channel end not found".to_owned()))?;
		let inner_counterparty = inner_channel_end.counterparty();
		let state = match inner_channel_end.state {
			ibc_core_channel_types::channel::State::Uninitialized => 0,
			ibc_core_channel_types::channel::State::Init => 1,
			ibc_core_channel_types::channel::State::TryOpen => 2,
			ibc_core_channel_types::channel::State::Open => 3,
			ibc_core_channel_types::channel::State::Closed => 4,
		};
		let ordering = match inner_channel_end.ordering {
			ibc_core_channel_types::channel::Order::None => 0,
			ibc_core_channel_types::channel::Order::Unordered => 1,
			ibc_core_channel_types::channel::Order::Ordered => 2,
		};
		let channel_end = Channel {
			state,
			ordering,
			counterparty: Some(ChanCounterparty {
				port_id: inner_counterparty.port_id.to_string(),
				channel_id: if inner_counterparty.channel_id.is_none() {
					"".to_owned()
				} else {
					inner_counterparty.channel_id.clone().unwrap().to_string()
				},
			}),
			connection_hops: inner_channel_end
				.connection_hops
				.iter()
				.map(|connection_id| connection_id.to_string())
				.collect(),
			version: inner_channel_end.version.to_string(),
		};
		let block_header = if !self.common_state.handshake_completed && at_height {
			log::info!("Fetching previous block header");
			events::get_header_from_height(
				self.rpc_client(),
				self.solana_ibc_program_id,
				at.revision_height,
			)
			.await
			.expect(&format!("No block header found for height {:?}", at.revision_height))
		} else {
			log::info!("Fetching latest block header");
			let chain_account = self.get_chain_storage().await?;
			chain_account.head().unwrap().clone()
		};
		Ok(QueryChannelResponse {
			channel: Some(channel_end),
			proof: borsh::to_vec(&(block_header, &channel_end_proof)).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		log::info!("Querying proof at {:?}", at.revision_height);
		log::info!("This is the bytes for keys {:?} ", keys);
		let key_str = String::from_utf8(keys[0].clone())
			.map_err(|_| Error::Custom("Invalid key".to_owned()))?;
		log::info!("This is the keys in string{:?}", key_str);
		let split_keys = key_str.split("/").collect::<Vec<&str>>();
		let trie_key = match split_keys[0] {
			"nextSequenceRecv" => {
				let port_str = split_keys[2];
				let channel_str = split_keys[4];
				let new_port_id =
					ibc_core_host_types::identifiers::PortId::from_str(port_str).unwrap();
				let new_channel_id =
					ibc_core_host_types::identifiers::ChannelId::from_str(channel_str).unwrap();
				let next_seq_recv_path =
					PortChannelPK::try_from(new_port_id.clone(), new_channel_id.clone()).unwrap();
				TrieKey::for_next_sequence(&next_seq_recv_path)
			},
			"receipts" => {
				let port_str = split_keys[2];
				let channel_str = split_keys[4];
				let sequence_str = split_keys[6];
				let new_port_id =
					ibc_core_host_types::identifiers::PortId::from_str(port_str).unwrap();
				let new_channel_id =
					ibc_core_host_types::identifiers::ChannelId::from_str(channel_str).unwrap();
				let new_seq =
					ibc_core_host_types::identifiers::Sequence::from_str(sequence_str).unwrap();
				let packet_receipt_path = ibc_core_host_types::path::ReceiptPath {
					port_id: new_port_id,
					channel_id: new_channel_id,
					sequence: new_seq,
				};
				TrieKey::try_from(&packet_receipt_path).unwrap()
			},
			"commitments" => {
				log::info!("Entered commitments");
				let port_str = split_keys[2];
				let channel_str = split_keys[4];
				let sequence_str = split_keys[6];
				let new_port_id =
					ibc_core_host_types::identifiers::PortId::from_str(port_str).unwrap();
				let new_channel_id =
					ibc_core_host_types::identifiers::ChannelId::from_str(channel_str).unwrap();
				let new_seq =
					ibc_core_host_types::identifiers::Sequence::from_str(sequence_str).unwrap();
				let packet_commitment_path = ibc_core_host_types::path::CommitmentPath {
					port_id: new_port_id,
					channel_id: new_channel_id,
					sequence: new_seq,
				};
				TrieKey::try_from(&packet_commitment_path).unwrap()
			},
			"acks" => {
				let port_str = split_keys[2];
				let channel_str = split_keys[4];
				let sequence_str = split_keys[6];
				let new_port_id =
					ibc_core_host_types::identifiers::PortId::from_str(port_str).unwrap();
				let new_channel_id =
					ibc_core_host_types::identifiers::ChannelId::from_str(channel_str).unwrap();
				let new_seq =
					ibc_core_host_types::identifiers::Sequence::from_str(sequence_str).unwrap();
				let packet_ack_path = ibc_core_host_types::path::AckPath {
					port_id: new_port_id,
					channel_id: new_channel_id,
					sequence: new_seq,
				};
				TrieKey::try_from(&packet_ack_path).unwrap()
			},
			"channelEnds" => {
				let port_str = split_keys[2];
				let channel_str = split_keys[4];
				let new_port_id =
					ibc_core_host_types::identifiers::PortId::from_str(port_str).unwrap();
				let new_channel_id =
					ibc_core_host_types::identifiers::ChannelId::from_str(channel_str).unwrap();
				let channel_end_path =
					PortChannelPK::try_from(new_port_id.clone(), new_channel_id.clone()).unwrap();
				TrieKey::for_channel_end(&channel_end_path)
			},
			_ => {
				log::error!("invalid key in proof query proof");
				return Err(Error::Custom("Invalid key".to_owned()));
			},
		};
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let (val, proof) = trie
			.prove(&trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		log::info!("This is proof {:?}", proof);
		let chain_account = self.get_chain_storage().await?;
		let block_header_at_height = if at_height {
			log::info!("Fetching block header at height");
			events::get_header_from_height(
				self.rpc_client(),
				self.solana_ibc_program_id,
				at.revision_height,
			)
			.await
			.expect(&format!("No block header found for height {:?}", at.revision_height))
		} else {
			log::info!("Block header not found at height, so fetching latest height");
			chain_account.head().unwrap().clone()
		};
		let result = proof.verify(&block_header_at_height.state_root, &trie_key, val.as_ref());
		log::info!("latest Block header height {}", block_header_at_height.block_height);
		log::info!("state root {:?}", &block_header_at_height.state_root);
		log::info!("trie root {:?}", &trie.hash());
		log::info!("trie key {:?}", trie_key);
		log::info!("Value {:?}", val.as_ref());
		log::info!("This is value in proof verify {:?}", val);
		log::info!("This is result of time out packet proof verify lts {:?}", result);
		Ok(borsh::to_vec(&(block_header_at_height.clone(), &proof)).unwrap())
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_core_host_types::identifiers::Sequence::from(seq);
		let packet_commitment_path = ibc_core_host_types::path::CommitmentPath {
			port_id: new_port_id,
			channel_id: new_channel_id,
			sequence: new_seq,
		};
		let packet_commitment_trie_key = TrieKey::try_from(&packet_commitment_path).unwrap();
		let (packet_commitment, packet_commitment_proof) = trie
			.prove(&packet_commitment_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let commitment =
			packet_commitment.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let block_header = events::get_header_from_height(
			self.rpc_client(),
			self.solana_ibc_program_id,
			at.revision_height,
		)
		.await
		.expect(&format!("No block header found for height {:?}", at.revision_height));
		log::info!("This is packet commitment {:?}", commitment.0.to_vec());
		Ok(QueryPacketCommitmentResponse {
			commitment: commitment.0.to_vec(),
			proof: borsh::to_vec(&(block_header, &packet_commitment_proof)).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_core_host_types::identifiers::Sequence::from(seq);
		let packet_ack_path = ibc_core_host_types::path::AckPath {
			port_id: new_port_id,
			channel_id: new_channel_id,
			sequence: new_seq,
		};
		let packet_ack_trie_key = TrieKey::try_from(&packet_ack_path).unwrap();
		let (packet_ack, packet_ack_proof) = trie
			.prove(&packet_ack_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let ack = packet_ack.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let (packet_ack, packet_ack_proof) = trie
			.prove(&packet_ack_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let ack = packet_ack.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let block_header = events::get_header_from_height(
			self.rpc_client(),
			self.solana_ibc_program_id,
			at.revision_height,
		)
		.await
		.expect(&format!("No block header found for height {:?}", at.revision_height));
		log::info!("This is packet ack {:?}", ack.0.to_vec());
		Ok(QueryPacketAcknowledgementResponse {
			acknowledgement: ack.0.to_vec(),
			proof: borsh::to_vec(&(block_header, packet_ack_proof)).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let storage = self.get_ibc_storage().await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let next_sequence_recv_path = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let next_sequence_recv_trie_key = TrieKey::for_next_sequence(&next_sequence_recv_path);
		let (_, next_sequence_recv_proof) = trie
			.prove(&next_sequence_recv_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let next_seq = &storage
			.port_channel
			.deref()
			.get(&next_sequence_recv_path)
			.ok_or(Error::Custom("No value at given key".to_owned()))?
			.next_sequence;
		let next_seq_recv = next_seq
			.get(SequenceKind::Recv)
			.ok_or(Error::Custom("No value set for the next sequence receive".to_owned()))?;
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive: next_seq_recv.into(),
			proof: borsh::to_vec(&next_sequence_recv_proof).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, true).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_core_host_types::identifiers::Sequence::from(seq);
		let packet_recv_path = ibc_core_host_types::path::ReceiptPath {
			port_id: new_port_id,
			channel_id: new_channel_id,
			sequence: new_seq,
		};
		let packet_recv_trie_key = TrieKey::try_from(&packet_recv_path).unwrap();
		let (packet_recv, packet_recv_proof) = trie
			.prove(&packet_recv_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		Ok(QueryPacketReceiptResponse {
			received: packet_recv.is_some(),
			proof: borsh::to_vec(&packet_recv_proof).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn latest_height_and_timestamp(
		&self,
	) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
		let chain = self.get_chain_storage().await?;
		let block_header = chain.head().unwrap();
		let height = block_header.block_height.into();
		let timestamp_ns: u64 = block_header.timestamp_ns.into();
		log::info!("THis is the timestamp and height of solana {:?} {:?}", timestamp_ns, height);
		Ok((Height::new(1, height), Timestamp::from_nanoseconds(timestamp_ns).unwrap()))
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, false).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Commitment, trie_comp);
		let sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(sequences)
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, false).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Ack, trie_comp);
		let sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(sequences)
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		log::info!("----------Unreceived packets seqs on solana {:?} ", seqs);
		let (trie, at_height) = self.get_trie(at.revision_height, false).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Receipt, trie_comp);
		let packet_receipt_sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(seqs
			.iter()
			.flat_map(|&seq| {
				match packet_receipt_sequences.iter().find(|&&receipt_seq| receipt_seq == seq) {
					Some(_) => None,
					None => Some(seq),
				}
			})
			.collect())
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let (trie, at_height) = self.get_trie(at.revision_height, false).await?;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Commitment, trie_comp);
		let packet_receipt_sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(seqs
			.iter()
			.flat_map(|&seq| {
				match packet_receipt_sequences.iter().find(|&&receipt_seq| receipt_seq == seq) {
					Some(_) => Some(seq),
					None => None,
				}
			})
			.collect())
	}

	fn channel_whitelist(
		&self,
	) -> std::collections::HashSet<(
		ibc::core::ics24_host::identifier::ChannelId,
		ibc::core::ics24_host::identifier::PortId,
	)> {
		self.channel_whitelist.lock().unwrap().clone()
	}

	/// We just return all the channels since there doesnt seem to be any kind of relation between
	/// connection ID and channels.
	async fn query_connection_channels(
		&self,
		at: Height,
		_connection_id: &ConnectionId,
	) -> Result<ibc_proto::ibc::core::channel::v1::QueryChannelsResponse, Self::Error> {
		let storage = self.get_ibc_storage().await?;
		let channels: Vec<IdentifiedChannel> = storage
			.port_channel
			.deref()
			.into_iter()
			.map(|(key, value)| {
				let channel = value.channel_end().unwrap().unwrap();
				let state = match channel.state {
					ibc_core_channel_types::channel::State::Uninitialized => 0,
					ibc_core_channel_types::channel::State::Init => 1,
					ibc_core_channel_types::channel::State::TryOpen => 2,
					ibc_core_channel_types::channel::State::Open => 3,
					ibc_core_channel_types::channel::State::Closed => 4,
				};
				let ordering = match channel.ordering {
					ibc_core_channel_types::channel::Order::None => 0,
					ibc_core_channel_types::channel::Order::Unordered => 1,
					ibc_core_channel_types::channel::Order::Ordered => 2,
				};
				IdentifiedChannel {
					state,
					ordering,
					counterparty: Some(ChanCounterparty {
						port_id: channel.counterparty().port_id.to_string(),
						channel_id: "".to_string(),
					}),
					connection_hops: channel
						.connection_hops
						.iter()
						.map(|connection_id| connection_id.to_string())
						.collect(),
					version: channel.version.to_string(),
					port_id: key.port_id().to_string(),
					channel_id: key.channel_id().to_string(),
				}
			})
			.collect();
		Ok(ibc_proto::ibc::core::channel::v1::QueryChannelsResponse {
			channels,
			pagination: None,
			height: Some(at.into()),
		})
	}

	async fn query_send_packets(
		&self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		log::info!("Inside query send packets");
		let rpc_client = self.rpc_client();
		if seqs.is_empty() {
			return Ok(Vec::new());
		}
		let mut total_packets = Vec::new();
		let mut before_hash = None;
		let maximum_sequence_number = seqs.iter().max().unwrap();
		let mut is_sequence_greater = false;
		let mut is_maximum_seq_found = false;
		while total_packets.len() < seqs.len() {
			let (transactions, last_searched_hash) = events::get_previous_transactions(
				&rpc_client,
				self.solana_ibc_program_id,
				before_hash,
				SearchIn::IBC,
			)
			.await?;
			if transactions.is_empty() {
				log::warn!("No more transactions found");
				break;
			}

			before_hash = Some(anchor_client::solana_sdk::signature::Signature::from_str(
				&last_searched_hash,
			)?);
			let send_packet_events: Vec<_> =
				transactions
					.iter()
					.filter_map(|tx| {
						let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
					solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
					_ => Vec::new(),
				};
						let (events, _proof_height) = events::get_events_from_logs(logs.clone());
						let mut send_packet = None;
						for event in events {
							send_packet = match event {
								solana_ibc::events::Event::IbcEvent(event) => match event {
									ibc_core_handler_types::events::IbcEvent::SendPacket(
										packet,
									) => {
										if packet.chan_id_on_a().as_str() == &channel_id.to_string() &&
											packet.port_id_on_a().as_str() == port_id.as_str() &&
											seqs.iter()
												.find(|&&seq| packet.seq_on_a().value() == seq)
												.is_some()
										{
											if *maximum_sequence_number == packet.seq_on_a().value()
											{
												is_maximum_seq_found = true;
											}
											log::info!(
												"These are logs for send packet transaction {:?}",
												logs
											);
											let height_str = logs.iter().find_map(|log| {
										if log.starts_with("Program log: Current Block height ") {
											Some(log.strip_prefix("Program log: Current Block height ").unwrap())
										} else {
											None
										}
									}).expect("No height found while fetching send packet event");
											log::info!("This is height_str {:?}", height_str);
											let height = height_str.parse::<u64>().unwrap();
											return Some((packet.clone(), height + 1));
										}
										if *maximum_sequence_number > packet.seq_on_a().value() {
											is_sequence_greater = true;
										}
										None
									},
									_ => None,
								},
								_ => None,
							};
							if send_packet.is_some() {
								break;
							}
						}
						send_packet
					})
					.collect();
			if is_sequence_greater && !is_maximum_seq_found {
				log::info!("Sequence number found in logs is lesser than the set of sequence which we are looking for");
				return Ok(Vec::new());
			}
			let packets: Vec<_> = send_packet_events
				.iter()
				.map(|(packet, proof_height)| ibc_rpc::PacketInfo {
					height: Some(proof_height.clone()),
					sequence: packet.seq_on_a().value(),
					source_port: packet.port_id_on_a().to_string(),
					source_channel: packet.chan_id_on_a().to_string(),
					destination_port: packet.port_id_on_b().to_string(),
					destination_channel: packet.chan_id_on_b().to_string(),
					channel_order: packet.channel_ordering().to_string(),
					data: packet.packet_data().to_vec(),
					timeout_height: Height {
						revision_height: packet.timeout_height_on_b().commitment_revision_height(),
						revision_number: packet.timeout_height_on_b().commitment_revision_number(),
					}
					.into(),
					timeout_timestamp: packet.timeout_timestamp_on_b().nanoseconds(),
					ack: None,
				})
				.collect();
			total_packets.extend(packets);
		}
		log::info!("Found sent packets {:?}", total_packets);
		Ok(total_packets)
	}

	async fn query_received_packets(
		&self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		log::info!("Inside received packets");
		let rpc_client = self.rpc_client();
		if seqs.is_empty() {
			return Ok(Vec::new());
		}
		let maximum_sequence_number = seqs.iter().max().unwrap();
		let mut before_hash = None;
		let mut total_packets = Vec::new();
		while total_packets.len() < seqs.len() {
			let (transactions, last_searched_hash) = events::get_previous_transactions(
				&rpc_client,
				self.solana_ibc_program_id,
				before_hash,
				SearchIn::IBC,
			)
			.await?;
			if transactions.is_empty() {
				log::warn!("No more transactions found");
				break;
			}
			before_hash = Some(anchor_client::solana_sdk::signature::Signature::from_str(
				&last_searched_hash,
			)?);
			let mut is_sequence_greater = false;
			let mut is_maximum_seq_found = false;
			let recv_packet_events: Vec<_> = transactions
			.iter()
			.filter_map(|tx| {
				let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
					solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
					_ => Vec::new(),
				};
				let (events, proof_height) = events::get_ibc_events_from_logs(logs.clone());
				let receive_packet_event = events.iter().find(|event| {
					matches!(
						event,
						ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(_)
					)
				});
				match receive_packet_event {
					Some(e) => match e {
						ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(packet) =>
							if packet.chan_id_on_b().as_str() == &channel_id.to_string() &&
								packet.port_id_on_b().as_str() == port_id.as_str() &&
								seqs.iter()
									.find(|&&seq| packet.seq_on_a().value() == seq)
									.is_some()
							{
								if *maximum_sequence_number == packet.seq_on_a().value() {
									is_maximum_seq_found = true;
								}
								log::info!("Found receive packet");
								Some((e.clone(), proof_height + 1))
							} else {
								if *maximum_sequence_number > packet.seq_on_a().value() {
									is_sequence_greater = true;
								}
								log::info!("Receive Ids dont match expected channel id: {:?} got channel id: {:?} expect port id: {:?} got port id: {:?} expected seq: {:?} got seq: {:?}", packet.chan_id_on_b(), channel_id, packet.port_id_on_b(), port_id, seqs, packet.seq_on_a().value());
								None
							},
						_ => None,
					},
					None => None,
				}
			})
			.collect();
			if is_sequence_greater && !is_maximum_seq_found {
				log::info!("Sequence number found in logs is lesser than the set of sequence which we are looking for");
				return Ok(Vec::new());
			}
			let packets: Vec<_> = recv_packet_events
				.iter()
				.map(|(recv_packet, height)| match recv_packet {
					ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(packet) =>
						ibc_rpc::PacketInfo {
							height: Some(*height),
							sequence: packet.seq_on_a().value(),
							source_port: packet.port_id_on_a().to_string(),
							source_channel: packet.chan_id_on_a().to_string(),
							destination_port: packet.port_id_on_b().to_string(),
							destination_channel: packet.chan_id_on_b().to_string(),
							channel_order: String::from(""),
							data: packet.packet_data().to_vec(),
							timeout_height: Height {
								revision_height: packet
									.timeout_height_on_b()
									.commitment_revision_height(),
								revision_number: packet
									.timeout_height_on_b()
									.commitment_revision_number(),
							}
							.into(),
							timeout_timestamp: packet.timeout_timestamp_on_b().nanoseconds(),
							ack: Some(packet.acknowledgement().as_bytes().to_vec()),
						},
					_ => panic!("Infallible"),
				})
				.collect();
			total_packets.extend(packets);
		}
		println!("Length of receive packets {}", total_packets.len());
		Ok(total_packets)
	}

	fn expected_block_time(&self) -> Duration {
		// solana block time is roughly 400 milliseconds
		Duration::from_millis(400)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
		if cfg!(feature = "no_indexer") {
			let sql = r#"
            SELECT client_update_revision_number, client_update_revision_height, client_update_time
            FROM consensus_states
            WHERE client_id = $1
              AND revision_number = $2
              AND revision_height = $3
            ORDER BY client_update_revision_number DESC, client_update_revision_height DESC
            LIMIT 1
        "#;

			let query = sqlx::query(sql)
				.bind(client_id.as_str())
				.bind(client_height.revision_number as i64)
				.bind(client_height.revision_height as i64);

			let rows = self.query(query).await?;

			if let Some(row) = rows.into_iter().next() {
				let client_update_revision_number: i64 = row.get("client_update_revision_number");
				let client_update_revision_height: i64 = row.get("client_update_revision_height");
				let client_update_time: DateTime<Utc> = row.get("client_update_time");

				let height = Height {
					revision_number: client_update_revision_number as u64,
					revision_height: client_update_revision_height as u64,
				};

				let timestamp =
					Timestamp::from_nanoseconds(client_update_time.timestamp_nanos() as u64)
						.map_err(|e| Self::Error::from(e))?;

				Ok((height, timestamp))
			} else {
				Err(Self::Error::Custom(
					"No data found for the given client_id and client_height at the specified block"
						.to_string(),
				))
			}
		} else {
			let storage = self.get_ibc_storage().await?;
			let client_store = storage
				.clients
				.iter()
				.find(|&client| client.client_id.as_str() == client_id.as_str())
				.ok_or("Client not found with the given client id while querying client update time and
			height".to_owned())?;
			let inner_client_height = ibc_core_client_types::Height::new(
				client_height.revision_number,
				client_height.revision_height,
			)
			.unwrap();
			let height = client_store
				.consensus_states
				.deref()
				.get(&inner_client_height)
				.ok_or("No host height found with the given height".to_owned())?
				.processed_height()
				.ok_or("No height found".to_owned())?;
			let timestamp = client_store
				.consensus_states
				.deref()
				.get(&inner_client_height)
				.ok_or("No timestamp found with the given height".to_owned())?
				.processed_time()
				.ok_or("No timestamp found".to_owned())?;
			Ok((
				Height {
					revision_height: u64::from(height),
					// TODO: Use epoch
					revision_number: 1_u64,
				},
				Timestamp::from_nanoseconds(u64::from(timestamp)).unwrap(),
			))
		}
	}

	async fn query_host_consensus_state_proof(
		&self,
		client_state: &pallet_ibc::light_clients::AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		let height = client_state.latest_height();
		let (trie, at_height) = self.get_trie(height.revision_height, true).await?;
		let client_id = self.client_id();
		let new_client_id =
			ibc_core_host_types::identifiers::ClientId::from_str(client_id.as_str()).unwrap();
		let client_idx = ClientIdx::try_from(new_client_id).unwrap();
		let consensus_state_trie_key = TrieKey::for_consensus_state(
			client_idx,
			ibc_core_client_types::Height::new(height.revision_number, height.revision_height)
				.unwrap(),
		);
		let (_, host_consensus_state_proof) = trie
			.prove(&consensus_state_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		Ok(Some(borsh::to_vec(&host_consensus_state_proof).unwrap()))
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<ibc::applications::transfer::PrefixedCoin>, Self::Error> {
		let denom = &asset_id;
		// let (token_mint_key, _bump) =
		// 	Pubkey::find_program_address(&[denom.as_ref()], &self.solana_ibc_program_id);
		let token_mint_key = Pubkey::from_str(&asset_id).unwrap();
		let user_token_address =
			get_associated_token_address(&self.keybase.public_key, &token_mint_key);
		let sol_rpc_client = self.rpc_client();
		let balance = sol_rpc_client.get_token_account_balance(&user_token_address).await.unwrap();
		log::info!("IBC Balance on solana {}", balance.amount);
		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom {
				trace_path: TracePath::default(),
				base_denom: BaseDenom::from_str(denom).unwrap(),
			},
			amount: Amount::from_str(&balance.amount).unwrap(),
		}])
	}

	fn connection_prefix(&self) -> ibc::core::ics23_commitment::commitment::CommitmentPrefix {
		self.commitment_prefix.clone()
	}

	fn client_id(&self) -> ClientId {
		self.client_id
			.lock()
			.unwrap()
			.as_ref()
			.expect("Client Id should be defined")
			.clone()
	}

	fn set_client_id(&mut self, client_id: ClientId) {
		*self.client_id.lock().unwrap() = Some(client_id);
	}

	fn connection_id(&self) -> Option<ConnectionId> {
		self.connection_id.lock().unwrap().clone()
	}

	fn set_channel_whitelist(
		&mut self,
		channel_whitelist: std::collections::HashSet<(
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		)>,
	) {
		*self.channel_whitelist.lock().unwrap() = channel_whitelist;
	}

	fn add_channel_to_whitelist(
		&mut self,
		channel: (
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		),
	) {
		self.channel_whitelist.lock().unwrap().insert(channel);
	}

	fn set_connection_id(&mut self, connection_id: ConnectionId) {
		*self.connection_id.lock().unwrap() = Some(connection_id)
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		self.client_type.clone()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let rpc_client = self.rpc_client();
		let header =
			events::get_header_from_height(rpc_client, self.solana_ibc_program_id, block_number)
				.await;
		if let Some(header) = header {
			return Ok(header.timestamp_ns.into());
		} else {
			log::error!("No block header found for height {:?}", block_number);
			return Err(Error::RpcError(format!(
				"No block header found for height {:?}",
				block_number
			)));
		}
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let storage = self.get_ibc_storage().await?;
		let client_ids: Vec<ClientId> = storage
			.clients
			.iter()
			.map(|client| ClientId::from_str(&client.client_id.to_string()).unwrap())
			.collect();
		Ok(client_ids)
	}

	async fn query_channels(
		&self,
	) -> Result<
		Vec<(
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		)>,
		Self::Error,
	> {
		let storage = self.get_ibc_storage().await?;
		let channels: Vec<(ChannelId, PortId)> = storage
			.port_channel
			.deref()
			.keys()
			.map(|channel_store| {
				(
					ChannelId::from_str(&channel_store.channel_id().as_str()).unwrap(),
					PortId::from_str(&channel_store.port_id().as_str()).unwrap(),
				)
			})
			.collect();
		Ok(channels)
	}

	async fn query_connection_using_client(
		&self,
		_height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let storage = self.get_ibc_storage().await?;
		let client_id_key =
			ibc_core_host_types::identifiers::ClientId::from_str(&client_id).unwrap();
		let mut index = -1;
		let connections: Vec<IdentifiedConnection> = storage
			.connections
			.iter()
			.filter_map(|serialized_connection| {
				index += 1;
				// let serialized_connection = &storage.connections[0];
				let connection = serialized_connection.get().unwrap();
				if connection.client_id_matches(&client_id_key) {
					let versions = connection
						.versions()
						.iter()
						.map(|version| {
							let proto_version: ibc_proto_new::ibc::core::connection::v1::Version =
								version.clone().try_into().unwrap();
							Version {
								identifier: proto_version.identifier,
								features: proto_version.features,
							}
						})
						.collect();
					let counterparty = connection.counterparty();
					let connection_id = format!("{}-{}", ConnectionId::prefix(), index);
					return Some(IdentifiedConnection {
						id: connection_id,
						client_id: client_id.clone(),
						versions,
						state: i32::from(connection.state),
						counterparty: Some(ConnCounterparty {
							client_id: counterparty.client_id.to_string(),
							connection_id: counterparty
								.connection_id()
								.map_or_else(|| "".to_string(), |v| v.as_str().to_string()),
							prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
								key_prefix: counterparty.prefix.clone().into_vec(),
							}),
						}),
						delay_period: connection.delay_period().as_nanos() as u64,
					});
				};
				None
			})
			.collect();
		log::info!("querying connecting using client final");
		Ok(connections)
	}

	async fn is_update_required(
		&self,
		_latest_height: u64,
		_latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		// we never need to use LightClientSync trait in this case, because
		// all the events will be eventually submitted via `finality_notifications`
		Ok(false)
	}

	async fn initialize_client_state(
		&self,
	) -> Result<
		(pallet_ibc::light_clients::AnyClientState, pallet_ibc::light_clients::AnyConsensusState),
		Self::Error,
	> {
		let latest_height_timestamp = self.latest_height_and_timestamp().await?;
		println!("This is height on solana {:?}", latest_height_timestamp);
		let chain_account = self.get_chain_storage().await?;
		let header = chain_account.head().unwrap().clone();
		let blockhash = header.calc_hash();
		let validators = chain_account.validators().unwrap();
		let all_validators = validators
			.iter()
			.map(|validator| {
				let new_validator: Validator<pallet_ibc::light_clients::PubKey> = Validator::new(
					PubKey::from_bytes(&validator.pubkey.to_vec()).unwrap(),
					validator.stake,
				);
				new_validator
			})
			.collect();
		let epoch = Epoch::new_with(all_validators, |total| {
			let quorum = NonZeroU128::new(total.get() / 2 + 1).unwrap();
			// min_quorum_stake may be greater than total_stake so we’re not
			// using .clamp to make sure we never return value higher than
			// total_stake.
			println!("This is total {:?} and quorum {:?}", total, quorum);
			quorum.max(NonZeroU128::new(1000).unwrap()).min(total)
		})
		.unwrap();
		let client_state = cf_guest::ClientState::new(
			chain_account.genesis().unwrap(),
			header.block_height,
			640000 * 10_u64.pow(9),
			epoch.calc_commitment(),
			None,
			false,
		);
		log::info!("This is epoch id {:?}", header.epoch_id);
		let consensus_state = cf_guest_og::ConsensusState {
			block_hash: blockhash.to_vec().into(),
			timestamp_ns: header.timestamp_ns,
		};
		Ok((AnyClientState::Guest(client_state), AnyConsensusState::Guest(consensus_state.into())))
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		log::info!("This is signature {:?}", tx_id);
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client = program.async_rpc();
		let tx = sol_rpc_client
			.get_transaction(&signature, UiTransactionEncoding::Base64)
			.await
			.unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let (events, _proof_height) = events::get_ibc_events_from_logs(logs);
		log::info!("These are events {:?}", events);
		let result: Vec<&ibc_core_client_types::events::CreateClient> = events
			.iter()
			.filter_map(|event| match event {
				ibc_core_handler_types::events::IbcEvent::CreateClient(e) => Some(e),
				_ => None,
			})
			.collect();
		if result.len() != 1 {
			return Err(Error::Custom(format!(
				"Expected exactly one CreateClient event, found {}",
				result.len()
			)));
		}
		let client_id = result[0].client_id();
		Ok(ClientId::from_str(client_id.as_str()).unwrap())
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		log::info!("This is tx id {:?}", tx_id);
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client = program.async_rpc();
		let tx = sol_rpc_client
			.get_transaction(&signature, UiTransactionEncoding::Base64)
			.await
			.unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let (events, _proof_height) = events::get_ibc_events_from_logs(logs);
		log::info!("These are events {:?}", events);
		let result: Vec<&ibc_core_connection_types::events::OpenInit> = events
			.iter()
			.filter_map(|event| match event {
				ibc_core_handler_types::events::IbcEvent::OpenInitConnection(e) => Some(e),
				_ => None,
			})
			.collect();
		if result.len() != 1 {
			return Err(Error::Custom(format!(
				"Expected exactly one OpenInitConnection event, found {}",
				result.len()
			)));
		}
		let connection_id = result[0].conn_id_on_a();
		Ok(ConnectionId::from_str(connection_id.as_str()).unwrap())
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<
		(ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId),
		Self::Error,
	> {
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client = program.async_rpc();
		let tx = sol_rpc_client
			.get_transaction(&signature, UiTransactionEncoding::Json)
			.await
			.unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let (events, _proof_height) = events::get_ibc_events_from_logs(logs);
		let result: Vec<&ibc_core_channel_types::events::OpenInit> = events
			.iter()
			.filter_map(|event| match event {
				ibc_core_handler_types::events::IbcEvent::OpenInitChannel(e) => Some(e),
				_ => None,
			})
			.collect();
		if result.len() != 1 {
			return Err(Error::Custom(format!(
				"Expected exactly one OpenInitChannel event, found {}",
				result.len()
			)));
		}
		let channel_id = result[0].chan_id_on_a();
		let port_id = result[0].port_id_on_a();
		Ok((
			ChannelId::from_str(channel_id.as_str()).unwrap(),
			PortId::from_str(port_id.as_str()).unwrap(),
		))
	}

	async fn upload_wasm(&self, _wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}

	async fn fast_proof_search(
		&self,
		client_id: ClientId,
		source_delay_period: Duration,
	) -> Option<Height> {
		log::info!(
			"Fast proof search for client_id: {}, source_delay_period: {:?}",
			client_id,
			source_delay_period
		);

		// Convert source_delay_period to seconds as a float
		let source_delay_secs = source_delay_period.as_secs_f64();

		// Build the SQL query
		let sql = r#"
            SELECT revision_number, revision_height
            FROM consensus_states
            WHERE client_id = $1
              AND client_update_time <= NOW() - ($2 * INTERVAL '1 second')
            ORDER BY revision_number DESC, revision_height DESC
            LIMIT 1
        "#;

		// Prepare the query with parameter binding
		let query = sqlx::query(sql).bind(client_id.as_str()).bind(source_delay_secs);

		// Execute the query using `self.query(..)`
		let rows = self.query(query).await.ok()?;

		// Process the result
		if let Some(row) = rows.into_iter().next() {
			// Extract the revision_number and revision_height
			let revision_number: i64 = row.get("revision_number");
			let revision_height: i64 = row.get("revision_height");

			log::info!(
				"Fast proof search found matching consensus state: revision_number={}, revision_height={}",
				revision_number,
				revision_height
			);
			// Construct the Height object to return
			Some(Height {
				revision_number: revision_number.try_into().ok()?,
				revision_height: revision_height.try_into().ok()?,
			})
		} else {
			log::info!("No matching consensus state found");
			// No matching consensus state found
			None
		}
	}

	async fn should_ignore_packet(&self, packet: &PacketInfo) -> bool {
		let Ok(result) = serde_json::from_slice::<PacketData>(packet.data.as_slice()) else {
			return true;
		};
		let base_denom = result.token.denom.base_denom.as_str();
		if Pubkey::from_str(base_denom).is_err() {
			return true;
		}
		false
	}
}

impl KeyProvider for SolanaClient {
	fn account_id(&self) -> ibc::signer::Signer {
		let key_entry = &self.keybase;
		let public_key = key_entry.public_key;
		ibc::signer::Signer::from_str(&public_key.to_string()).unwrap()
	}
}

#[async_trait::async_trait]
impl MisbehaviourHandler for SolanaClient {
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		_counterparty: &C,
		_client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		Ok(())
	}
}

#[async_trait::async_trait]
impl LightClientSync for SolanaClient {
	async fn is_synced<C: Chain>(&self, _counterparty: &C) -> Result<bool, anyhow::Error> {
		Ok(true)
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
		log::info!("Fetching mandatory updates");
		let latest_height = counterparty.latest_height_and_timestamp().await?.0;
		let response = counterparty.query_client_state(latest_height, self.client_id()).await?;
		let any_client_state = response.client_state.ok_or_else(|| {
			Error::Custom("Received an empty client state from counterparty".to_string())
		})?;
		let AnyClientState::Guest(client_state) =
			AnyClientState::decode_recursive(any_client_state.clone(), |c| {
				matches!(c, AnyClientState::Guest(_))
			})
			.or_else(|| {
				log::info!("This is wasm {:?}", any_client_state);
				let wasm_client_state = AnyClientState::decode_recursive(any_client_state, |c| {
					matches!(c, AnyClientState::Wasm(_))
				})
				.unwrap();
				Some(wasm_client_state.unpack_recursive().clone())
			})
			.unwrap()
		else {
			unreachable!()
		};
		let height = client_state.0.latest_height.into();
		let (signatures, _events) = events::get_signatures_upto_height(
			self.rpc_client(),
			self.solana_ibc_program_id,
			height,
		)
		.await?;
		let chain_account = self.get_chain_storage().await?;
		let mut heights = Vec::new();
		let updates: Vec<Any> = signatures
			.iter()
			.map(|(sig, block_header, epoch)| {
				let validators = epoch.clone().unwrap().validators().to_vec();
				let all_validators: Vec<Validator<pallet_ibc::light_clients::PubKey>> = validators
					.iter()
					.map(|validator| {
						let new_validator: Validator<pallet_ibc::light_clients::PubKey> =
							Validator::new(
								PubKey::from_bytes(&validator.pubkey.to_vec()).unwrap(),
								validator.stake,
							);
						new_validator
					})
					.collect();
				let final_signatures: Vec<_> = sig
					.iter()
					.enumerate()
					.map(|(index, (validator, signature))| {
						let validator_idx = all_validators
							.iter()
							.position(|v| {
								v.pubkey ==
									PubKey::from_bytes(&validator.to_bytes().as_slice()).unwrap()
							})
							.unwrap();
						(validator_idx as u16, signature.clone())
					})
					.collect();
				let current_epoch = Epoch::new_with(all_validators, |total| {
					let quorum = NonZeroU128::new(total.get() / 2 + 1).unwrap();
					// min_quorum_stake may be greater than total_stake so we’re not
					// using .clamp to make sure we never return value higher than
					// total_stake.
					quorum.max(NonZeroU128::new(1000).unwrap()).min(total)
				})
				.unwrap();
				let guest_header = cf_guest_og::Header {
					genesis_hash: chain_account.genesis().unwrap().clone(),
					block_hash: block_header.calc_hash(),
					block_header: block_header.clone(),
					epoch_commitment: current_epoch.calc_commitment(),
					epoch: current_epoch,
					signatures: final_signatures,
				};
				let msg = MsgUpdateAnyClient::<LocalClientTypes> {
					client_id: self.client_id(),
					client_message: AnyClientMessage::Guest(guest_header.into()),
					signer: counterparty.account_id(),
				};
				let value = msg.clone().encode_vec();
				heights.push(IbcEvent::NewBlock(NewBlock::new(Height::new(
					1,
					block_header.block_height.into(),
				))));
				Any { type_url: msg.type_url(), value }
			})
			.collect();

		Ok((updates, heights))
	}
}

#[async_trait::async_trait]
impl Chain for SolanaClient {
	fn name(&self) -> &str {
		&self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_tx_size as u64
	}

	async fn estimate_weight(&self, _msg: Vec<Any>) -> Result<u64, Self::Error> {
		Ok(0)
	}

	async fn finality_notifications(
		&self,
	) -> Result<
		Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>>,
		Error,
	> {
		let (tx, rx) = unbounded_channel();
		let ws_url = self.ws_url.clone();
		let program_id = self.solana_ibc_program_id.clone();
		let this = self.clone();
		tokio::spawn(async move {
			let client = PubsubClient::new(&ws_url).await.unwrap();
			let (mut logs_subscription, receiver) = client
				.logs_subscribe(
					RpcTransactionLogsFilter::Mentions(vec![program_id.to_string()]),
					RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::finalized()) },
				)
				.await
				.unwrap();

			loop {
				let response = timeout(Duration::from_secs(10), logs_subscription.next()).await;
				if let Ok(Some(logs)) = &response {
					let (events, _proof_height) =
						events::get_events_from_logs(logs.clone().value.logs);
					let finality_events: Vec<&solana_ibc::events::BlockFinalised> = events
						.iter()
						.filter_map(|event| match event {
							Event::BlockFinalised(e) => Some(e),
							_ => None,
						})
						.collect();
					// Only one finality event is emitted in a transaction
					if !finality_events.is_empty() {
						log::info!("Found finality event");
						let mut broke = false;
						assert_eq!(finality_events.len(), 1);
						let finality_event = finality_events[0].clone();
						let finality_event = FinalityEvent::Guest {
							blockhash: finality_event.block_hash,
							block_height: u64::from(finality_event.block_height),
						};
						let _ = tx.send(finality_event).map_err(|_| broke = true);
						if broke {
							break;
						}
					}
				} else {
					log::trace!("No new events found, returning latest");
					if let Ok(chain_storage) = this.get_chain_storage().await {
						let latest_height = chain_storage.head().unwrap().block_height;
						let latest_finality_event = FinalityEvent::Guest {
							blockhash: Default::default(),
							block_height: latest_height.into(),
						};

						let broke = tx.send(latest_finality_event).is_err();
						if broke {
							break;
						}
					}
				}
			}
		});

		let streams = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
		Ok(Box::pin(streams))
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Error> {
		let keypair = self.keybase.keypair();
		log::info!("submitting tx now, {} with messages {}", keypair.pubkey(), messages.len());
		log::info!("Messages {:?}", messages);
		let authority = Arc::new(keypair);
		let program = self.program();

		if *self.is_transaction_processing.lock().unwrap() {
			log::info!("Waiting for other thread to complete");
			sleep(Duration::from_millis(500))
		}

		*self.is_transaction_processing.lock().unwrap() = true;

		// Build, sign, and send program instruction
		let mut signature = String::new();
		let rpc = program.async_rpc();

		let mut all_transactions = Vec::new();
		let mut index = -1;

		let mut messages_indeed = vec![];

		for message in messages {
			let storage = self.get_ibc_storage().await?;
			let my_message = Ics26Envelope::<LocalClientTypes>::try_from(message.clone()).unwrap();
			let new_messages = convert_old_msgs_to_new(vec![my_message]);
			let message = new_messages[0].clone();
			let mut instruction_data =
				anchor_lang::InstructionData::data(&solana_ibc::instruction::Deliver {
					message: message.clone(),
				});
			let instruction_len = instruction_data.len() as u32;
			instruction_data.splice(..0, instruction_len.to_le_bytes());

			messages_indeed.push(message.clone());

			let balance = program.async_rpc().get_balance(&authority.pubkey()).await.unwrap();
			log::info!("This is balance {}", balance);
			log::info!("This is start of payload ---------------------------------");
			log::info!("{:?}", message);
			log::info!("This is end of payload ----------------------------------");

			let max_tries = 5;

			let (mut chunks, chunk_account, _) = write::instruction::WriteIter::new(
				&self.write_program_id,
				authority.pubkey(),
				WRITE_ACCOUNT_SEED,
				instruction_data.clone(),
			)
			.unwrap();

			chunks.chunk_size = core::num::NonZeroU16::new(800).unwrap();

			let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(30_000);
			let compute_unit_price_ix = ComputeBudgetInstruction::set_compute_unit_price(50_000);
			// let compute_unit_price_ix =
			// 	ComputeBudgetInstruction::set_compute_unit_price(10_000_001);

			let chunking_transactions: Vec<Transaction> = chunks
				.map(|ix| {
					Transaction::new_with_payer(
						&[compute_budget_ix.clone(), compute_unit_price_ix.clone(), ix],
						Some(&authority.pubkey()),
					)
				})
				.collect();

			// for instruction in &mut chunks {
			// 	let transaction = Transaction::new_signed_with_payer(
			// 		&[instruction],
			// 		Some(&authority.pubkey()),
			// 		&[&*authority],
			// 		blockhash,
			// 	);
			// 	let sig =
			// rpc.send_and_confirm_transaction_with_spinner(&transaction).await.unwrap(); 	//
			// futures. push(x); 	println!("  Signature {sig}");
			// }

			let (signature_chunking_transactions, further_transactions) =
				// if let MsgEnvelope::Client(ClientMsg::UpdateClient(e)) = message {
				// 	self.send_deliver(
				// 		DeliverIxType::UpdateClient {
				// 			client_message: e.client_message,
				// 			client_id: e.client_id,
				// 		},
				// 		chunk_account,
				// 		max_tries,
				// 	)
				// 	.await?
				// // msg!("Packet Update Signature {:?}", signature);
				// } else 
				if let MsgEnvelope::Packet(PacketMsg::Recv(e)) = message {
					let packet_data: ibc_app_transfer_types::packet::PacketData =
						serde_json::from_slice(&e.packet.data).unwrap();
					self.send_deliver(
						DeliverIxType::Recv {
							token: packet_data.token,
							port_id: e.packet.port_id_on_a,
							channel_id: e.packet.chan_id_on_a,
							receiver: packet_data.receiver.to_string(),
						},
						chunk_account,
						max_tries,
					)
					.await?
				// msg!("Packet Recv Signature {:?}", signature);
				} else if let MsgEnvelope::Packet(PacketMsg::Timeout(e)) = message {
					let packet_data: ibc_app_transfer_types::packet::PacketData =
						serde_json::from_slice(&e.packet.data).unwrap();
					self.send_deliver(
						DeliverIxType::Timeout {
							token: packet_data.token,
							port_id: e.packet.port_id_on_a,
							channel_id: e.packet.chan_id_on_a,
							sender_account: packet_data.sender.to_string(),
						},
						chunk_account,
						max_tries,
					)
					.await?
				// msg!("Packet Timeout Signature {:?}", signature);
				} else if let MsgEnvelope::Packet(PacketMsg::Ack(e)) = message {
					let packet_data: ibc_app_transfer_types::packet::PacketData =
						serde_json::from_slice(&e.packet.data).unwrap();
					self.send_deliver(
						DeliverIxType::Timeout {
							token: packet_data.token,
							port_id: e.packet.port_id_on_a,
							channel_id: e.packet.chan_id_on_a,
							sender_account: packet_data.sender.to_string(),
						},
						chunk_account,
						max_tries,
					)
					.await?
				// msg!("Packet Acknowledgement Signature {:?}", signature);
				} else {
					// signature =
					self.send_deliver(DeliverIxType::Normal, chunk_account, max_tries).await?
					// msg!("Packet Normal Signature {:?}", signature);
				};
			// transactions.extend(further_transactions);
			// log::info!("Chunking tx {:?}", chunking_transactions);
			// log::info!("Complete tx {:?}", further_transactions);
			// let message_chunking_futures =
			// 	further_transactions.iter().map(|tx| rpc.send_and_confirm_transaction(tx));
			// if chunking_transactions.is_empty() {
			// 	all_transactions.extend(further_transactions);
			// } else {
			// 	let message_chunking_futures =
			// 		chunking_transactions.iter().map(|tx| rpc.send_and_confirm_transaction(tx));
			// 	let futures = join_all(message_chunking_futures).await;
			// 	for sig in futures {
			// 		println!("  Chunking Signature {:?}", sig);
			// 		signature = sig.unwrap().to_string();
			// 	}
			// 	if !signature_chunking_transactions.is_empty() {
			// 		let signature_chunking_futures = signature_chunking_transactions
			// 			.iter()
			// 			.map(|tx| rpc.send_and_confirm_transaction(tx));
			// 		let futures = join_all(signature_chunking_futures).await;
			// 		for sig in futures {
			// 			println!("  Signature chunking Signature {:?}", sig);
			// 			signature = sig.unwrap().to_string();
			// 		}
			// 	}
			// 	if !further_transactions.is_empty() {
			// 		let further_transactions_futures =
			// 			further_transactions.iter().map(|tx| rpc.send_and_confirm_transaction(tx));
			// 		let futures = join_all(further_transactions_futures).await;
			// 		for sig in futures {
			// 			println!("  Completed Signature {:?}", sig);
			// 			let blockhash = rpc.get_latest_blockhash().await.unwrap();
			// 			// Wait for finalizing the transaction
			// 			let _ = rpc
			// 				.confirm_transaction_with_spinner(
			// 					&sig.as_ref().unwrap(),
			// 					&blockhash,
			// 					CommitmentConfig::finalized(),
			// 				)
			// 				.await
			// 				.unwrap();
			// 			signature = sig.unwrap().to_string();
			// 		}
			// 	}
			// }

			let mut current_transactions = Vec::new();

			current_transactions.extend(chunking_transactions);
			current_transactions.extend(signature_chunking_transactions);
			current_transactions.extend(further_transactions);

			all_transactions.push(current_transactions);

			// let signatures = join_all(futures).await;
			// for sig in signatures {
			// 	println!("  Signature {:?}", sig);
			// 	signature = sig.unwrap().to_string();
			// }
		}

		let total_transactions_length = all_transactions.iter().fold(0, |acc, tx| acc + tx.len());
		let mut failure = false;

		match self.transaction_sender {
			TransactionSender::RPC => {
				let length = all_transactions.len();
				log::info!("Total transactions {:?}", length);
				let start_time = Instant::now();
				for transactions_iter in all_transactions {
					log::info!("Came inside all events");
					let mut tries = 0;
					let before_time = Instant::now();
					for mut transaction in transactions_iter {
						loop {
							sleep(Duration::from_secs(1));
							log::info!("Current Try: {}", tries);
							let blockhash = rpc.get_latest_blockhash().await.unwrap();
							transaction.sign(&[&*authority], blockhash);
							let sig = rpc
								.send_transaction_with_config(
									&transaction,
									RpcSendTransactionConfig {
										skip_preflight: true,
										max_retries: Some(0),
										..RpcSendTransactionConfig::default()
									},
								)
								.await;

							if let Ok(si) = sig {
								signature = si.to_string();
								// Wait for finalizing the transaction
								let mut success = false;
								// let blockhash = rpc.get_latest_blockhash().await.unwrap();
								for status_retry in 0..usize::MAX {
									match rpc.get_signature_status(&si).await.unwrap() {
										Some(Ok(_)) => {
											log::info!("  Signature {:?}", si);
											success = true;
											break;
										},
										Some(Err(e)) => {
											log::error!(
												"Error while sending the transaction {:?}",
												e
											);
											success = true;
											break;
										},
										None => {
											if !rpc
												.is_blockhash_valid(
													&blockhash,
													CommitmentConfig::processed(),
												)
												.await
												.unwrap()
											{
												// Block hash is not found by some reason
												log::error!("Blockhash not found");
												success = false;
												break;
											} else if status_retry < usize::MAX {
												// Retry twice a second
												sleep(Duration::from_millis(500));
												continue;
											}
										},
									}
								}
								if !success {
									tries += 1;
									continue;
								}
								break;
							} else {
								log::error!("Error {:?}", sig);
								tries += 1;
								continue;
							}
						}
						let after_time = Instant::now();
						let diff = after_time - before_time;
						let success_rate = 100 / (tries + 1);
						log::info!("Time taken {}", diff.as_millis());
						log::info!("Success rate {}", success_rate);
						log::info!("Failure {}", failure);
					}
				}
				let end_time = Instant::now();
				let diff = end_time - start_time;
				log::info!("Time taken for all transactions {}", diff.as_millis());
				log::info!(
					"Average time for 1 transaction {}",
					(diff.as_millis() / length as u128)
				);
			},
			TransactionSender::JITO => {
				log::info!("Total transactions {:?}", all_transactions.len());
				let start_time = Instant::now();
				for transactions_iter in all_transactions {
					log::info!("Transactions to be sent {:?}", transactions_iter.len());

					for transactions in transactions_iter.chunks(4) {
						let mut tries = 0;

						let before_time = Instant::now();
						let mut current_transactions = Vec::new();
						let jito_address =
							Pubkey::from_str("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5")
								.unwrap();
						let ix = anchor_lang::solana_program::system_instruction::transfer(
							&authority.pubkey(),
							&jito_address,
							100000,
						);

						let tx = Transaction::new_with_payer(&[ix], Some(&authority.pubkey()));

						current_transactions.push(tx);
						current_transactions.extend(transactions.to_vec());
						while tries < 5 {
							println!("Try For Tx: {}", tries);
							let mut client = jito_searcher_client::get_searcher_client(
								&BLOCK_ENGINE_URL,
								&authority,
							)
							.await
							.expect("connects to searcher client");
							let mut bundle_results_subscription = client
								.subscribe_bundle_results(SubscribeBundleResultsRequest {})
								.await
								.expect("subscribe to bundle results")
								.into_inner();

							let rpc_client = self.rpc_client();
							let blockhash = rpc.get_latest_blockhash().await.unwrap();
							let versioned_transactions: Vec<VersionedTransaction> =
								current_transactions
									.clone()
									.into_iter()
									.map(|mut tx| {
										tx.sign(&[&*authority], blockhash);
										let serialized_tx = bincode::serialize(&tx).unwrap();
										// encode in base 58
										let encoded_tx = bs58::encode(serialized_tx).into_string();
										log::info!("Encoded tx {:?}", encoded_tx);
										tx.clone().into()
									})
									.collect();

							let signatures = jito_searcher_client::send_bundle_with_confirmation(
								&versioned_transactions,
								&rpc_client,
								&mut client,
								&mut bundle_results_subscription,
							)
							.await
							.or_else(|e| {
								println!("This is error {:?}", e);
								ibc::prelude::Err("Error".to_owned())
							});

							if let Ok(sigs) = signatures {
								signature = sigs.last().unwrap().to_string();
								break;
							} else {
								tries += 1;
								continue;
							}
						}
						if tries == 5 {
							log::error!("Failed to send transaction with the tries, Sending it through RPC Now");
							// Removing the first tx since its a tip
							current_transactions.remove(0);
							for mut transaction in current_transactions {
								let mut rpc_tries = 0;
								while rpc_tries < 10 {
									sleep(Duration::from_secs(1));
									log::info!("Current Try In RPC: {}", rpc_tries);
									let blockhash = rpc.get_latest_blockhash().await.unwrap();
									transaction.sign(&[&*authority], blockhash);
									let sig = rpc
										.send_transaction_with_config(
											&transaction,
											RpcSendTransactionConfig {
												skip_preflight: true,
												max_retries: Some(0),
												..RpcSendTransactionConfig::default()
											},
										)
										.await;

									if let Ok(si) = sig {
										signature = si.to_string();
										// Wait for finalizing the transaction
										let mut success = false;
										// let blockhash =
										// rpc.get_latest_blockhash().await.unwrap();
										for status_retry in 0..usize::MAX {
											match rpc.get_signature_status(&si).await.unwrap() {
												Some(Ok(_)) => {
													log::info!("  Signature {:?}", si);
													success = true;
													break;
												},
												Some(Err(e)) => {
													log::error!(
														"Error while sending the transaction {:?}",
														e
													);
													success = true;
													break;
												},
												None => {
													if !rpc
														.is_blockhash_valid(
															&blockhash,
															CommitmentConfig::processed(),
														)
														.await
														.unwrap()
													{
														// Block hash is not found by some reason
														log::error!("Blockhash not found");
														success = false;
														break;
													} else if status_retry < usize::MAX {
														// Retry twice a second
														sleep(Duration::from_millis(500));
														continue;
													}
												},
											}
										}
										if !success {
											rpc_tries += 1;
											continue;
										}
										break;
									} else {
										log::error!("Error {:?}", sig);
										rpc_tries += 1;
										continue;
									}
								}
								if rpc_tries == 10 {
									log::error!("Failed to send transaction through RPC too...");
								}
							}
						} else {
							let after_time = Instant::now();
							let diff = after_time - before_time;
							let success_rate = 100 / (tries + 1);
							log::info!("Time taken {}", diff.as_millis());
							log::info!("Success rate {}", success_rate);
						}
					}
				}
				let end_time = Instant::now();
				let diff = end_time - start_time;
				log::info!("Time taken for all transactions {}", diff.as_millis());
				log::info!(
					"Average time for 1 transaction {}",
					(diff.as_millis() / total_transactions_length as u128)
				);
			},
		};

		let blockhash = rpc.get_latest_blockhash().await.unwrap();
		// Wait for finalizing the transaction
		if !self.common_state.handshake_completed {
			loop {
				log::info!("Finalizing Transaction");
				let result = rpc
					.confirm_transaction_with_commitment(
						&Signature::from_str(&signature).unwrap(),
						CommitmentConfig::finalized(),
					)
					.await
					.unwrap();
				if result.value {
					break;
				}
				sleep(Duration::from_secs(1));
				log::info!("This is result {:?}", result);
				continue;
			}
		}

		*self.is_transaction_processing.lock().unwrap() = false;
		log::info!("Finished processing transaction");
		Ok(signature)
	}

	async fn query_client_message(
		&self,
		_update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		todo!()
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height
	}

	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error> {
		let err_str = if let Some(rpc_err) = error.downcast_ref::<Error>() {
			match rpc_err {
				Error::RpcError(s) => s.clone(),
				_ => "".to_string(),
			}
		} else {
			error.to_string()
		};
		log::info!(target: "hyperspace_solana", "Handling error: {err_str}");
		if err_str.contains("dispatch task is gone") ||
			err_str.contains("failed to send message to internal channel")
		{
			// self.reconnect().await?;
			self.common_state.rpc_call_delay *= 2;
		}

		Ok(())
	}

	fn common_state(&self) -> &CommonClientState {
		&self.common_state
	}

	fn common_state_mut(&mut self) -> &mut CommonClientState {
		&mut self.common_state
	}

	async fn reconnect(&mut self) -> anyhow::Result<()> {
		todo!()
	}

	async fn on_undelivered_sequences(&self, has: bool, kind: UndeliveredType) {
		let _ = Box::pin(async move {
			let __self = self;
			let has = has;
			let kind = kind;
			let () = { __self.common_state().on_undelivered_sequences(has, kind).await };
		});
	}

	fn has_undelivered_sequences(&self, kind: UndeliveredType) -> bool {
		self.common_state().has_undelivered_sequences(kind)
	}

	fn rpc_call_delay(&self) -> Duration {
		self.common_state().rpc_call_delay()
	}

	fn initial_rpc_call_delay(&self) -> Duration {
		self.common_state().initial_rpc_call_delay
	}

	fn set_rpc_call_delay(&mut self, delay: Duration) {
		self.common_state_mut().set_rpc_call_delay(delay)
	}
}

// #[test]
// pub fn test_seq() {
// 	let program_id = Pubkey::from_str("2HLLVco5HvwWriNbUhmVwA2pCetRkpgrqwnjcsZdyTKT").unwrap();
// 	let port_id = PortId::from_str("transfer").unwrap();
// 	let channel_id = ChannelId::from_str("channel-0").unwrap();
// 	let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
// 	let trie_seeds = &[TRIE_SEED];
// 	let trie_key = Pubkey::find_program_address(trie_seeds, &program_id).0;
// 	let trie_account = rpc_client
// 		.get_account_with_commitment(&trie_key, CommitmentConfig::processed())
// 		// .await
// 		.unwrap()
// 		.value
// 		.unwrap();
// 	let trie = solana_trie::TrieAccount::new(trie_account.data).unwrap();
// 	let new_port_id = ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
// 	let new_channel_id = ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
// 	let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
// 	let key = TrieKey::new(Tag::Receipt, trie_comp);
// 	let packet_receipt_sequences: Vec<u64> = trie
// 		.get_subtrie(&key)
// 		.unwrap()
// 		.iter()
// 		.map(|c| {
// 			u64::from_be_bytes(
// 				Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
// 			)
// 		})
// 		.collect();
// 	println!("{:?}", packet_receipt_sequences);
// }
