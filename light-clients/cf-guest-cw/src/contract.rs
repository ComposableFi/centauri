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
	helpers,
	ics23::ReadonlyProcessedStates,
	msg::{
		CheckForMisbehaviourMsg,
		ContractResult,
		ExportMetadataMsg,
		QueryMsg,
		QueryResponse,
		StatusMsg,
		SudoMsg as ExecuteMsg,
		UpdateStateMsg,
		UpdateStateOnMisbehaviourMsg,
		VerifyClientMessage,
		VerifyMembershipMsg,
		VerifyNonMembershipMsg,
		VerifyUpgradeAndUpdateStateMsg,
		// MigrateMsg
	},
	state::{get_client_state, get_consensus_state},
};
use cf_guest::{client_def::GuestClient, proof::verify};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState as _,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_state::ClientState as _,
			context::{ClientKeeper, ClientReader},
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::ClientId,
	},
	protobuf::Protobuf,
};
use ics08_wasm::{instantiate::InstantiateMessage, SUBJECT_PREFIX};
use prost::Message;
use std::str::FromStr;

// #[entry_point]
// pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
//     // No state migrations performed, just returned a Response
//     Ok(Response::default())
// }

fn process_instantiate_msg(
	msg: InstantiateMessage,
	ctx: &mut Context,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	let any = ibc_proto::google::protobuf::Any::decode(&mut msg.client_state.as_slice())?;
	let client_state = cf_guest::ClientState::decode_vec(&any.value)?;
	let any = ibc_proto::google::protobuf::Any::decode(&mut msg.consensus_state.as_slice())?;
	let consensus_state = cf_guest::ConsensusState::decode_vec(&any.value)?;

	ctx.checksum = Some(msg.checksum);
	let height = client_state.latest_height();
	ctx.store_client_state(client_id.clone(), client_state)
		.map_err(ContractError::from)?;
	ctx.store_consensus_state(client_id.clone(), height, consensus_state)
		.map_err(ContractError::from)?;

	ctx.store_update_height(client_id.clone(), height, ctx.host_height())
		.map_err(ContractError::from)?;
	ctx.store_update_time(client_id, height, ctx.host_timestamp())
		.map_err(ContractError::from)?;

	Ok(to_binary(&ContractResult::success())?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: InstantiateMessage,
) -> Result<Response, ContractError> {
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");

	let mut ctx = Context::new(deps, env);
	let data = process_instantiate_msg(msg, &mut ctx, client_id.clone())?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: ExecuteMsg) -> Result<Response, ContractError> {
	let client = GuestClient::<crate::crypto::PubKey>::default();
	let mut ctx = Context::new(deps, env);
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	let data = process_message(msg, client, &mut ctx, client_id)?;
	let mut response = Response::default();
	response.data = Some(data);
	Ok(response)
}

fn process_message(
	msg: ExecuteMsg,
	client: GuestClient<crate::crypto::PubKey>,
	ctx: &mut Context,
	client_id: ClientId,
) -> Result<Binary, ContractError> {
	//log!(ctx, "process_message: {:?}", msg);
	let result = match msg {
		ExecuteMsg::VerifyMembership(msg) => {
			let _ = ctx.client_state(&client_id)?;
			let msg = VerifyMembershipMsg::try_from(msg)?;
			// panic!("Message {:?}", msg.proof.as_bytes());
			// crate::helpers::verify_delay_passed(
			// 	ctx,
			// 	msg.height,
			// 	msg.delay_time_period,
			// 	msg.delay_block_period,
			// )?;
			let consensus_state = ctx.consensus_state(&client_id, msg.height)?;
			verify(
				&CommitmentPrefix::default(),
				&msg.proof,
				consensus_state.root(),
				msg.path,
				Some(msg.value.as_ref()),
			)?;
			Ok(()).map(|_| to_binary(&ContractResult::success()))
		},
		ExecuteMsg::VerifyNonMembership(msg) => {
			let _ = ctx.client_state(&client_id)?;
			let msg = VerifyNonMembershipMsg::try_from(msg)?;
			// crate::helpers::verify_delay_passed(
			// 	ctx,
			// 	msg.height,
			// 	msg.delay_time_period,
			// 	msg.delay_block_period,
			// )?;
			let consensus_state = ctx.consensus_state(&client_id, msg.height)?;

			verify(&CommitmentPrefix::default(), &msg.proof, consensus_state.root(), msg.path, None)
				.map_err(ContractError::from)
				.map(|_| to_binary(&ContractResult::success()))
		},
		// ExecuteMsg::VerifyClientMessage(msg) => {
		// 	let client_state = ctx.client_state(&client_id)?;
		// 	let msg = VerifyClientMessage::try_from(msg)?;
		// 	client
		// 		.verify_client_message(ctx, client_id, client_state, msg.client_message)
		// 		.map_err(ContractError::from)
		// 		.map(|_| to_binary(&ContractResult::success()))
		// },
		// ExecuteMsg::CheckForMisbehaviour(msg) => {
		// 	let client_state = ctx.client_state(&client_id)?;
		// 	let msg = CheckForMisbehaviourMsg::try_from(msg)?;
		// 	client
		// 		.check_for_misbehaviour(ctx, client_id, client_state, msg.client_message)
		// 		.map_err(ContractError::from)
		// 		.map(|result| to_binary(&ContractResult::success().misbehaviour(result)))
		// },
		ExecuteMsg::UpdateStateOnMisbehaviour(msg_raw) => {
			let client_state = ctx.client_state(&client_id)?;
			let msg = UpdateStateOnMisbehaviourMsg::try_from(msg_raw)?;
			client
				.update_state_on_misbehaviour(client_state, msg.client_message)
				.map_err(ContractError::from)
				.and_then(|cs| {
					ctx.store_client_state(client_id, cs)?;
					Ok(to_binary(&ContractResult::success()))
				})
		},
		ExecuteMsg::UpdateState(msg_raw) => {
			let client_state = ctx.client_state(&client_id)?;
			let msg = UpdateStateMsg::try_from(msg_raw)?;
			let latest_revision_height = client_state.latest_height().revision_height;
			helpers::prune_oldest_consensus_state(
				ctx,
				&client_state,
				ctx.host_timestamp().nanoseconds(),
			);
			client
				.update_state(ctx, client_id.clone(), client_state, msg.client_message)
				.map_err(ContractError::from)
				.and_then(|(cs, cu)| {
					let height = cs.latest_height();
					match cu {
						ConsensusUpdateResult::Single(cs) => {
							ctx.store_consensus_state(client_id.clone(), height, cs)?;
						},
						ConsensusUpdateResult::Batch(css) =>
							for (height, cs) in css {
								ctx.store_consensus_state(client_id.clone(), height, cs)?;
							},
					}
					if u64::from(cs.0.latest_height) > latest_revision_height {
						ctx.store_client_state(client_id, cs)?;
					}
					Ok(to_binary(&ContractResult::success()))
				})
		},
		ExecuteMsg::MigrateClientStore(msg) => helpers::check_substitute_and_update_state(ctx)
			.map_err(Into::into)
			.and_then(|(cs, cu)| {
				let height = cs.latest_height();
				ctx.store_consensus_state_prefixed(height, cu, SUBJECT_PREFIX);
				ctx.store_client_state_prefixed(cs, SUBJECT_PREFIX)?;
				Ok(to_binary(&ContractResult::success()))
			}),
		ExecuteMsg::VerifyUpgradeAndUpdateState(msg) => {
			let old_client_state = ctx.client_state(&client_id)?;
			let msg: VerifyUpgradeAndUpdateStateMsg =
				VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;
			helpers::verify_upgrade_and_update_state(
				ctx,
				client_id.clone(),
				old_client_state,
				msg.upgrade_client_state,
				msg.upgrade_consensus_state,
				msg.proof_upgrade_client,
				msg.proof_upgrade_consensus_state,
			)
			.map_err(Into::into)
			.and_then(|(cs, cu)| {
				let height = cs.latest_height();
				ctx.store_consensus_state(client_id.clone(), height, cu)?;
				ctx.store_client_state(client_id, cs)?;
				Ok(to_binary(&ContractResult::success()))
			})
		},
	};
	Ok(result??)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	let client_id = ClientId::from_str("08-wasm-0").expect("client id is valid");
	//deps.api.debug("In query");
	match msg {
		// QueryMsg::ClientTypeMsg(_) => unimplemented!("ClientTypeMsg"),
		// QueryMsg::GetLatestHeightsMsg(_) => unimplemented!("GetLatestHeightsMsg"),
		QueryMsg::ExportMetadata(ExportMetadataMsg {}) => {
			let ro_proceeded_state = ReadonlyProcessedStates::new(deps.storage);
			to_binary(&QueryResponse::success().genesis_metadata(ro_proceeded_state.get_metadata()))
		},
		QueryMsg::Status(StatusMsg {}) => {
			let client_state = match get_client_state::<crate::crypto::PubKey>(deps) {
				Ok(state) => state,
				Err(_) => return to_binary(&QueryResponse::success().status("Unknown".to_string())),
			};

			if client_state.frozen_height().is_some() {
				return to_binary(&QueryResponse::success().status("Frozen".to_string()));
			}

			let height = client_state.latest_height();
			let consensus_state = match get_consensus_state(deps, &client_id, height) {
				Ok(state) => state,
				Err(_) => return to_binary(&QueryResponse::success().status("Expired".to_string())),
			};

			let last_update = consensus_state.0.timestamp_ns.get();
			let trusting_period = client_state.0.trusting_period_ns;
			let now = env.block.time.nanos();
			if last_update + trusting_period < now {
				return to_binary(&QueryResponse::success().status("Expired".to_string()));
			}
			to_binary(&QueryResponse::success().status("Active".to_string()))
		},
		QueryMsg::VerifyClientMessage(msg) => {
			let ctx = Context::new_ro(deps, env);
			let client_state = ctx.client_state(&client_id).map_err(ContractError::from)?;
			let client = GuestClient::<crate::crypto::PubKey>::default();
			let msg = VerifyClientMessage::try_from(msg)?;
			client
				.verify_client_message(&ctx, client_id, client_state, msg.client_message)
				.map_err(ContractError::Client)
				.map(|_| to_binary(&QueryResponse::success()))?
		},
		QueryMsg::TimestampAtHeight(_msg) => todo!(),
		QueryMsg::CheckForMisbehaviour(msg) => {
			let ctx = Context::new_ro(deps, env);
			let client_state = ctx.client_state(&client_id).map_err(ContractError::from)?;
			let client = GuestClient::<crate::crypto::PubKey>::default();
			let msg = CheckForMisbehaviourMsg::try_from(msg)?;
			client
				.check_for_misbehaviour(&ctx, client_id, client_state, msg.client_message)
				.map_err(ContractError::Client)
				.map(|result| to_binary(&QueryResponse::success().misbehaviour(result)))?
		},
	}
}
