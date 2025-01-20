use super::{
	client::CosmosClient,
	events::{
		event_is_type_channel, event_is_type_client, event_is_type_connection,
		ibc_event_try_from_abci_event, IbcEventWithHeight,
	},
};
use crate::error::Error;
use futures::{
	stream::{self, select_all},
	Stream, StreamExt,
};
use ibc::{
	applications::transfer::{Amount, BaseDenom, PrefixedCoin, PrefixedDenom, TracePath},
	core::{
		ics02_client::{
			client_state::ClientType, events as ClientEvents,
			msgs::update_client::MsgUpdateAnyClient, trust_threshold::TrustThreshold,
		},
		ics04_channel::packet::Sequence,
		ics23_commitment::{commitment::CommitmentPrefix, specs::ProofSpecs},
		ics24_host::{
			identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, Path, ReceiptsPath, SeqRecvsPath, SeqSendsPath,
			},
		},
	},
	events::IbcEvent,
	protobuf::Protobuf,
	signer::Signer,
	timestamp::Timestamp,
	tx_msg::Msg,
	Height,
};
use ibc_primitives::PacketInfo as IbcPacketInfo;
use ibc_proto::{
	cosmos::{bank::v1beta1::QueryBalanceRequest, base::query::v1beta1::PageRequest},
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			Channel, QueryChannelResponse, QueryChannelsRequest, QueryChannelsResponse,
			QueryConnectionChannelsRequest, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketAcknowledgementsRequest,
			QueryPacketCommitmentResponse, QueryPacketCommitmentsRequest,
			QueryPacketReceiptResponse, QueryUnreceivedAcksRequest, QueryUnreceivedPacketsRequest,
		},
		client::v1::{
			QueryClientStateResponse, QueryClientStatesRequest, QueryConsensusStateResponse,
		},
		connection::v1::{
			ConnectionEnd, IdentifiedConnection, QueryConnectionResponse, QueryConnectionsRequest,
		},
	},
};
use ibc_rpc::PacketInfo;
use ics07_tendermint::{
	client_message::ClientMessage, client_state::ClientState, consensus_state::ConsensusState,
};
use ics08_wasm::msg::MsgPushNewWasmCode;
use pallet_ibc::light_clients::{
	AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager,
};
use primitives::{
	filter_events_by_ids, mock::LocalClientTypes, Chain, IbcProvider, KeyProvider, UpdateType,
};
use prost::Message;
use rand::Rng;
use std::{
	collections::{hash_map::Entry, HashMap, HashSet},
	pin::Pin,
	str::FromStr,
	time::Duration,
};
use tendermint::block::Height as TmHeight;
pub use tendermint::Hash;
use tendermint_rpc::{
	endpoint::tx::Response,
	event::{Event, EventData},
	query::{EventType, Query},
	Client, Error as RpcError, Order, SubscriptionClient,
};
use tokio::{task::JoinSet, time::sleep};

use tendermint::{
	account::Id as TMAccountId, block::signed_header::SignedHeader,
	validator::Set as TMValidatorSet,
};
use tendermint_light_client::components::io::{AtHeight, IoError};
use tendermint_light_client_verifier::types::{LightBlock, PeerId};
use tendermint_rpc::Paging;

// At least one *mandatory* update should happen during that period
// TODO: make it configurable
// pub const NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER: u64 = 500;

// These are the sequences which have an issue but have infinite timeout so we skip them
pub const SEQUENCES_TO_SKIP: [u64; 5] = [2545, 2546, 2555, 2556, 2557, 2583];

#[derive(Clone, Debug)]
pub enum FinalityEvent {
	Tendermint { from: TmHeight, to: TmHeight },
}

#[derive(Clone, Debug)]
pub struct TransactionId<Hash> {
	pub hash: Hash,
}

