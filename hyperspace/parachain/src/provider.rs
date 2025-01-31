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

use super::{error::Error, ParachainClient};
use crate::{finality_protocol::FinalityEvent, FinalityProtocol, GrandpaClientState};
use beefy_prover::helpers::fetch_timestamp_extrinsic_with_proof;
use codec::{Decode, Encode};
use finality_grandpa::BlockNumberOps;
use futures::Stream;
use grandpa_light_client_primitives::ParachainHeaderProofs;
use ibc::{
	applications::transfer::{Amount, PrefixedCoin, PrefixedDenom},
	core::{
		ics02_client::client_state::{ClientState, ClientType},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			IdentifiedChannel, QueryChannelResponse, QueryChannelsResponse,
			QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
			QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
		},
		client::v1::{
			IdentifiedClientState, QueryClientStateResponse, QueryConsensusStateResponse,
		},
		connection::v1::{IdentifiedConnection, QueryConnectionResponse},
	},
};
use ibc_rpc::{IbcApiClient, PacketInfo};
use ics11_beefy::client_state::ClientState as BeefyClientState;
use light_client_common::config::{AsInnerEvent, IbcEventsT, RuntimeStorage};
use pallet_ibc::{
	light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager},
	HostConsensusProof,
};
use primitives::{apply_prefix, Chain, IbcProvider, KeyProvider, UpdateType};
use sp_core::H256;
use sp_runtime::{
	traits::{IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use std::{
	collections::{BTreeMap, HashSet},
	fmt::Display,
	pin::Pin,
	str::FromStr,
	time::Duration,
};
use subxt::config::{
	extrinsic_params::BaseExtrinsicParamsBuilder, ExtrinsicParams, Header as HeaderT, Header,
};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug)]
pub struct TransactionId<Hash> {
	pub ext_hash: Hash,
	pub block_hash: Hash,
}

