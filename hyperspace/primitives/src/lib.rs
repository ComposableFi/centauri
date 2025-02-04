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

#![allow(clippy::all)]

use futures::Stream;
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::QueryConnectionResponse,
	},
};
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, HashSet},
	fmt::Debug,
	pin::Pin,
	str::FromStr,
	sync::{Arc, Mutex},
	time::{Duration, SystemTime},
};
use tokio::{sync::Mutex as AsyncMutex, task::JoinSet, time::sleep};

use crate::error::Error;
#[cfg(any(feature = "testing", test))]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{
			client_consensus::ConsensusState as ConsensusStateT,
			client_state::{ClientState as ClientStateT, ClientType},
			events::UpdateClient,
		},
		ics04_channel::{
			channel::{ChannelEnd, Order},
			context::calculate_block_delay,
			packet::{Packet, Sequence},
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::ibc::core::{
	channel::v1::QueryChannelsResponse, connection::v1::IdentifiedConnection,
};
use ibc_rpc::PacketInfo;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};

pub mod error;
pub mod mock;
pub mod utils;

pub enum UpdateMessage {
	Single(Any),
	Batch(Vec<Any>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateType {
	// contains an authority set change.
	Mandatory,
	// doesn't contain an authority set change
	Optional,
}

impl UpdateType {
	pub fn is_optional(&self) -> bool {
		match self {
			UpdateType::Mandatory => false,
			UpdateType::Optional => true,
		}
	}
}

fn default_skip_optional_client_updates() -> bool {
	true
}

fn max_packets_to_process() -> u32 {
	50
}

// TODO: move other fields like `client_id`, `connection_id`, etc. here
/// Common relayer parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonClientConfig {
	/// Skip optional client updates
	#[serde(default = "default_skip_optional_client_updates")]
	pub skip_optional_client_updates: bool,
	#[serde(default = "max_packets_to_process")]
	pub max_packets_to_process: u32,
	/// Minimal time that should pass between two client updates
	#[serde(default)]
	pub client_update_interval_sec: u32,

	#[serde(default)]
	pub handshake_completed: bool,
}

impl Default for CommonClientConfig {
	fn default() -> Self {
		Self {
			skip_optional_client_updates: default_skip_optional_client_updates(),
			max_packets_to_process: max_packets_to_process(),
			client_update_interval_sec: 10,
			handshake_completed: false,
		}
	}
}

/// A common data that all clients should keep.
#[derive(Debug, Clone)]
pub struct CommonClientState {
	/// Enable skipping client updates when possible.
	pub skip_optional_client_updates: bool,
	/// Used to determine whether client updates should be forced to send
	/// even if it's optional. It's required, because some timeout packets
	/// should use proof of the client states.
	///
	/// Set inside `on_undelivered_sequences`.
	pub maybe_has_undelivered_packets: Arc<Mutex<HashMap<UndeliveredType, bool>>>,
	/// Delay between parallel RPC calls to be friendly with the node and avoid MaxSlotsExceeded
	/// error
	pub rpc_call_delay: Duration,
	/// Initial value for the [`rpc_call_delay`] to reset it after a successful RPC call
	pub initial_rpc_call_delay: Duration,
	pub misbehaviour_client_msg_queue: Arc<AsyncMutex<Vec<AnyClientMessage>>>,
	pub max_packets_to_process: usize,
	pub ignored_timeouted_sequences: Arc<AsyncMutex<HashSet<u64>>>,
	/// Minimal time that should pass between two client updates
	pub client_update_interval: Duration,
	/// Last time when client was updated
	pub last_client_update_time: SystemTime,
	/// Flag which provides information if handshake is completed
	///
	/// Used to prevent finding proof for client state, connection state and channel state
	/// once the handshake is completed.
	pub handshake_completed: bool,
}

impl Default for CommonClientState {
	fn default() -> Self {
		let rpc_call_delay = Duration::from_millis(1000);
		Self {
			skip_optional_client_updates: true,
			maybe_has_undelivered_packets: Default::default(),
			rpc_call_delay,
			initial_rpc_call_delay: rpc_call_delay,
			misbehaviour_client_msg_queue: Arc::new(Default::default()),
			max_packets_to_process: 10,
			ignored_timeouted_sequences: Arc::new(Default::default()),
			client_update_interval: Default::default(),
			last_client_update_time: SystemTime::now(),
			handshake_completed: false,
		}
	}
}

impl CommonClientState {
	pub fn from_config(config: &CommonClientConfig) -> Self {
		Self {
			skip_optional_client_updates: config.skip_optional_client_updates,
			maybe_has_undelivered_packets: Default::default(),
			rpc_call_delay: Duration::from_millis(10),
			initial_rpc_call_delay: Duration::from_millis(10),
			misbehaviour_client_msg_queue: Arc::new(Default::default()),
			max_packets_to_process: config.max_packets_to_process as usize,
			ignored_timeouted_sequences: Arc::new(Default::default()),
			client_update_interval: Duration::from_secs(config.client_update_interval_sec.into()),
			last_client_update_time: SystemTime::now(),
			handshake_completed: config.handshake_completed,
		}
	}

