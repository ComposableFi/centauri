// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	context::Context,
	error::ContractError,
	log,
	msg::{
		CheckForMisbehaviourMsg, ContractResult, ExportMetadataMsg, QueryMsg, QueryResponse,
		StatusMsg, SudoMsg, UpdateStateMsg, UpdateStateOnMisbehaviourMsg, VerifyClientMessage,
		VerifyMembershipMsg, VerifyNonMembershipMsg, VerifyUpgradeAndUpdateStateMsg,
	},
	state::{get_client_state, get_consensus_state},
	Bytes,
};
use byteorder::{ByteOrder, LittleEndian};
use core::hash::Hasher;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw_storage_plus::{Item, Map};
use digest::Digest;
use grandpa_light_client_primitives::justification::AncestryChain;
use ibc::core::{
	ics02_client::{
		client_def::{ClientDef, ConsensusUpdateResult},
		context::{ClientKeeper, ClientReader},
		height::Height,
	},
	ics24_host::identifier::ClientId,
};
use ibc_proto::google::protobuf::Any;
use ics08_wasm::{instantiate::InstantiateMessage, SUBJECT_PREFIX, SUBSTITUTE_PREFIX};
use ics10_grandpa::{
	client_def::GrandpaClient,
	client_message::{ClientMessage, RelayChainHeader},
	client_state::ClientState,
	consensus_state::ConsensusState,
};
use light_client_common::{verify_membership, verify_non_membership};
use prost::Message;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Header};
use sp_runtime_interface::unpack_ptr_and_len;
use std::{collections::BTreeSet, str::FromStr};
use tendermint_proto::Protobuf;
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ics10-grandpa-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

pub const CHANNELS_CONNECTION: Map<Bytes, Vec<(Bytes, Bytes)>> = Map::new("channels_connection");
pub const CLIENT_UPDATE_TIME: Map<(Bytes, Bytes), u64> = Map::new("client_update_time");
pub const CLIENT_UPDATE_HEIGHT: Map<(Bytes, Bytes), Bytes> = Map::new("client_update_height");
pub const CHANNEL_COUNTER: Item<u32> = Item::new("channel_counter");
pub const EXPECTED_BLOCK_TIME: Item<u64> = Item::new("expected_block_time");
pub const CONNECTION_PREFIX: Item<Vec<u8>> = Item::new("connection_prefix");
pub const CONNECTION_COUNTER: Item<u32> = Item::new("connection_counter");
pub const CLIENT_COUNTER: Item<u32> = Item::new("client_counter");
pub const HOST_CONSENSUS_STATE: Map<u64, ConsensusState> = Map::new("host_consensus_state");
pub const CONSENSUS_STATES_HEIGHTS: Map<Bytes, BTreeSet<Height>> =
	Map::new("consensus_states_heights");
pub const GRANDPA_HEADER_HASHES_STORAGE: Item<Vec<H256>> = Item::new("grandpa_header_hashes");
pub const GRANDPA_HEADER_HASHES_SET_STORAGE: Map<Vec<u8>, ()> =
	Map::new("grandpa_header_hashes_set");

pub const GRANDPA_BLOCK_HASHES_CACHE_SIZE: usize = 500;

#[derive(Clone, Copy, Debug, PartialEq, Default, Eq)]
pub struct HostFunctions;

impl light_client_common::HostFunctions for HostFunctions {
	type BlakeTwo256 = BlakeTwo256;
}

impl grandpa_light_client_primitives::HostFunctions for HostFunctions {
	type Header = RelayChainHeader;

	fn ed25519_verify(
		sig: &sp_core::ed25519::Signature,
		msg: &[u8],
		pub_key: &sp_core::ed25519::Public,
	) -> bool {
		use ed25519_zebra::{Signature, VerificationKey as PublicKey, VerificationKeyBytes};
		let bytes: [u8; 64] = (sig.clone()).try_into().expect("signature is 64 bytes; qed");
		let sig = Signature::from(bytes);
		let slice: &[u8] = pub_key.as_ref();
		let pub_key = PublicKey::try_from(
			VerificationKeyBytes::try_from(slice).expect("key is always valid; qed"),
		)
		.expect("key is always valid; qed");
		pub_key.verify(&sig, msg).is_ok()
	}

	fn insert_relay_header_hashes(_headers: &[<Self::Header as Header>::Hash]) {
		// implementation of this method is in `Context`
	}

	fn contains_relay_header_hash(_hash: <Self::Header as Header>::Hash) -> bool {
		// implementation of this method is in `Context`
		true
	}
}

