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

#![allow(unreachable_patterns)]

use crate::{
	chains,
	substrate::{
		dali::DaliConfig, default::DefaultConfig, ComposableConfig, PicassoKusamaConfig,
		PicassoPolkadotConfig,
	},
};
use async_trait::async_trait;
#[cfg(feature = "cosmos")]
use cosmos::client::{CosmosClient, CosmosClientConfig};
use futures::Stream;
#[cfg(any(test, feature = "testing"))]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{client_state::ClientType, events::UpdateClient},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	downcast,
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryChannelsResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{IdentifiedConnection, QueryConnectionResponse},
	},
};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
#[cfg(any(test, feature = "testing"))]
use pallet_ibc::Timeout;
use parachain::{ParachainClient, ParachainClientConfig};
use primitives::{
	Chain, IbcProvider, KeyProvider, LightClientSync, MisbehaviourHandler, UpdateType,
};
use serde::{Deserialize, Serialize};
use std::{pin::Pin, time::Duration};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub chain_a: AnyConfig,
	pub chain_b: AnyConfig,
	pub core: CoreConfig,
}

#[derive(Serialize, Deserialize)]
pub struct CoreConfig {
	pub prometheus_endpoint: Option<String>,
}

impl From<String> for AnyError {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

chains! {
	Parachain(
		ParachainClientConfig,
		ParachainClient<DefaultConfig>,
		parachain::finality_protocol::FinalityEvent,
		parachain::provider::TransactionId<sp_core::H256>,
		<ParachainClient<DefaultConfig> as IbcProvider>::AssetId,
		parachain::error::Error
	),
	Dali(
		ParachainClientConfig,
		ParachainClient<DaliConfig>,
		parachain::finality_protocol::FinalityEvent,
		parachain::provider::TransactionId<sp_core::H256>,
		<ParachainClient<DaliConfig> as IbcProvider>::AssetId,
		parachain::error::Error
	),
	Composable(
		ParachainClientConfig,
		ParachainClient<ComposableConfig>,
		parachain::finality_protocol::FinalityEvent,
		parachain::provider::TransactionId<sp_core::H256>,
		<ParachainClient<ComposableConfig> as IbcProvider>::AssetId,
		parachain::error::Error
	),
	PicassoPolkadot(
		ParachainClientConfig,
		ParachainClient<PicassoPolkadotConfig>,
		parachain::finality_protocol::FinalityEvent,
		parachain::provider::TransactionId<sp_core::H256>,
		<ParachainClient<PicassoPolkadotConfig> as IbcProvider>::AssetId,
		parachain::error::Error
	),
	PicassoKusama(
		ParachainClientConfig,
		ParachainClient<PicassoKusamaConfig>,
		parachain::finality_protocol::FinalityEvent,
		parachain::provider::TransactionId<sp_core::H256>,
		<ParachainClient<PicassoKusamaConfig> as IbcProvider>::AssetId,
		parachain::error::Error
	),
	Cosmos(
		CosmosClientConfig,
		CosmosClient<DefaultConfig>,
		cosmos::provider::FinalityEvent,
		cosmos::provider::TransactionId<cosmos::provider::Hash>,
		<CosmosClient<DefaultConfig> as IbcProvider>::AssetId,
		cosmos::error::Error
	),
}