	pub async fn on_undelivered_sequences(&self, has: bool, kind: UndeliveredType) {
		log::trace!(
			target: "hyperspace",
			"on_undelivered_sequences: {:?}, type: {kind:?}",
			has
		);
		self.maybe_has_undelivered_packets.lock().unwrap().insert(kind, has);
	}

	pub fn has_undelivered_sequences(&self, kind: UndeliveredType) -> bool {
		self.maybe_has_undelivered_packets
			.lock()
			.unwrap()
			.get(&kind)
			.as_deref()
			.cloned()
			.unwrap_or_default()
	}

	pub fn rpc_call_delay(&self) -> Duration {
		self.rpc_call_delay
	}

	pub fn set_rpc_call_delay(&mut self, delay: Duration) {
		self.rpc_call_delay = delay;
	}
}

pub fn apply_prefix(mut commitment_prefix: Vec<u8>, path: impl Into<Vec<u8>>) -> Vec<u8> {
	let path = path.into();
	commitment_prefix.extend_from_slice(&path);
	commitment_prefix
}

/// A type of undelivered sequences (packets). Can be:
/// - acknowledgement packet (`Acks`),
/// - receive packet (`Recvs`)
/// - timeout packet (`Timeouts`)
/// - proofs for the packets (`Proofs`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UndeliveredType {
	Acks,
	Recvs,
	Timeouts,
}

/// Provides an interface for accessing new events and Ibc data on the chain which must be
/// relayed to the counterparty chain.
#[async_trait::async_trait]
pub trait IbcProvider {
	/// Finality event type, passed on to [`Chain::query_latest_ibc_events`]
	type FinalityEvent: Debug + Send + 'static;
	/// A representation of the transaction id for the chain
	type TransactionId: Debug;
	/// Asset Id
	type AssetId: Clone;

	/// Error type, just needs to implement standard error trait.
	type Error: std::error::Error + From<String> + Send + Sync + 'static;

	/// Query the latest ibc events finalized by the recent finality event. Use the counterparty
	/// [`Chain`] to query the on-chain [`ClientState`] so you can scan for new events in between
	/// the client state and the new finality event.
	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
	where
		T: Chain;

	/// Return a stream that yields when new [`IbcEvents`] are parsed from a finality notification
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>>;