#[async_trait::async_trait]
impl<T: light_client_common::config::Config + Send + Sync + Clone> IbcProvider
	for ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	u32: From<<<T as subxt::Config>::Header as Header>::Number>,
	Self: KeyProvider,
	<<T as light_client_common::config::Config>::Signature as Verify>::Signer:
		From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<MultiSignature> + Send + Sync,
	<<T as subxt::Config>::Header as Header>::Number: BlockNumberOps
		+ From<u32>
		+ Display
		+ Ord
		+ sp_runtime::traits::Zero
		+ One
		+ Send
		+ Sync
		+ Clone,
	<T as subxt::Config>::Header: Decode + Send + Sync + Clone,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	sp_core::H256: From<T::Hash>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, T::Tip>> + Send + Sync,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
	<T as light_client_common::config::Config>::AssetId: Clone,
{
	type FinalityEvent = FinalityEvent;
	type TransactionId = TransactionId<T::Hash>;
	type Error = Error;
	type AssetId = <T as light_client_common::config::Config>::AssetId;

	async fn query_latest_ibc_events<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
	where
		C: Chain,
	{
		self.finality_protocol
			.clone()
			.query_latest_ibc_events(self, finality_event, counterparty)
			.await
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		use futures::StreamExt;

		let (tx, rx) = tokio::sync::mpsc::channel(32);
		let event = self.para_client.events();
		let para_client = self.para_client.clone();
		tokio::spawn(async move {
			let stream = para_client
				.blocks()
				.subscribe_all()
				.await
				.expect("should susbcribe to blocks")
				.filter_map(|block| async {
					let block = block.ok()?;
					let hash = block.hash();
					let events = event.at(hash).await.ok()?;
					let result = events
						.find::<<T::Events as AsInnerEvent>::Inner>()
						.filter_map(|ev| {
							let ok_event = ev
								.map_err(|e| {
									log::error!(target: "hyperspace_parachain", "Error event at block {hash:?}: {:?}", e);
								})
								.ok()?;
							let ev = <T::Events as AsInnerEvent>::from_inner(ok_event).events();
							ev.into_iter()
								.map(|ev| TryInto::<IbcEvent>::try_into(ev))
								.collect::<Result<Vec<_>, _>>()
								.ok()
						})
						.flatten()
						.collect::<Vec<_>>();
					Some(result)
				});

			let mut stream = Box::pin(stream);

			while let Some(evs) = stream.next().await {
				let mut should_exit = false;
				for ev in evs {
					if let Err(_) = tx.send(ev).await {
						should_exit = true;
						break
					}
				}
				if should_exit {
					break
				}
			}
		});

		Box::pin(ReceiverStream::new(rx))
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
		include_proof: bool,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_client_consensus_state(
			&*self.para_ws_client,
			Some(at.revision_height as u32),
			client_id.to_string(),
			consensus_height.revision_height,
			consensus_height.revision_number,
			false,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let response =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_client_state(
				&*self.para_ws_client,
				at.revision_height as u32,
				client_id.to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_connection(
			&*self.para_ws_client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_channel(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	/// Query the proof of the given keys at the given height.
	///
	/// Note: all the keys will be prefixed with the connection prefix.
	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let prefix = self.connection_prefix().into_vec();
		let prefixed_keys =
			keys.into_iter().map(|path| apply_prefix(prefix.clone(), path)).collect();

		let proof = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_proof(
			&*self.para_ws_client,
			at.revision_height as u32,
			prefixed_keys,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(proof.proof)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let res =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_packet_commitment(
				&*self.para_ws_client,
				at.revision_height as u32,
				channel_id.to_string(),
				port_id.to_string(),
				seq,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_packet_acknowledgement(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_next_seq_recv(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_packet_receipt(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		let finalized_header = self
			.para_client
			.rpc()
			.header(None)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let latest_height: u64 = (finalized_header.number()).into();
		let height = Height::new(self.para_id.into(), latest_height.into());

		let subxt_block_number: subxt::rpc::types::BlockNumber = latest_height.into();
		let block_hash =
			self.para_client.rpc().block_hash(Some(subxt_block_number)).await?.ok_or_else(
				|| Error::Custom("Latest block hash query returned None".to_string()),
			)?;
		let timestamp_addr = T::Storage::timestamp_now();
		let unix_timestamp_millis = self
			.para_client
			.storage()
			.at(block_hash)
			.fetch(&timestamp_addr)
			.await?
			.ok_or_else(|| Error::from("Timestamp should exist".to_string()))?;
		let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

		Ok((height, Timestamp::from_nanoseconds(timestamp_nanos)?))
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_packet_commitments(
				&*self.para_ws_client,
				at.revision_height as u32,
				channel_id.to_string(),
				port_id.to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res.commitments.into_iter().map(|packet_state| packet_state.sequence).collect())
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_packet_acknowledgements(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res
			.acknowledgements
			.into_iter()
			.map(|packet_state| packet_state.sequence)
			.collect())
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let res =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_unreceived_packets(
				&*self.para_ws_client,
				at.revision_height as u32,
				channel_id.to_string(),
				port_id.to_string(),
				seqs,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		log::trace!(
			target: "hyperspace_parachain",
			"query_unreceived_acknowledgements at: {:?}, channel_id: {:?}, port_id: {:?}, seqs: {:?}",
			at,
			channel_id,
			port_id,
			seqs
		);
		let res = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_unreceived_acknowledgements(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	fn channel_whitelist(&self) -> HashSet<(ChannelId, PortId)> {
		self.channel_whitelist.lock().unwrap().iter().cloned().collect()
	}

	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		let response =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_connection_channels(
				&*self.para_ws_client,
				at.revision_height as u32,
				connection_id.to_string(),
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_send_packets(
				&*self.para_ws_client,
				channel_id.to_string(),
				port_id.to_string(),
				seqs,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		Ok(response)
	}

	async fn query_received_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response =
			IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_recv_packets(
				&*self.para_ws_client,
				channel_id.to_string(),
				port_id.to_string(),
				seqs,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	fn expected_block_time(&self) -> Duration {
		// Parachains have an expected block time of 12 seconds
		Duration::from_secs(12)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		log::trace!(
			target: "hyperspace_parachain",
			"Querying client update time and height for client {:?} at height {:?}",
			client_id,
			client_height
		);
		let response = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_client_update_time_and_height(
			&*self.para_ws_client,
			client_id.to_string(),
			client_height.revision_number,
			client_height.revision_height,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok((
			response.height.into(),
			Timestamp::from_nanoseconds(response.timestamp)
				.map_err(|_| Error::Custom("Received invalid timestamp".to_string()))?,
		))
	}

	async fn query_host_consensus_state_proof(
		&self,
		client_state: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		let hash = self
			.para_client
			.rpc()
			.block_hash(Some(client_state.latest_height().revision_height.into()))
			.await?;
		let header = self
			.para_client
			.rpc()
			.header(hash)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let extrinsic_with_proof =
			fetch_timestamp_extrinsic_with_proof(&self.para_client, Some(header.hash()))
				.await
				.map_err(Error::BeefyProver)?;
		let code_id = if let AnyClientState::Wasm(client_state) = &client_state {
			Some(client_state.code_id.clone())
		} else {
			None
		};
		let host_consensus_proof = HostConsensusProof {
			header: header.encode(),
			extrinsic: extrinsic_with_proof.ext,
			extrinsic_proof: extrinsic_with_proof.proof,
			code_id,
		};
		Ok(Some(host_consensus_proof.encode()))
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		let account = self.public_key.clone().into_account();
		let account = subxt::utils::AccountId32::from(<[u8; 32]>::from(account));
		let mut hex_string = hex::encode(account.0.to_vec());
		hex_string.insert_str(0, "0x");
		let coin: ibc_proto::cosmos::base::v1beta1::Coin = IbcApiClient::<
			u32,
			H256,
			<T as light_client_common::config::Config>::AssetId,
		>::query_balance_with_address(
			&*self.para_ws_client,
			hex_string,
			asset_id,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom::from_str(&coin.denom)?,
			amount: Amount::from_str(&coin.amount)?,
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
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

	fn client_type(&self) -> ClientType {
		match self.finality_protocol {
			FinalityProtocol::Grandpa => GrandpaClientState::<HostFunctionsManager>::client_type(),
			FinalityProtocol::Beefy => BeefyClientState::<HostFunctionsManager>::client_type(),
		}
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let subxt_block_number: subxt::rpc::types::BlockNumber = block_number.into();
		let block_hash =
			self.para_client.rpc().block_hash(Some(subxt_block_number)).await?.ok_or_else(
				|| Error::Custom("Block hash not found for block number".to_string()),
			)?;
		let timestamp_addr = T::Storage::timestamp_now();
		let unix_timestamp_millis = self
			.para_client
			.storage()
			.at(block_hash)
			.fetch(&timestamp_addr)
			.await?
			.expect("Timestamp should exist");
		let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

		Ok(timestamp_nanos)
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let response: Vec<IdentifiedClientState> = IbcApiClient::<
			u32,
			H256,
			<T as light_client_common::config::Config>::AssetId,
		>::query_clients(&*self.para_ws_client)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		response
			.into_iter()
			.map(|client| {
				ClientId::from_str(&client.client_id)
					.map_err(|_| Error::Custom("Invalid client id ".to_string()))
			})
			.collect()
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let response = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_channels(
			&*self.para_ws_client,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		response
			.channels
			.into_iter()
			.map(|identified_chan| {
				Ok((
					ChannelId::from_str(&identified_chan.channel_id)
						.expect("Failed to convert invalid string to channel id"),
					PortId::from_str(&identified_chan.port_id)
						.expect("Failed to convert invalid string to port id"),
				))
			})
			.collect::<Result<Vec<_>, _>>()
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let response = IbcApiClient::<u32, H256, <T as light_client_common::config::Config>::AssetId>::query_connection_using_client(
			&*self.para_ws_client,
			height,
			client_id,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		Ok(response)
	}

	async fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		let prover = self.grandpa_prover();
		let session_length = prover
			.session_length()
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		// We divide the session into some places and if the diff in block updates is greater than
		// this update is required
		let base =
			if cfg!(test) { (session_length / 2) as u64 } else { (session_length / 12) as u64 };
		let diff = latest_height.saturating_sub(latest_client_height_on_counterparty);
		let pruning_len = 256;
		Ok(diff >= base.min(pruning_len as u64))
	}

	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let res = self.construct_grandpa_client_state().await?;
				Ok(res)
			},
			FinalityProtocol::Beefy => {
				let res = self.construct_beefy_client_state().await?;
				Ok(res)
			},
		}
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		// Query newly created client Id
		let TransactionId { ext_hash, block_hash } = tx_id;
		let identified_client_state = IbcApiClient::<
			u32,
			H256,
			<T as light_client_common::config::Config>::AssetId,
		>::query_newly_created_client(
			&*self.para_ws_client, block_hash.into(), ext_hash.into()
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		let client_id = ClientId::from_str(&identified_client_state.client_id)
			.expect("Should have a valid client id");
		Ok(client_id)
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		// Query newly created connection Id
		let TransactionId { ext_hash, block_hash } = tx_id;
		let identified_connection: IdentifiedConnection = IbcApiClient::<
			u32,
			H256,
			<T as light_client_common::config::Config>::AssetId,
		>::query_newly_created_connection(
			&*self.para_ws_client,
			block_hash.into(),
			ext_hash.into(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		let connection_id = ConnectionId::from_str(&identified_connection.id)
			.expect("Should have a valid connection id");
		Ok(connection_id)
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<(ChannelId, PortId), Self::Error> {
		// Query newly created channel Id
		let TransactionId { ext_hash, block_hash } = tx_id;
		let identified_channel: IdentifiedChannel = IbcApiClient::<
			u32,
			H256,
			<T as light_client_common::config::Config>::AssetId,
		>::query_newly_created_channel(
			&*self.para_ws_client,
			block_hash.into(),
			ext_hash.into(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		let channel_id = ChannelId::from_str(&identified_channel.channel_id)
			.expect("Should have a valid channel id");
		let port_id =
			PortId::from_str(&identified_channel.port_id).expect("Should have a valid port id");
		Ok((channel_id, port_id))
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

	async fn upload_wasm(&self, _wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		Err(Error::Custom("Uploading WASM to parachain is not supported".to_string()))
	}
}