fn process_instantiate_msg(
	msg: InstantiateMessage,
	ctx: &mut Context<HostFunctions>,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	let any = Any::decode(&mut msg.client_state.as_slice())?;
	let client_state = ClientState::decode_vec(&any.value)?;
	let any = Any::decode(&mut msg.consensus_state.as_slice())?;
	let consensus_state = ConsensusState::decode_vec(&any.value)?;

	let height = client_state.latest_height();
	ctx.checksum = Some(msg.checksum);
	ctx.store_client_state(client_id.clone(), client_state)
		.map_err(|e| ContractError::Grandpa(e.to_string()))?;
	ctx.store_consensus_state(client_id, height, consensus_state)
		.map_err(|e| ContractError::Grandpa(e.to_string()))?;
	Ok(to_binary(&ContractResult::success())?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: InstantiateMessage,
) -> Result<Response, ContractError> {
	let client_id = ClientId::from_str(env.contract.address.as_str()).expect("client id is valid");
	let mut ctx = Context::<HostFunctions>::new(deps, env);
	let data = process_instantiate_msg(msg, &mut ctx, client_id.clone())?;

	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
	let client = GrandpaClient::<HostFunctions>::default();
	let client_id = ClientId::from_str(env.contract.address.as_str()).expect("client id is valid");
	let mut ctx = Context::<HostFunctions>::new(deps, env);
	let data = process_message(msg, client, &mut ctx, client_id)?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

fn process_message(
	msg: SudoMsg,
	client: GrandpaClient<HostFunctions>,
	ctx: &mut Context<HostFunctions>,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	// log!(ctx, "process_message: {:?}", msg);
	let result = match msg {
		SudoMsg::UpdateStateOnMisbehaviour(msg_raw) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg_raw)?;
			client
				.update_state_on_misbehaviour(client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.and_then(|cs| {
					ctx.store_client_state(client_id, cs)
						.map_err(|e| ContractError::Grandpa(e.to_string()))?;
					Ok(to_binary(&ContractResult::success()))
				})
		},
		SudoMsg::UpdateState(msg_raw) => {
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			let msg = UpdateStateMsg::try_from(msg_raw)?;

			let finalized_headers = match &msg.client_message {
				ClientMessage::Header(header) => {
					use finality_grandpa::Chain;
					let ancestry = AncestryChain::<RelayChainHeader>::new(
						&header.finality_proof.unknown_headers,
					);
					let from = client_state.latest_relay_hash;
					let mut finalized =
						ancestry.ancestry(from, header.finality_proof.block).map_err(|_| {
							ContractError::Grandpa("[update_state] Invalid ancestry!".to_string())
						})?;
					finalized.reverse();
					finalized
				},
				_ => Vec::new(),
			};

			client
				.update_state(ctx, client_id.clone(), client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.and_then(|(cs, cu)| {
					ctx.insert_relay_header_hashes(&finalized_headers);
					store_client_and_consensus_states(ctx, client_id.clone(), cs, cu)
				})
		},
		SudoMsg::MigrateClientStore(_msg) => {
			// load the substitute client state from the combined storage using the appropriate
			// prefix
			let substitute_client_state = ctx
				.client_state_prefixed(SUBSTITUTE_PREFIX)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;

			// No items for the grandpa client state are required to be the same

			let height = substitute_client_state.latest_height();
			// consensus state should be replaced as well
			let substitute_consensus_state =
				ctx.consensus_state_prefixed(height, SUBSTITUTE_PREFIX)?;
			ctx.store_consensus_state_prefixed(height, substitute_consensus_state, SUBJECT_PREFIX);
			ctx.store_client_state_prefixed(substitute_client_state, SUBJECT_PREFIX, client_id)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;

			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		SudoMsg::VerifyMembership(msg) => {
			let msg = VerifyMembershipMsg::try_from(msg)?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			verify_membership::<BlakeTwo256, _>(
				&msg.prefix,
				&msg.proof,
				&consensus_state.root,
				msg.path,
				msg.value,
			)
			.map_err(|e| ContractError::Grandpa(e.to_string()))
			.map(|_| to_binary(&ContractResult::success()))
		},
		SudoMsg::VerifyNonMembership(msg) => {
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;

			verify_non_membership::<BlakeTwo256, _>(
				&msg.prefix,
				&msg.proof,
				&consensus_state.root,
				msg.path,
			)
			.map_err(|e| ContractError::Grandpa(e.to_string()))
			.map(|_| to_binary(&ContractResult::success()))
		},
		SudoMsg::VerifyUpgradeAndUpdateState(msg) => {
			let old_client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			let msg: VerifyUpgradeAndUpdateStateMsg<HostFunctions> =
				VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			client
				.verify_upgrade_and_update_state(
					ctx,
					client_id.clone(),
					&old_client_state,
					&msg.upgrade_client_state,
					&msg.upgrade_consensus_state,
					msg.proof_upgrade_client,
					msg.proof_upgrade_consensus_state,
				)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.and_then(|(cs, cu)| {
					store_client_and_consensus_states(ctx, client_id.clone(), cs, cu)
				})
		},
	};
	Ok(result??)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	let client_id = ClientId::from_str(env.contract.address.as_str()).expect("client id is valid");
	match msg {
		QueryMsg::CheckForMisbehaviour(msg) => {
			let ctx = Context::<HostFunctions>::new_ro(deps, env);
			let client = GrandpaClient::<HostFunctions>::default();
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(&ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(e.to_string()))
				.map(|result| to_binary(&QueryResponse::success().misbehaviour(result)))?
		},
		QueryMsg::ClientTypeMsg(_) => unimplemented!("ClientTypeMsg"),
		QueryMsg::GetLatestHeightsMsg(_) => unimplemented!("GetLatestHeightsMsg"),
		QueryMsg::ExportMetadata(ExportMetadataMsg {}) =>
			to_binary(&QueryResponse::success().genesis_metadata(None)),
		QueryMsg::Status(StatusMsg {}) => {
			let client_state = match get_client_state::<HostFunctions>(deps, client_id.clone()) {
				Ok(client_state) => client_state,
				Err(_) => return to_binary(&QueryResponse::success().status("Unknown".to_string())),
			};

			if client_state.frozen_height().is_some() {
				to_binary(&QueryResponse::success().status("Frozen".to_string()))
			} else {
				let height = client_state.latest_height();
				match get_consensus_state(deps, &client_id, height) {
					Ok(consensus_state_raw) => {
						let consensus_state =
							Context::<HostFunctions>::decode_consensus_state(&consensus_state_raw)
								.map_err(|e| {
									StdError::serialize_err(e.to_string(), e.to_string())
								})?;
						if client_state.expired(core::time::Duration::from_secs(
							env.block.time.seconds() -
								consensus_state.timestamp.unix_timestamp() as u64,
						)) {
							return to_binary(
								&QueryResponse::success().status("Expired".to_string()),
							)
						}
						to_binary(&QueryResponse::success().status("Active".to_string()))
					},
					Err(_) => to_binary(&QueryResponse::success().status("Expired".to_string())),
				}
			}
		},
		QueryMsg::TimestampAtHeight(msg) => {
			let ctx = Context::<HostFunctions>::new_ro(deps, env);
			let consensus_state = ctx
				.consensus_state(&client_id, msg.height)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			to_binary(
				&QueryResponse::success().timestamp(
					consensus_state.timestamp.unix_timestamp_nanos().unsigned_abs() as u64,
				),
			)
		},
		QueryMsg::VerifyClientMessage(msg) => {
			let ctx = Context::<HostFunctions>::new_ro(deps, env);
			let client = GrandpaClient::<HostFunctions>::default();
			let client_state = ctx
				.client_state(&client_id)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			let msg = VerifyClientMessage::try_from(msg)?;

			match &msg.client_message {
				ClientMessage::Misbehaviour(misbehavior) => {
					let first_proof = &misbehavior.first_finality_proof;
					let first_base = first_proof
						.unknown_headers
						.iter()
						.min_by_key(|h| *h.number())
						.ok_or_else(|| {
							ContractError::Grandpa("Unknown headers can't be empty!".to_string())
						})?;
					let first_parent = first_base.parent_hash;
					if !ctx.contains_relay_header_hash(first_parent) {
						Err(ContractError::Grandpa(
							"Could not find the known header for first finality proof".to_string(),
						))?
					}
				},
				_ => {},
			}

			let f = client
				.verify_client_message(&ctx, client_id, client_state, msg.client_message)
				.map_err(|e| ContractError::Grandpa(format!("{e:?}")))
				.map(|_| to_binary(&QueryResponse::success()))?;
			f
		},
	}
}

fn store_client_and_consensus_states<H>(
	ctx: &mut Context<H>,
	client_id: ClientId,
	client_state: ClientState<H>,
	consensus_update: ConsensusUpdateResult<Context<H>>,
) -> Result<StdResult<Binary>, ContractError>
where
	H: grandpa_light_client_primitives::HostFunctions<Header = RelayChainHeader>,
{
	let height = client_state.latest_height();
	let mut heights: Vec<Height> = vec![];
	match consensus_update {
		ConsensusUpdateResult::Single(cs) => {
			log!(ctx, "Storing consensus state: {:?}", height);
			heights.push(height);
			ctx.store_consensus_state(client_id.clone(), height, cs)
				.map_err(|e| ContractError::Grandpa(e.to_string()))?;
		},
		ConsensusUpdateResult::Batch(css) =>
			for (height, cs) in css {
				log!(ctx, "Storing consensus state: {:?}", height);
				heights.push(height);
				ctx.store_consensus_state(client_id.clone(), height, cs)
					.map_err(|e| ContractError::Grandpa(e.to_string()))?;
			},
	}
	log!(ctx, "Storing client state with height: {:?}", height);
	ctx.store_client_state(client_id, client_state)
		.map_err(|e| ContractError::Grandpa(e.to_string()))?;
	Ok(to_binary(&ContractResult::success().heights(heights)))
}

// The FFIs below are required because of sp-io dependency that expects the functions to be
// available on the host.
#[no_mangle]
// &[u8] -> [u8; 32]
pub extern "C" fn ext_hashing_blake2_256_version_1(data: i64) -> i32 {
	let (ptr, len) = unpack_ptr_and_len(data as _);
	let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as _) };
	let hash = blake2_rfc::blake2b::blake2b(32, &[], data);
	let out_ptr = hash.as_ref().to_vec().leak().as_ptr();
	out_ptr as i32
}

pub fn twox_64_into(data: &[u8], dest: &mut [u8; 8]) {
	let r0 = twox_hash::XxHash::with_seed(0).chain_update(data).finish();
	LittleEndian::write_u64(&mut dest[0..8], r0);
}

pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
	let r0 = twox_hash::XxHash::with_seed(0).chain_update(data).finish();
	let r1 = twox_hash::XxHash::with_seed(1).chain_update(data).finish();
	LittleEndian::write_u64(&mut dest[0..8], r0);
	LittleEndian::write_u64(&mut dest[8..16], r1);
}