#[async_trait::async_trait]
impl<H> IbcProvider for CosmosClient<H>
where
	H: Clone + Send + Sync + 'static,
{
	type FinalityEvent = FinalityEvent;
	type TransactionId = TransactionId<Hash>;
	type AssetId = String;
	type Error = Error;

	async fn query_latest_ibc_events<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
	where
		C: Chain,
	{
		log::info!("Fetching latest cosmos ibc events");
		let finality_event_height = match finality_event {
			FinalityEvent::Tendermint { from: _, to } => to,
		};
		let client_id = self.client_id();
		let (latest_cp_height, latest_cp_time) = counterparty.latest_height_and_timestamp().await?;
		let latest_cp_client_state =
			counterparty.query_client_state(latest_cp_height, client_id.clone()).await?;
		let client_state_response = latest_cp_client_state
			.client_state
			.ok_or_else(|| Error::Custom("counterparty returned empty client state".to_string()))?;
		let client_state =
			ics07_tendermint::client_state::ClientState::<LocalClientTypes>::decode_vec(
				&client_state_response.value,
			)
			.map_err(|_| Error::Custom("failed to decode client state response".to_string()))?;
		let latest_cp_client_height = client_state.latest_height().revision_height;
		log::info!(
			"This is solana (cp) height on cosmos {:?} {:?}",
			client_state.latest_height(),
			client_state.chain_id()
		);
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let latest_revision = latest_height.revision_number;

		let trusted_latest_h = client_state.latest_height();

		// let (_, last_update_time) = counterparty
		// 	.query_client_update_time_and_height(client_id.clone(), trusted_latest_h)
		// 	.await?;

		let from = TmHeight::try_from(latest_cp_client_height).unwrap();
		let to = finality_event_height;
		log::info!(target: "hyperspace_cosmos", "--------------------------Getting blocks {}..{}----------------------", from, to);

		// let time_passed_since_last_update =
		// latest_cp_time.duration_since(&last_update_time).unwrap_or_else(|| { 	log::warn!(target:
		// "hyperspace_cosmos", "Last update time {last_update_time} > {latest_cp_time} (current
		// time on the counterparty chain)", ); 	Duration::from_secs(0)
		// });
		log::info!("Latest height {:?} and trusted height {:?}", latest_height, trusted_latest_h);
		let time_passed_since_last_update = Duration::from_secs(
			(latest_height.revision_height - trusted_latest_h.revision_height) *
				self.expected_block_time().as_secs(),
		);
		log::info!(
			"Time passed since last update on cosmos {:?}",
			time_passed_since_last_update.as_secs()
		);
		let mut force_update_at = None;
		// Force update if the finality event height is reached and the client was not
		// updated for the trusting period / 2 to avoid client expiration
		if time_passed_since_last_update > Duration::from_secs(30 * 60) {
			// This fixation on the block is needed to wait for the proof for the same block
			// in the next iterations, instead of requesting a new proof for another block
			// and never using it
			let fixed_update_height = finality_event_height.value();

			log::debug!(target: "hyperspace_cosmos", "Time passed since last update: {}, trusting period: {}", time_passed_since_last_update.as_secs(), client_state.trusting_period.as_secs());

			force_update_at = Some(Height::new(latest_height.revision_number, fixed_update_height));
		}

		if let Some(height) = force_update_at {
			log::info!(target: "hyperspace_cosmos", "Forcing update at height: {height}");
		}

		// query (exclusively) up to `to`, because the proof for the event at `to - 1` will be
		// contained at `to` and will be fetched below by `msg_update_client_header`
		let update_headers =
			self.msg_update_client_header(from, to, client_state.latest_height()).await?;
		// log::info!(target: "hyperspace_cosmos", "These are update headers {:?}", update_headers);
		let mut block_events = Vec::new();
		let mut join_set: JoinSet<Result<_, anyhow::Error>> = JoinSet::new();
		let range = (from.value()..to.value()).collect::<Vec<_>>();
		let to = self.rpc_call_delay().as_millis();
		for heights in range.chunks(100) {
			for height in heights.iter().copied() {
				log::trace!(target: "hyperspace_cosmos", "Parsing events at height {:?}", height);
				let client = self.clone();
				let duration = Duration::from_millis(rand::thread_rng().gen_range(0..to) as u64);
				let counterparty = counterparty.clone();
				join_set.spawn(async move {
					sleep(duration).await;
					let xs = tokio::time::timeout(
						Duration::from_secs(30),
						client.parse_ibc_events_at(&counterparty, latest_revision, height),
					)
					.await??;
					Ok((height, xs))
				});
			}
			while let Some(res) = join_set.join_next().await {
				let out = res??;
				block_events.push(out);
			}
		}

		if block_events.len() != update_headers.len() {
			return Err(anyhow::anyhow!(
				"block events and updates must match, got {} and {}",
				block_events.len(),
				update_headers.len()
			))
		}
		block_events.sort_by_key(|(height, _)| *height);

		let mut updates = Vec::new();
		for (i, (events, (update_header, mut update_type))) in block_events
			.into_iter()
			.map(|(_, events)| events)
			.zip(update_headers)
			.enumerate()
		{
			// if i == NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER as usize - 1 {
			// 	update_type = UpdateType::Mandatory;
			// }
			let height = update_header.height();
			let update_client_header = {
				let msg = MsgUpdateAnyClient::<LocalClientTypes> {
					client_id: client_id.clone(),
					client_message: AnyClientMessage::Tendermint(ClientMessage::Header(
						update_header,
					)),
					signer: counterparty.account_id(),
				};
				let value = msg.encode_vec().map_err(|e| {
					Error::from(format!("Failed to encode MsgUpdateClient {msg:?}: {e:?}"))
				})?;
				Any { value, type_url: msg.type_url() }
			};

			if Some(height) == force_update_at {
				update_type = UpdateType::Mandatory;
				log::info!(target: "hyperspace_cosmos", "Forcing update to mandatory at height: {:?}", height);
			}

			// println!("These are events caught query latest events {:?}", events);
			updates.push((update_client_header, height, events, update_type));
		}
		Ok(updates)
	}

	// TODO: Changed result: `Item =` from `IbcEvent` to `IbcEventWithHeight` to include the
	// necessary height field, as `height` is removed from `Attribute` from ibc-rs v0.22.0
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		// Create websocket client. Like what `EventMonitor::subscribe()` does in `hermes`
		let ws_client = self.rpc_client.clone();
		log::info!("Inside cosmos ibc events");

		let query_all = vec![
			Query::from(EventType::NewBlock),
			Query::eq("message.module", "ibc_client"),
			Query::eq("message.module", "ibc_connection"),
			Query::eq("message.module", "ibc_channel"),
		];
		let mut subscriptions = vec![];
		for query in &query_all {
			let subscription = ws_client
				.subscribe(query.clone())
				.await
				.map_err(|e| Error::from(format!("Web Socket Client Error {e:?}")))
				.unwrap();
			subscriptions.push(subscription);
		}
		// Collect IBC events from each RPC event, Like what `stream_batches()` does in `hermes`
		let all_subs: Box<dyn Stream<Item = Result<Event, RpcError>> + Send + Sync + Unpin> =
			Box::new(select_all(subscriptions));
		let chain_id = self.chain_id.clone();
		let events = all_subs
			.map(move |event| {
				// Like what `get_all_events()` does in `hermes`
				let mut events_with_height: Vec<IbcEventWithHeight> = vec![];
				let Event { data, events: _, query } = event.unwrap();
				match data {
					EventData::NewBlock { block, .. }
					if query == Query::from(EventType::NewBlock).to_string() =>
						{
							let height = Height::new(
								ChainId::chain_version(chain_id.to_string().as_str()),
								u64::from(block.as_ref().ok_or("tx.height").unwrap().header.height),
							);
							events_with_height.push(IbcEventWithHeight::new(
								ClientEvents::NewBlock::new(height).into(),
								height,
							));
						},
					EventData::Tx { tx_result } => {
						let height = Height::new(
							ChainId::chain_version(chain_id.to_string().as_str()),
							tx_result.height as u64,
						);
						for abci_event in &tx_result.result.events {
							if let Ok(ibc_event) = ibc_event_try_from_abci_event(abci_event, height)
							{
								log::debug!(target: "hyperspace_cosmos", "Retrieved event: {}, query: {}, parsed: {:?}", abci_event.kind, query, ibc_event);
								let is_client_event = query == Query::eq("message.module", "ibc_client").to_string() &&
									event_is_type_client(&ibc_event);
								let is_connection_event = (query ==
									Query::eq("message.module", "ibc_connection").to_string() ||
									query ==
										Query::eq("message.module", "ibc_client").to_string()) &&
									event_is_type_connection(&ibc_event);
								let is_channel_event = query ==
									Query::eq("message.module", "ibc_channel").to_string() &&
									event_is_type_channel(&ibc_event);
								if is_client_event || is_connection_event || is_channel_event {
									events_with_height
										.push(IbcEventWithHeight::new(ibc_event, height));
								} else {
									log::debug!(target: "hyperspace_cosmos", "the event is unknown");
								}
							} else {
								log::debug!(target: "hyperspace_cosmos", "Event wasn't parsed {:?}", abci_event);
							}
						}
					},
					_ => {},
				}
				stream::iter(events_with_height)
			})
			.flatten()
			.map(|e| e.event)
			.boxed();
		events
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let path_bytes = Path::ClientConsensusState(ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: consensus_height.revision_number,
			height: consensus_height.revision_height,
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let consensus_state = Any::decode(&*query_result.value)?;
		Ok(QueryConsensusStateResponse {
			consensus_state: Some(consensus_state),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let path_bytes =
			Path::ClientState(ClientStatePath(client_id.clone())).to_string().into_bytes();
		let (q, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let client_state = Any::decode(&*q.value)?;
		if client_state.type_url.is_empty() || client_state.value.is_empty() {
			return Err(Error::Custom(format!("empty client state for height {at}")))
		}
		Ok(QueryClientStateResponse {
			client_state: Some(client_state),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let path_bytes = Path::Connections(ConnectionsPath(connection_id.clone()))
			.to_string()
			.into_bytes();
		let (q, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let connection = ConnectionEnd::decode(&*q.value)?;
		Ok(QueryConnectionResponse {
			connection: Some(connection),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let path_bytes = Path::ChannelEnds(ChannelEndsPath(port_id.clone(), channel_id))
			.to_string()
			.into_bytes();
		let (q, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let channel = Channel::decode(&*q.value)?;
		Ok(QueryChannelResponse {
			channel: Some(channel),
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let (_, proof) = self.query_path(keys[0].clone(), at, true).await?;
		Ok(proof)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let path_bytes = Path::Commitments(CommitmentsPath {
			port_id: port_id.clone(),
			channel_id: *channel_id,
			sequence: Sequence::from(seq),
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		Ok(QueryPacketCommitmentResponse {
			commitment: query_result.value,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let path_bytes = Path::Acks(AcksPath {
			port_id: port_id.clone(),
			channel_id: *channel_id,
			sequence: Sequence::from(seq),
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes, at, true).await?;
		Ok(QueryPacketAcknowledgementResponse {
			acknowledgement: query_result.value,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let path_bytes = Path::SeqRecvs(SeqRecvsPath(port_id.clone(), *channel_id))
			.to_string()
			.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes.clone(), at, true).await?;
		let next_sequence_receive = u64::from_be_bytes(
			query_result
				.value
				.try_into()
				.map_err(|_| Error::Custom("invalid next_sequence_receive value".to_owned()))?,
		);
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let path_bytes = Path::Receipts(ReceiptsPath {
			port_id: port_id.clone(),
			channel_id: *channel_id,
			sequence: Sequence::from(seq),
		})
		.to_string()
		.into_bytes();
		let (query_result, proof) = self.query_path(path_bytes, at, true).await?;
		let received = query_result.value[0] == 1;
		Ok(QueryPacketReceiptResponse {
			received,
			proof,
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		// We cannot rely on `/status` endpoint to provide details about the latest block.
		// Instead, we need to pull block height via `/abci_info` and then fetch block
		// metadata at the given height via `/blockchain` endpoint.
		let abci_info = self
			.rpc_client
			.abci_info()
			.await
			.map_err(|e| Error::RpcError(format!("{e:?}")))?;

		// Query `/blockchain` endpoint to pull the block metadata corresponding to
		// the latest block that the application committed.
		// TODO: Replace this query with `/header`, once it's available.
		//  https://github.com/informalsystems/tendermint-rs/pull/1101
		let blocks = self
			.rpc_client
			.blockchain(abci_info.last_block_height, abci_info.last_block_height)
			.await
			.map_err(|e| {
				Error::RpcError(format!(
					"failed to query /blockchain endpoint for latest app. block: {e:?}"
				))
			})?;

		let latest_app_block = blocks.block_metas.first().ok_or_else(|| {
			Error::Custom("/blockchain endpoint for latest app. block".to_owned())
		})?;

		let height = Height::new(
			ChainId::chain_version(latest_app_block.header.chain_id.as_str()),
			u64::from(abci_info.last_block_height),
		);
		let timestamp = latest_app_block.header.time.into();
		Ok((height, timestamp))
	}

	async fn query_packet_commitments(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryPacketCommitmentsRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			pagination: Some(PageRequest { limit: u32::MAX as _, ..Default::default() }),
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.packet_commitments(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> =
			response.commitments.into_iter().map(|v| v.sequence).collect();

		// Filtering out the sequences that are not in the SEQUENCES_TO_SKIP list
		//
		// These sequences have infinite timeout and are failing on solana due to an issue with
		// the hook.
		let filtered_seqs = commitment_sequences
			.iter()
			.filter(|&seq| !SEQUENCES_TO_SKIP.contains(seq))
			.copied()
			.collect::<Vec<_>>();

		Ok(filtered_seqs)
	}

	async fn query_packet_acknowledgements(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"Querying packet acknowledgements for channel {:?} on port {:?}",
			channel_id,
			port_id
		);
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryPacketAcknowledgementsRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			packet_commitment_sequences: vec![],
			pagination: Some(PageRequest { limit: u32::MAX as _, ..Default::default() }),
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.packet_acknowledgements(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> =
			response.acknowledgements.into_iter().map(|v| v.sequence).collect();

		Ok(commitment_sequences)
	}

	async fn query_unreceived_packets(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryUnreceivedPacketsRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			packet_commitment_sequences: seqs,
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.unreceived_packets(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> = response.sequences.into_iter().collect();

		Ok(commitment_sequences)
	}

	async fn query_unreceived_acknowledgements(
		&self,
		_at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(e.to_string()))?;

		let request = QueryUnreceivedAcksRequest {
			port_id: port_id.to_string(),
			channel_id: channel_id.to_string(),
			packet_ack_sequences: seqs,
		};
		let request = tonic::Request::new(request);
		let response = grpc_client
			.unreceived_acks(request)
			.await
			.map_err(|e| Error::from(e.to_string()))?
			.into_inner();

		let commitment_sequences: Vec<u64> = response.sequences.into_iter().collect();

		Ok(commitment_sequences)
	}

	fn channel_whitelist(&self) -> HashSet<(ChannelId, PortId)> {
		self.channel_whitelist.lock().unwrap().clone()
	}

	async fn query_connection_channels(
		&self,
		_at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("{e:?}")))?;
		let request = tonic::Request::new(QueryConnectionChannelsRequest {
			connection: connection_id.to_string(),
			pagination: Some(PageRequest { limit: u32::MAX as _, ..Default::default() }),
		});

		let response = grpc_client
			.connection_channels(request)
			.await
			.map_err(|e| Error::from(format!("{e:?}")))?
			.into_inner();
		let channels = QueryChannelsResponse {
			channels: response.channels,
			pagination: response.pagination,
			height: response.height,
		};

		Ok(channels)
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		log::info!(
			target: "hyperspace_cosmos",
			"query_send_packets: channel_id: {}, port_id: {}, seqs: {:?}", channel_id, port_id, seqs
		);
		let mut block_events = HashMap::<u64, PacketInfo>::new();

		for seq in seqs.iter() {
			if block_events.contains_key(seq) {
				continue
			}
			let query_str = Query::eq("send_packet.packet_src_channel", channel_id.to_string())
				.and_eq("send_packet.packet_src_port", port_id.to_string())
				.and_eq("send_packet.packet_sequence", seq.to_string());

			let response = self
				.rpc_http_client
				.tx_search(
					query_str,
					true,
					1,
					1, // get only the first Tx matching the query
					Order::Descending, /* query the most recent event, there is possibility that the same
					    * sequence number is used twice in send_packet event (in case of an
					    * error during the message processing) */
				)
				.await
				.map_err(|e| Error::RpcError(format!("{e:?}")))?;

			for tx in response.txs {
				for ev in &tx.tx_result.events {
					let height = tx.height.value();
					let ev =
						ibc_event_try_from_abci_event(ev, Height::new(self.id().version(), height));

					match ev {
						Ok(IbcEvent::SendPacket(p))
							if seqs.contains(&p.packet.sequence.0) &&
								p.packet.source_port == port_id && p.packet.source_channel ==
								channel_id =>
						{
							let seq = p.packet.sequence.0;
							let mut info = PacketInfo::try_from(IbcPacketInfo::from(p.packet))
								.map_err(|_| {
									Error::from(
										"failed to convert packet info from IbcPacketInfo"
											.to_string(),
									)
								})?;
							info.height = Some(p.height.revision_height);
							let entry = block_events.entry(seq);
							match entry {
								Entry::Occupied(mut packet) => {
									if packet.get().height.unwrap() <= p.height.revision_height {
										packet.insert(info);
									}
								},
								Entry::Vacant(v) => {
									v.insert(info);
								},
							}
						},
						_ => (),
					}
				}
			}
		}
		Ok(block_events.into_values().collect())
	}

	async fn query_received_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"query_recv_packets: channel_id: {}, port_id: {}, seqs: {:?}", channel_id, port_id, seqs
		);

		let mut block_events = HashMap::<u64, PacketInfo>::new();

		for seq in seqs.iter() {
			if block_events.contains_key(seq) {
				continue
			}

			let query_str =
				Query::eq("write_acknowledgement.packet_dst_channel", channel_id.to_string())
					.and_eq("write_acknowledgement.packet_dst_port", port_id.to_string())
					.and_eq("write_acknowledgement.packet_sequence", seq.to_string());

			let response = self
				.rpc_http_client
				.tx_search(
					query_str,
					true,
					1,
					1, // get only the first Tx matching the query
					Order::Descending, /* query the most recent event, there is possibility that the same
					    * sequence number is used twice in write_acknowledgement event (in case
					    * of an error during the message processing) */
				)
				.await
				.map_err(|e| Error::RpcError(format!("{e:?}")))?;

			for tx in response.txs {
				for ev in &tx.tx_result.events {
					let height = tx.height.value();
					let ev =
						ibc_event_try_from_abci_event(ev, Height::new(self.id().version(), height));

					match ev {
						Ok(IbcEvent::WriteAcknowledgement(p))
							if seqs.contains(&p.packet.sequence.0) &&
								p.packet.destination_port == port_id &&
								p.packet.destination_channel == channel_id =>
						{
							let seq = p.packet.sequence.0;
							let mut info = PacketInfo::try_from(IbcPacketInfo::from(p.packet))
								.map_err(|_| {
									Error::from(
										"failed to convert packet info from IbcPacketInfo"
											.to_string(),
									)
								})?;
							info.ack = Some(p.ack);
							info.height = Some(p.height.revision_height);
							let entry = block_events.entry(seq);
							match entry {
								Entry::Occupied(mut packet) => {
									if packet.get().height.unwrap() <= p.height.revision_height {
										packet.insert(info);
									}
								},
								Entry::Vacant(v) => {
									v.insert(info);
								},
							}
						},
						_ => (),
					}
				}
			}
		}
		Ok(block_events.into_values().collect())
	}

	fn expected_block_time(&self) -> Duration {
		// cosmos chain block time is roughly 6-7 seconds
		Duration::from_secs(5)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		log::debug!(
			target: "hyperspace_cosmos",
			"Querying client update time and height for client {:?} at height {:?}",
			client_id,
			client_height
		);
		let query_update = Query::eq("update_client.client_id", client_id.to_string())
			.and_eq("update_client.consensus_height", client_height.to_string());
		let query_create = Query::eq("create_client.client_id", client_id.to_string())
			.and_eq("create_client.consensus_height", client_height.to_string());
		for query_str in [query_update, query_create] {
			let response = self
				.rpc_http_client
				.tx_search(
					query_str,
					true,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::RpcError(format!("{e:?}")))?;

			for tx in response.txs {
				for ev in &tx.tx_result.events {
					let height = tx.height.value();
					let ev =
						ibc_event_try_from_abci_event(ev, Height::new(self.id().version(), height));
					let timestamp = self
						.query_timestamp_at(height)
						.await
						.map_err(|e| Error::RpcError(format!("{e:?}")))?;
					match ev {
						Ok(IbcEvent::UpdateClient(e)) if e.client_id() == &client_id =>
							return Ok((
								Height::new(self.chain_id.version(), height),
								Timestamp::from_nanoseconds(timestamp)?,
							)),
						Ok(IbcEvent::CreateClient(e)) if e.client_id() == &client_id =>
							return Ok((
								Height::new(self.chain_id.version(), height),
								Timestamp::from_nanoseconds(timestamp)?,
							)),
						_ => (),
					}
				}
			}
		}
		Err(Error::from("not found".to_string()))
	}

	async fn query_host_consensus_state_proof(
		&self,
		_client_state: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		unimplemented!()
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		let denom = &asset_id;
		let mut grpc_client = ibc_proto::cosmos::bank::v1beta1::query_client::QueryClient::connect(
			self.grpc_url.clone().to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("{e:?}")))?;

		let request = tonic::Request::new(QueryBalanceRequest {
			address: self.keybase.clone().account,
			denom: denom.to_string(),
		});

		let response = grpc_client
			.balance(request)
			.await
			.map(|r| r.into_inner())
			.map_err(|e| Error::from(format!("{e:?}")))?;

		// Querying for a balance might fail, i.e. if the account doesn't actually exist
		let balance = response
			.balance
			.ok_or_else(|| Error::from(format!("No balance for denom {denom}")))?;

		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom {
				trace_path: TracePath::default(),
				base_denom: BaseDenom::from_str(denom)?,
			},
			amount: Amount::from_str(balance.amount.as_str())?,
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		self.commitment_prefix.clone()
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	fn set_client_id(&mut self, client_id: ClientId) {
		*self.client_id.lock().unwrap() = Some(client_id);
	}

	fn connection_id(&self) -> Option<ConnectionId> {
		self.connection_id.lock().unwrap().clone()
	}

	/// Set the channel whitelist for the relayer task.
	fn set_channel_whitelist(&mut self, channel_whitelist: HashSet<(ChannelId, PortId)>) {
		*self.channel_whitelist.lock().unwrap() = channel_whitelist;
	}

	fn add_channel_to_whitelist(&mut self, channel: (ChannelId, PortId)) {
		self.channel_whitelist.lock().unwrap().insert(channel);
	}

	fn set_connection_id(&mut self, connection_id: ConnectionId) {
		*self.connection_id.lock().unwrap() = Some(connection_id);
	}

	fn client_type(&self) -> ClientType {
		ClientState::<()>::client_type()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let height = TmHeight::try_from(block_number)
			.map_err(|e| Error::from(format!("Invalid block number: {e}")))?;
		let response = self
			.rpc_client
			.block(height)
			.await
			.map_err(|e| Error::RpcError(e.to_string()))?;
		let time: Timestamp = response.block.header.time.into();
		Ok(time.nanoseconds())
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let request = tonic::Request::new(QueryClientStatesRequest {
			pagination: Some(PageRequest { limit: u32::MAX as _, ..Default::default() }),
		});
		let grpc_client = ibc_proto::ibc::core::client::v1::query_client::QueryClient::new(
			self.grpc_client.clone(),
		);
		let response = grpc_client
			.clone()
			.client_states(request)
			.await
			.map_err(|e| {
				Error::from(format!("Failed to query client states from grpc client: {e:?}"))
			})?
			.into_inner();

		// Deserialize into domain type
		let clients: Vec<ClientId> = response
			.client_states
			.into_iter()
			.filter_map(|cs| {
				let id = ClientId::from_str(&cs.client_id).ok()?;
				Some(id)
			})
			.collect();
		Ok(clients)
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let request = tonic::Request::new(QueryChannelsRequest {
			pagination: Some(PageRequest { limit: u32::MAX as _, ..Default::default() }),
		});
		let mut grpc_client =
			ibc_proto::ibc::core::channel::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("{e:?}")))?;
		let response = grpc_client
			.channels(request)
			.await
			.map_err(|e| Error::from(format!("{e:?}")))?
			.into_inner()
			.channels
			.into_iter()
			.filter_map(|c| {
				let id = ChannelId::from_str(&c.channel_id).ok()?;
				let port_id = PortId::from_str(&c.port_id).ok()?;
				Some((id, port_id))
			})
			.collect::<Vec<_>>();
		Ok(response)
	}

	async fn query_connection_using_client(
		&self,
		_height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let mut grpc_client =
			ibc_proto::ibc::core::connection::v1::query_client::QueryClient::connect(
				self.grpc_url.clone().to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("{e:?}")))?;

		let request = tonic::Request::new(QueryConnectionsRequest {
			pagination: Some(PageRequest { limit: u32::MAX as _, ..Default::default() }),
		});

		let response = grpc_client
			.connections(request)
			.await
			.map_err(|e| Error::from(format!("{e:?}")))?
			.into_inner();

		let connections = response
			.connections
			.into_iter()
			.filter(|conn| {
				conn.client_id == client_id ||
					conn.counterparty.as_ref().map(|x| x.client_id == client_id).unwrap_or(false)
			})
			.collect();
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
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		let latest_height_timestamp = self.latest_height_and_timestamp().await?;
		println!("This is height on cosmos {:?}", latest_height_timestamp);
		let client_state = ClientState::new(
			self.chain_id.clone(),
			TrustThreshold::default(),
			Duration::from_secs(640000),
			Duration::from_secs(1814400),
			Duration::new(1500000, 0),
			latest_height_timestamp.0,
			ProofSpecs::default(),
			vec!["upgrade".to_string(), "upgradedIBCState".to_string()],
		)
		.map_err(|e| Error::from(format!("Invalid client state {e}")))?;
		let light_block = self
			.light_client
			.verify(latest_height_timestamp.0, latest_height_timestamp.0, &client_state)
			.await
			.map_err(|e| Error::from(format!("Invalid light block {e}")))?;
		let consensus_state = ConsensusState::from(light_block.signed_header.header);
		println!("--------------------------------------------");
		println!("This is consensus state timestamp {:?}", consensus_state.timestamp);
		println!("--------------------------------------------");
		Ok((
			AnyClientState::Tendermint(client_state),
			AnyConsensusState::Tendermint(consensus_state),
		))
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		const WAIT_BACKOFF: Duration = Duration::from_millis(300);
		const TIME_OUT: Duration = Duration::from_millis(30000);
		let start_time = std::time::Instant::now();

		let response: Response = loop {
			let response = self
				.rpc_client
				.tx_search(
					Query::eq("tx.hash", tx_id.hash.to_string()),
					false,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::from(format!("Failed to query tx hash: {e}")))?;
			match response.txs.into_iter().next() {
				None => {
					let elapsed = start_time.elapsed();
					if elapsed > TIME_OUT {
						return Err(Error::from(format!(
							"Timeout waiting for tx {:?} to be included in a block",
							tx_id.hash
						)))
					} else {
						std::thread::sleep(WAIT_BACKOFF);
					}
				},
				Some(resp) => break resp,
			}
		};

		let height = Height::new(
			ChainId::chain_version(self.chain_id.to_string().as_str()),
			response.height.value(),
		);
		let deliver_tx_result = response.tx_result;
		if deliver_tx_result.code.is_err() {
			Err(Error::from(format!(
				"Transaction failed with code {:?} and log {:?}",
				deliver_tx_result.code, deliver_tx_result.log
			)))
		} else {
			let result = deliver_tx_result
				.events
				.iter()
				.flat_map(|e| ibc_event_try_from_abci_event(e, height).ok().into_iter())
				.filter(|e| matches!(e, IbcEvent::CreateClient(_)))
				.collect::<Vec<_>>();
			if result.len() != 1 {
				Err(Error::from(format!(
					"Expected exactly one CreateClient event, found {}",
					result.len()
				)))
			} else {
				Ok(match result[0] {
					IbcEvent::CreateClient(ref e) => e.client_id().clone(),
					_ => unreachable!(),
				})
			}
		}
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		const WAIT_BACKOFF: Duration = Duration::from_millis(300);
		const TIME_OUT: Duration = Duration::from_millis(30000);
		let start_time = std::time::Instant::now();

		let response: Response = loop {
			let response = self
				.rpc_client
				.tx_search(
					Query::eq("tx.hash", tx_id.hash.to_string()),
					false,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::from(format!("Failed to query tx hash: {e}")))?;
			match response.txs.into_iter().next() {
				None => {
					let elapsed = start_time.elapsed();
					if elapsed > TIME_OUT {
						return Err(Error::from(format!(
							"Timeout waiting for tx {:?} to be included in a block",
							tx_id.hash
						)))
					} else {
						std::thread::sleep(WAIT_BACKOFF);
					}
				},
				Some(resp) => break resp,
			}
		};

		let height = Height::new(
			ChainId::chain_version(self.chain_id.to_string().as_str()),
			response.height.value(),
		);
		let deliver_tx_result = response.tx_result;
		if deliver_tx_result.code.is_err() {
			Err(Error::from(format!(
				"Transaction failed with code {:?} and log {:?}",
				deliver_tx_result.code, deliver_tx_result.log
			)))
		} else {
			let result = deliver_tx_result
				.events
				.iter()
				.flat_map(|e| ibc_event_try_from_abci_event(e, height).ok().into_iter())
				.filter(|e| matches!(e, IbcEvent::OpenInitConnection(_)))
				.collect::<Vec<_>>();
			if result.len() != 1 {
				Err(Error::from(format!(
					"Expected exactly one CreateClient event, found {}",
					result.len()
				)))
			} else {
				Ok(match result[0] {
					IbcEvent::OpenInitConnection(ref e) =>
						e.connection_id().expect("Connection id wasn't found").clone(),
					_ => unreachable!(),
				})
			}
		}
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<(ChannelId, PortId), Self::Error> {
		const WAIT_BACKOFF: Duration = Duration::from_millis(300);
		const TIME_OUT: Duration = Duration::from_millis(30000);
		let start_time = std::time::Instant::now();

		let response: Response = loop {
			let response = self
				.rpc_client
				.tx_search(
					Query::eq("tx.hash", tx_id.hash.to_string()),
					false,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::from(format!("Failed to query tx hash: {e}")))?;
			match response.txs.into_iter().next() {
				None => {
					let elapsed = start_time.elapsed();
					if elapsed > TIME_OUT {
						return Err(Error::from(format!(
							"Timeout waiting for tx {:?} to be included in a block",
							tx_id.hash
						)))
					} else {
						std::thread::sleep(WAIT_BACKOFF);
					}
				},
				Some(resp) => break resp,
			}
		};

		let height = Height::new(
			ChainId::chain_version(self.chain_id.to_string().as_str()),
			response.height.value(),
		);
		let deliver_tx_result = response.tx_result;
		if deliver_tx_result.code.is_err() {
			Err(Error::from(format!(
				"Transaction failed with code {:?} and log {:?}",
				deliver_tx_result.code, deliver_tx_result.log
			)))
		} else {
			let result = deliver_tx_result
				.events
				.iter()
				.flat_map(|e| ibc_event_try_from_abci_event(e, height).ok().into_iter())
				.filter(|e| matches!(e, IbcEvent::OpenInitChannel(_)))
				.collect::<Vec<_>>();
			if result.len() != 1 {
				Err(Error::from(format!(
					"Expected exactly one CreateClient event, found {}",
					result.len()
				)))
			} else {
				Ok(match result[0] {
					IbcEvent::OpenInitChannel(ref e) =>
						(*e.channel_id().expect("Channel id wasn't found"), e.port_id().clone()),
					_ => unreachable!(),
				})
			}
		}
	}

	async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		let msg = MsgPushNewWasmCode { signer: self.account_id(), code: wasm };
		// log::info!("This is the msg {:?}", msg);
		let hash = self.submit(vec![msg.into()]).await?;
		let resp = self.wait_for_tx_result(hash).await?;
		let height = Height::new(
			ChainId::chain_version(self.chain_id.to_string().as_str()),
			resp.height.value(),
		);
		let deliver_tx_result = resp.tx_result;
		let mut result = deliver_tx_result
			.events
			.iter()
			.flat_map(|e| ibc_event_try_from_abci_event(e, height).ok().into_iter())
			.filter(|e| matches!(e, IbcEvent::PushWasmCode(_)))
			.collect::<Vec<_>>();
		let code_id = if result.len() != 1 {
			return Err(Error::from(format!(
				"Expected exactly one PushWasmCode event, found {}",
				result.len()
			)))
		} else {
			match result.pop().unwrap() {
				IbcEvent::PushWasmCode(ev) => ev.0,
				_ => unreachable!(),
			}
		};
		// let resp = MsgClient::connect(
		// 	Endpoint::try_from(self.grpc_url.to_string())
		// 		.map_err(|e| Error::from(format!("Failed to parse grpc url: {:?}", e)))?,
		// )
		// .await
		// .map_err(|e| Error::from(format!("Failed to connect to grpc endpoint: {:?}", e)))?
		// .push_new_wasm_code(msg)
		// .await
		// .map_err(|e| {
		// 	Error::from(format!("Failed to upload wasm code to grpc endpoint: {:?}", e))
		// })?;

		Ok(code_id)
	}
}

impl<H> CosmosClient<H>
where
	H: 'static + Clone + Send + Sync,
{
	async fn fetch_validator_set(
		&self,
		height: AtHeight,
		proposer_address: Option<TMAccountId>,
	) -> Result<TMValidatorSet, IoError> {
		let height = match height {
			AtHeight::Highest => return Err(IoError::invalid_height()),
			AtHeight::At(height) => height,
		};

		let client = &self.rpc_client;
		let response = client.validators(height, Paging::All).await.map_err(IoError::rpc)?;

		let validator_set = match proposer_address {
			Some(proposer_address) =>
				TMValidatorSet::with_proposer(response.validators, proposer_address)
					.map_err(IoError::invalid_validator_set)?,
			None => TMValidatorSet::without_proposer(response.validators),
		};

		Ok(validator_set)
	}

	async fn fetch_signed_header(&self, height: AtHeight) -> Result<SignedHeader, IoError> {
		let client = self.rpc_client.clone();
		let res = match height {
			AtHeight::Highest => client.latest_commit().await,
			AtHeight::At(height) => client.commit(height).await,
		};

		match res {
			Ok(response) => Ok(response.signed_header),
			Err(err) => Err(IoError::rpc(err)),
		}
	}

	pub async fn fetch_light_block(
		&self,
		height: AtHeight,
		peer_id: PeerId,
	) -> Result<LightBlock, IoError> {
		let signed_header = self.fetch_signed_header(height).await?;
		let height = signed_header.header.height;
		let proposer_address = signed_header.header.proposer_address;

		let validator_set = self.fetch_validator_set(height.into(), Some(proposer_address)).await?;
		let next_validator_set = self.fetch_validator_set(height.increment().into(), None).await?;

		let light_block =
			LightBlock::new(signed_header, validator_set, next_validator_set, peer_id);

		Ok(light_block)
	}

	async fn parse_ibc_events_at<C: Chain>(
		&self,
		counterparty: &C,
		latest_revision: u64,
		height: u64,
	) -> Result<Vec<IbcEvent>, <Self as IbcProvider>::Error> {
		let mut ibc_events = Vec::new();

		let block_results = self
			.rpc_http_client
			.block_results(TmHeight::try_from(height)?)
			.await
			.map_err(|e| {
			Error::from(format!("Failed to query block result for height {height:?}: {e:?}"))
		})?;

		let tx_events = block_results
			.txs_results
			.unwrap_or_default()
			.into_iter()
			.flat_map(|tx| tx.events);
		let begin_events = block_results.begin_block_events.unwrap_or_default().into_iter();
		let end_events = block_results.end_block_events.unwrap_or_default().into_iter();
		let events = begin_events.chain(tx_events).chain(end_events);

		let ibc_height = Height::new(latest_revision, height);
		for event in events {
			let mut channel_and_port_ids = self.channel_whitelist();
			channel_and_port_ids.extend(counterparty.channel_whitelist());
			// log::info!("host channel and port ids {:?} ", self.channel_whitelist());
			// log::info!("counterparty channel and port ids {:?}, ",
			// counterparty.channel_whitelist()); log::info!("Host connection {:?} counterparty
			// {:?}", self.connection_id(), counterparty.connection_id()); log::info!("Host clientID
			// {:?} counterparty {:?}", self.client_id(), counterparty.client_id());

			let ibc_event = ibc_event_try_from_abci_event(&event, ibc_height).ok();
			if matches!(ibc_event, ibc::prelude::Some(IbcEvent::OpenTryConnection(_))) {
				println!("-------------------------OpenTryConnection found --------------");
			}
			match ibc_event {
				Some(mut ev) => {
					let is_filtered = filter_events_by_ids(
						&ev,
						&[self.client_id(), counterparty.client_id()],
						&[self.connection_id(), counterparty.connection_id()]
							.into_iter()
							.flatten()
							.collect::<Vec<_>>(),
						&channel_and_port_ids,
					);

					if matches!(ev, IbcEvent::OpenTryConnection(_)) {
						if is_filtered {
							println!("This is is_filtered")
						}
					}

					if is_filtered {
						ev.set_height(ibc_height);
						log::debug!(target: "hyperspace_cosmos", "Encountered event at {height}: {:?}", event.kind);
						ibc_events.push(ev);
					} else {
						log::debug!(target: "hyperspace_cosmos", "Filtered out event: {:?}",
					event.kind);
					}
				},
				None => {
					let ignored_events = [
						"commission",
						"rewards",
						"transfer",
						"mint",
						"withdraw_rewards",
						"coin_spent",
						"coin_received",
						"withdraw_commission",
						"message",
						"liveness",
						"tx",
						"fungible_token_packet",
					];
					if !ignored_events.contains(&event.kind.as_str()) {
						log::debug!(target: "hyperspace_cosmos", "Skipped event: {:?}", event.kind);
					}
					continue
				},
			}
		}
		Ok(ibc_events)
	}
}

impl<H: Clone + Send + Sync + 'static> CosmosClient<H> {
	#[allow(unused)]
	async fn wait_for_tx_result(
		&self,
		tx_id: <Self as IbcProvider>::TransactionId,
	) -> Result<Response, <Self as IbcProvider>::Error> {
		const WAIT_BACKOFF: Duration = Duration::from_millis(300);
		const TIME_OUT: Duration = Duration::from_millis(30000);
		let start_time = std::time::Instant::now();

		let response: Response = loop {
			let response = self
				.rpc_http_client
				.tx_search(
					Query::eq("tx.hash", tx_id.hash.to_string()),
					false,
					1,
					1, // get only the first Tx matching the query
					Order::Ascending,
				)
				.await
				.map_err(|e| Error::from(format!("Failed to query tx hash: {e}")))?;
			match response.txs.into_iter().next() {
				None => {
					let elapsed = start_time.elapsed();
					if elapsed > TIME_OUT {
						return Err(Error::from(format!(
							"Timeout waiting for tx {:?} to be included in a block",
							tx_id.hash
						)))
					} else {
						sleep(WAIT_BACKOFF).await;
					}
				},
				Some(resp) => break resp,
			}
		};

		let deliver_tx_result = &response.tx_result;
		if deliver_tx_result.code.is_err() {
			Err(Error::from(format!(
				"Transaction failed with code {:?} and log {:?}",
				deliver_tx_result.code, deliver_tx_result.log
			)))
		} else {
			Ok(response)
		}
	}
}

fn increment_proof_height(
	height: Option<ibc_proto::ibc::core::client::v1::Height>,
) -> Option<ibc_proto::ibc::core::client::v1::Height> {
	height.map(|height| ibc_proto::ibc::core::client::v1::Height {
		revision_height: height.revision_height + 1,
		..height
	})
}