	/// Query client consensus state with proof
	/// return the consensus height for the client along with the response
	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
		include_proof: bool,
	) -> Result<QueryConsensusStateResponse, Self::Error>;

	/// Query client state with proof
	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error>;

	/// Query connection end with proof
	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error>;

	/// Query channel end with proof
	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error>;

	/// Query proof for provided key path
	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error>;

	/// Query packet commitment with proof
	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error>;

	/// Query packet acknowledgement commitment with proof
	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error>;

	/// Query next sequence to be received
	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error>;

	/// Query packet receipt
	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error>;

	/// Return latest finalized height and timestamp specifically for the chain.
	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error>;

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error>;

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error>;

	/// Given a list of counterparty packet commitments, the querier checks if the packet
	/// has already been received by checking if a receipt exists on this
	/// chain for the packet sequence. All packets that haven't been received yet
	/// are returned in the response
	/// Usage: To use this method correctly, first query all packet commitments on
	/// the sending chain using the query_packet_commitments method.
	/// and send the request to this Query/UnreceivedPackets on the **receiving**
	/// chain. This method should then return the list of packet sequences that
	/// are yet to be received on the receiving chain.
	/// NOTE: WORKS ONLY FOR UNORDERED CHANNELS
	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error>;

	/// Given a list of packet acknowledgements sequences from the sink chain
	/// return a list of acknowledgement sequences that have not been received on the source chain
	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error>;

	/// Channel whitelist
	fn channel_whitelist(&self) -> HashSet<(ChannelId, PortId)>;

	/// Query all channels for a connection
	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error>;

	/// Query send packets
	/// This represents packets that for which the `SendPacket` event was emitted
	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error>;

	/// Query received packets with their acknowledgement
	/// This represents packets for which the `ReceivePacket` and `WriteAcknowledgement` events were
	/// emitted.
	async fn query_received_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error>;

	/// Return the expected block time for this chain
	fn expected_block_time(&self) -> Duration;

	/// Query the time and height at which this client was updated on this chain for the given
	/// client height
	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error>;

	/// Return a proof for the host consensus state at the given height to be included in the
	/// consensus state proof.
	async fn query_host_consensus_state_proof(
		&self,
		client_state: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error>;

	/// Should return the list of ibc denoms available to this account to spend.
	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error>;

	/// Return the chain connection prefix
	fn connection_prefix(&self) -> CommitmentPrefix;

	/// Return the host chain's light client id on counterparty chain
	fn client_id(&self) -> ClientId;

	/// Set the client id for the relayer task.
	fn set_client_id(&mut self, client_id: ClientId);

	/// Return the connection id on this chain
	fn connection_id(&self) -> Option<ConnectionId>;

	/// Set the channel whitelist for the relayer task.
	fn set_channel_whitelist(&mut self, channel_whitelist: HashSet<(ChannelId, PortId)>);

	/// Set the channel whitelist for the relayer task.
	fn add_channel_to_whitelist(&mut self, channel: (ChannelId, PortId));

	/// Set the connection id for the relayer task.
	fn set_connection_id(&mut self, connection_id: ConnectionId);

	/// Returns the client type of this chain.
	fn client_type(&self) -> ClientType;

	/// Should return timestamp in nanoseconds of chain at a given block height
	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error>;

	/// Should return a list of all clients on the chain
	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error>;

	/// Should return a list of all clients on the chain
	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error>;

	/// Query all connection states for associated client
	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error>;

	/// Returns a boolean value that determines if the light client should receive a mandatory
	/// update
	async fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error>;

	/// This should return a subjectively chosen client and consensus state for this chain.
	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error>;

	/// Should find client id that was created in this transaction
	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error>;

	/// Should find connection id that was created in this transaction
	async fn query_connection_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ConnectionId, Self::Error>;

	/// Should find channel and port id that was created in this transaction
	async fn query_channel_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<(ChannelId, PortId), Self::Error>;

	async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error>;

	async fn fast_proof_search(
		&self,
		_client_id: ClientId,
		_delay_period: Duration,
	) -> Option<Height> {
		None
	}
}

/// Provides an interface that allows us run the hyperspace-testsuite
/// with [`Chain`] implementations.
#[cfg(any(feature = "testing", test))]
#[async_trait::async_trait]
pub trait TestProvider: Chain + Clone + 'static {
	/// Initiate an ibc transfer on chain.
	async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error>;

	/// Send a packet on an ordered channel
	async fn send_ordered_packet(
		&self,
		channel_id: ChannelId,
		timeout: pallet_ibc::Timeout,
	) -> Result<(), Self::Error>;

	/// Returns a stream that yields chain Block number
	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>>;

	/// Increases IBC counters by 1 to check that relayer uses proper values for source/sink chains.
	async fn increase_counters(&mut self) -> Result<(), Self::Error>;
}