#[no_mangle]
// &[u8] -> [u8; 16]
pub extern "C" fn ext_hashing_twox_128_version_1(data: i64) -> i32 {
	let (ptr, len) = unpack_ptr_and_len(data as _);
	let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as _) };
	let mut hash = Box::new([0u8; 16]);
	twox_128_into(data, hash.as_mut());
	let out_ptr = Box::leak(hash).as_ptr();
	out_ptr as i32
}

#[no_mangle]
// &[u8] -> [u8; 8]
pub extern "C" fn ext_hashing_twox_64_version_1(data: i64) -> i32 {
	let (ptr, len) = unpack_ptr_and_len(data as _);
	let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as _) };
	let mut hash = Box::new([0u8; 8]);
	twox_64_into(data, hash.as_mut());
	let out_ptr = Box::leak(hash).as_ptr();
	out_ptr as i32
}

#[cfg(test)]
mod tests {
	use cosmwasm_std::{
		from_binary,
		testing::{mock_dependencies, mock_env},
	};
	use ibc::core::ics02_client::client_state::ClientState;
	use tendermint::Time;

	use crate::ics23::ClientStates;

	use super::*;
	#[test]
	fn test_query() {
		let mut deps = mock_dependencies();
		let env = mock_env();

		for (expected, offset) in
			[("Active", 0i64), ("Expired", env.block.time.seconds() as i64 - 10), ("Frozen", 0i64)]
		{
			let mut client_state =
				ics10_grandpa::client_state::ClientState::<HostFunctions>::default();
			let mut consensus_state = ics10_grandpa::consensus_state::ConsensusState::new(
				vec![],
				Time::from_unix_timestamp(0, 0).unwrap(),
			);
			let height = Height { revision_number: 0, revision_height: 1000 };
			client_state.latest_para_height = height.revision_height as _;

			consensus_state.timestamp =
				Time::from_unix_timestamp(env.block.time.seconds() as i64 - offset, 0).unwrap();
			let deps_mut = deps.as_mut();
			if expected == "Frozen" {
				let height =
					Height { revision_number: 0, revision_height: height.revision_height - 100 };
				client_state = client_state.with_frozen_height(height.clone()).unwrap();
			}

			let mut client_states = ClientStates::new(deps_mut.storage);
			client_states.insert(client_state.encode_to_vec().unwrap());

			let mut context = Context::new(deps_mut, env.clone());
			context.store_client_state(ClientId::default(), client_state).unwrap();
			context
				.store_consensus_state(ClientId::default(), height, consensus_state)
				.unwrap();

			let resp = query(deps.as_ref(), mock_env(), QueryMsg::Status(StatusMsg {})).unwrap();

			let resp: QueryResponse = from_binary(&resp).unwrap();

			assert_eq!(resp, QueryResponse::success().status(expected.to_string()));
		}
	}
}