/// Provides an interface for managing key management for signing.
pub trait KeyProvider {
	/// Should return the relayer's account id on the host chain as a string in the expected format
	/// Could be a hexadecimal, bech32 or ss58 string, any format the chain supports
	fn account_id(&self) -> Signer;
}

/// Provides an interface for managing IBC misbehaviour.
#[async_trait::async_trait]
pub trait MisbehaviourHandler {
	/// Check the client message for misbehaviour and submit it to the chain if any.
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error>;
}

/// Provides an interface for syncing light clients to the latest state
#[async_trait::async_trait]
pub trait LightClientSync {
	/// Checks if the self's light client on counterparty is synced
	async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, anyhow::Error>;

	/// Get all the messages from self required to update self's light client on the counterparty
	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error>;
}

/// Provides an interface for the chain to the relayer core for submitting IbcEvents as well as
/// finality notifications
#[async_trait::async_trait]
pub trait Chain:
	IbcProvider + LightClientSync + MisbehaviourHandler + KeyProvider + Clone + Send + Sync + 'static
{
	/// Name of this chain, used in logs.
	fn name(&self) -> &str;

	/// Should return a numerical value for the max weight of transactions allowed in a block.
	fn block_max_weight(&self) -> u64;

	/// Should return an estimate of the weight of a batch of messages.
	async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error>;

	/// Return a stream that yields when new [`IbcEvents`] are ready to be queried.
	async fn finality_notifications(
		&self,
	) -> Result<Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>>, Self::Error>;

	/// This should be used to submit new messages [`Vec<Any>`] from a counterparty chain to this
	/// chain.
	/// Should return the transaction id
	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Self::Error>;

	/// Returns an [`AnyClientMessage`] for an [`UpdateClient`] event
	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error>;

	async fn get_proof_height(&self, block_height: Height) -> Height;

	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error>;

	fn common_state(&self) -> &CommonClientState;

	fn common_state_mut(&mut self) -> &mut CommonClientState;

	async fn on_undelivered_sequences(&self, has: bool, kind: UndeliveredType) {
		self.common_state().on_undelivered_sequences(has, kind).await
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

	async fn reconnect(&mut self) -> anyhow::Result<()>;
}

/// Returns undelivered packet sequences that have been sent out from
/// the `source` chain to the `sink` chain
/// works for both ordered and unordered channels
pub async fn query_undelivered_sequences(
	source_height: Height,
	sink_height: Height,
	channel_id: ChannelId,
	port_id: PortId,
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<u64>, anyhow::Error> {
	let channel_response =
		source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
	let channel_end = ChannelEnd::try_from(
		channel_response
			.channel
			.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
	)
	.map_err(|e| Error::Custom(e.to_string()))?;
	// First we fetch all packet commitments from source
	// let _ignored_timeouts = source.common_state().ignored_timeouted_sequences.lock().await;
	let seqs = source
		.query_packet_commitments(source_height, channel_id, port_id.clone())
		.await?
		.into_iter()
		// .filter(|seq| !ignored_timeouts.contains(seq))
		.collect::<Vec<_>>();
	log::trace!(target: "hyperspace", "Seqs: {:?}", seqs);

	let counterparty_channel_id = channel_end
		.counterparty()
		.channel_id
		.ok_or_else(|| Error::Custom("Expected counterparty channel id".to_string()))?;
	let counterparty_port_id = channel_end.counterparty().port_id.clone();

	let undelivered_sequences = if channel_end.ordering == Order::Unordered {
		sink.query_unreceived_packets(
			sink_height,
			counterparty_channel_id,
			counterparty_port_id.clone(),
			seqs,
		)
		.await?
	} else {
		let next_seq_recv = sink
			.query_next_sequence_recv(sink_height, &counterparty_port_id, &counterparty_channel_id)
			.await?
			.next_sequence_receive;
		seqs.into_iter().filter(|seq| *seq > next_seq_recv).collect()
	};

	Ok(undelivered_sequences)
}

/// Queries the `source` chain for packet acknowledgements that have not been seen by the `sink`
/// chain.
pub async fn query_undelivered_acks(
	source_height: Height,
	sink_height: Height,
	channel_id: ChannelId,
	port_id: PortId,
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<u64>, anyhow::Error> {
	let channel_response =
		source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
	let channel_end = ChannelEnd::try_from(
		channel_response
			.channel
			.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
	)
	.map_err(|e| Error::Custom(e.to_string()))?;
	let counterparty_channel_id = channel_end
		.counterparty()
		.channel_id
		.ok_or_else(|| Error::Custom("Expected counterparty channel id".to_string()))?;
	let counterparty_port_id = channel_end.counterparty().port_id.clone();

	// First we fetch all packet acknowledgements from source
	let seqs = source
		.query_packet_acknowledgements(source_height, channel_id, port_id.clone())
		.await?;
	log::trace!(
		target: "hyperspace",
		"Found {} packet acks from {} chain",
		seqs.len(), source.name()
	);

	let mut undelivered_acks = sink
		.query_unreceived_acknowledgements(
			sink_height,
			counterparty_channel_id,
			counterparty_port_id.clone(),
			seqs,
		)
		.await?;
	log::trace!(
		target: "hyperspace",
		"Found {} undelivered packet acks for {} chain",
		undelivered_acks.len(), sink.name()
	);
	undelivered_acks.sort();
	undelivered_acks.dedup();

	Ok(undelivered_acks)
}

pub fn packet_info_to_packet(packet_info: &PacketInfo) -> Packet {
	Packet {
		sequence: packet_info.sequence.into(),
		source_port: PortId::from_str(&packet_info.source_port).expect("Port should be valid"),
		source_channel: ChannelId::from_str(&packet_info.source_channel)
			.expect("Channel should be valid"),
		destination_port: PortId::from_str(&packet_info.destination_port)
			.expect("Port should be valid"),
		destination_channel: ChannelId::from_str(&packet_info.destination_channel)
			.expect("Channel should be valid"),
		data: packet_info.data.clone(),
		timeout_height: packet_info.timeout_height.clone().into(),
		timeout_timestamp: Timestamp::from_nanoseconds(packet_info.timeout_timestamp)
			.expect("Timestamp should be valid"),
	}
}

/// Should return the first client consensus height with a consensus state timestamp that
/// is equal to or greater than the values provided
pub async fn find_suitable_proof_height_for_client(
	source: &impl Chain,
	sink: &impl Chain,
	at: Height,
	client_id: ClientId,
	start_height: Height,
	height_to_match: Option<Height>,
	timestamp_to_match: Option<Timestamp>,
	latest_client_height: Height,
	source_delay_period: Duration,
) -> Option<Height> {
	log::info!(
		target: "hyperspace",
		"Searching for suitable proof height for client {} ({}) starting at {}, {:?}, latest_client_height={}, at = {at}",
		client_id, sink.name(), start_height, timestamp_to_match, latest_client_height
	);
	let start_height = source.get_proof_height(start_height).await;

	if let Some(proof_height) = sink.fast_proof_search(client_id.clone(), source_delay_period).await
	{
		if proof_height < start_height {
			info!(
				"Fast proof search found proof height on {} as {} which is less than start height {}",
				sink.name(),
				proof_height,
				start_height
			);
			return None;
		}
		info!("Fast proof search found proof height on {} as {}", sink.name(), proof_height);
		return Some(proof_height)
	}

	// We use pure linear search because there's no valid comparison to be made and there might be
	// missing values  for some heights
	for height in start_height.revision_height..=latest_client_height.revision_height {
		log::info!(target: "hyperspace", "Searching for proof height: {}", height);
		let mut temp_height = Height::new(start_height.revision_number, height);

		if sink
			.query_client_update_time_and_height(client_id.clone(), temp_height)
			.await
			.map_err(|e| log::error!("Error querying client update time and height: {}", e))
			.ok()
			.is_none()
		{
			continue;
		}

		let consensus_state = sink
			.query_client_consensus(at, client_id.clone(), temp_height, false)
			.await
			.ok();
		let decoded = consensus_state
			.map(|x| x.consensus_state.map(AnyConsensusState::try_from))
			.flatten();
		if !matches!(decoded, Some(Ok(_))) {
			continue;
		}
		let mut matches = false;
		if let Some(timestamp_to_match) = &timestamp_to_match {
			let consensus_state = decoded.unwrap().unwrap();

			if consensus_state.timestamp().nanoseconds() >= timestamp_to_match.nanoseconds() {
				matches = true;
			}
		}
		if let Some(height_to_match) = &height_to_match {
			if temp_height >= *height_to_match {
				matches = true;
			}
		}
		if !matches {
			continue;
		}

		let proof_height = source.get_proof_height(temp_height).await;
		if proof_height != temp_height {
			temp_height.revision_height -=
				proof_height.revision_height - temp_height.revision_height;
		}
		info!("Found proof height on {} as {}:{}", sink.name(), temp_height, proof_height);
		return Some(temp_height);
	}
	None
}

// TODO: query_maximum_height_for_timeout_proofs: return Result<Option<Height>, _>
pub async fn query_maximum_height_for_timeout_proofs(
	source: &impl Chain,
	sink: &impl Chain,
) -> Option<Height> {
	let (source_height, ..) = source.latest_height_and_timestamp().await.ok()?;
	let (sink_height, ..) = sink.latest_height_and_timestamp().await.ok()?;
	let mut join_set: JoinSet<Option<_>> = JoinSet::new();
	for (channel, port_id) in source.channel_whitelist() {
		let undelivered_sequences = query_undelivered_sequences(
			source_height,
			sink_height,
			channel,
			port_id.clone(),
			source,
			sink,
		)
		.await
		.ok()?;
		let undelivered_sequences = undelivered_sequences
			.into_iter()
			.rev()
			.take(source.common_state().max_packets_to_process)
			.collect();
		let send_packets =
			source.query_send_packets(channel, port_id, undelivered_sequences).await.ok()?;
		for send_packet in send_packets {
			let source = source.clone();
			let sink = sink.clone();
			let duration = Duration::from_millis(
				rand::thread_rng().gen_range(1..source.rpc_call_delay().as_millis()) as u64,
			);
			join_set.spawn(async move {
				sleep(duration).await;
				let revision_height = send_packet.height.expect("expected height for packet");
				let sink_client_state = source
					.query_client_state(
						Height::new(source_height.revision_number, revision_height),
						sink.client_id(),
					)
					.await
					.ok()?;
				let sink_client_state =
					AnyClientState::try_from(sink_client_state.client_state?).ok()?;
				let height = sink_client_state.latest_height();
				let timestamp_at_creation =
					sink.query_timestamp_at(height.revision_height).await.ok()?;
				let period = send_packet.timeout_timestamp.saturating_sub(timestamp_at_creation);
				if period == 0 {
					return Some(Height::from(send_packet.timeout_height));
				}
				let period = Duration::from_nanos(period);
				let period =
					calculate_block_delay(period, sink.expected_block_time()).saturating_add(1);
				let approx_height = revision_height + period;
				let timeout_height = if send_packet.timeout_height.revision_height < approx_height {
					Height::from(send_packet.timeout_height)
				} else {
					Height::new(send_packet.timeout_height.revision_number, approx_height)
				};

				Some(timeout_height)
			});
		}
	}
	let mut max_timeout_height = None;
	while let Some(timeout_height) = join_set.join_next().await {
		max_timeout_height = max_timeout_height.max(timeout_height.ok()?)
	}
	max_timeout_height
}

pub fn filter_events_by_ids(
	ev: &IbcEvent,
	client_ids: &[ClientId],
	connection_ids: &[ConnectionId],
	channel_and_port_ids: &HashSet<(ChannelId, PortId)>,
) -> bool {
	use ibc::core::{
		ics02_client::events::Attributes as ClientAttributes,
		ics03_connection::events::Attributes as ConnectionAttributes,
		ics04_channel::events::Attributes as ChannelAttributes,
	};
	let channel_ids = channel_and_port_ids
		.iter()
		.map(|(channel_id, _)| channel_id)
		.collect::<Vec<_>>();

	let filter_packet = |packet: &Packet| {
		channel_and_port_ids.contains(&(packet.source_channel.clone(), packet.source_port.clone())) ||
			channel_and_port_ids
				.contains(&(packet.destination_channel.clone(), packet.destination_port.clone()))
	};
	let filter_client_attributes =
		|packet: &ClientAttributes| client_ids.contains(&packet.client_id);
	let filter_connection_attributes = |packet: &ConnectionAttributes| {
		packet
			.connection_id
			.as_ref()
			.map(|id| connection_ids.contains(&id))
			.unwrap_or(false) ||
			packet
				.counterparty_connection_id
				.as_ref()
				.map(|id| connection_ids.contains(&id))
				.unwrap_or(false)
	};
	let filter_channel_attributes = |packet: &ChannelAttributes| {
		packet.channel_id.as_ref().map(|id| channel_ids.contains(&id)).unwrap_or(false) ||
			packet
				.counterparty_channel_id
				.as_ref()
				.map(|id| channel_ids.contains(&id))
				.unwrap_or(false)
	};

	let v = match ev {
		IbcEvent::SendPacket(e) => filter_packet(&e.packet),
		IbcEvent::WriteAcknowledgement(e) => filter_packet(&e.packet),
		IbcEvent::TimeoutPacket(e) => filter_packet(&e.packet),
		IbcEvent::ReceivePacket(e) => filter_packet(&e.packet),
		IbcEvent::AcknowledgePacket(e) => filter_packet(&e.packet),
		IbcEvent::TimeoutOnClosePacket(e) => filter_packet(&e.packet),
		IbcEvent::CreateClient(e) => filter_client_attributes(&e.0),
		IbcEvent::UpdateClient(e) => filter_client_attributes(&e.common),
		IbcEvent::UpgradeClient(e) => filter_client_attributes(&e.0),
		IbcEvent::ClientMisbehaviour(e) => filter_client_attributes(&e.0),
		IbcEvent::OpenInitConnection(e) => filter_connection_attributes(&e.0),
		IbcEvent::OpenTryConnection(e) => filter_connection_attributes(&e.0),
		IbcEvent::OpenAckConnection(e) => filter_connection_attributes(&e.0),
		IbcEvent::OpenConfirmConnection(e) => filter_connection_attributes(&e.0),
		IbcEvent::OpenInitChannel(e) =>
			filter_channel_attributes(&ChannelAttributes::from(e.clone())),
		IbcEvent::OpenTryChannel(e) =>
			filter_channel_attributes(&ChannelAttributes::from(e.clone())),
		IbcEvent::OpenAckChannel(e) =>
			filter_channel_attributes(&ChannelAttributes::from(e.clone())),
		IbcEvent::OpenConfirmChannel(e) =>
			filter_channel_attributes(&ChannelAttributes::from(e.clone())),
		IbcEvent::CloseInitChannel(e) =>
			filter_channel_attributes(&ChannelAttributes::from(e.clone())),
		IbcEvent::CloseConfirmChannel(e) =>
			filter_channel_attributes(&ChannelAttributes::from(e.clone())),
		IbcEvent::PushWasmCode(_) => true,
		IbcEvent::NewBlock(_) |
		IbcEvent::AppModule(_) |
		IbcEvent::Empty(_) |
		IbcEvent::ChainError(_) => true,
	};
	if !v {
		log::debug!(target: "hyperspace", "Filtered out event: {:?}", ev);
	}
	v
}
