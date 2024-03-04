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

#![allow(deprecated)]

use crate::error::Error;
use alloc::{string::ToString, vec::Vec};
use bytes::Buf;
use core::cmp::Ordering;
use ibc::{
	core::{
		ics02_client,
		ics24_host::identifier::{ChainId, ClientId},
	},
	prelude::*,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::lightclients::tendermint::v1::{Header as RawHeader, Misbehaviour as RawMisbehaviour},
};
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tendermint::{
	block::{signed_header::SignedHeader, Commit, CommitSig}, validator::Set as ValidatorSet, vote::{SignedVote, ValidatorIndex}, PublicKey, Vote
};
use tendermint_proto::Protobuf;

pub const TENDERMINT_HEADER_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.Header";
pub const TENDERMINT_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.Misbehaviour";
pub const TENDERMINT_CLIENT_MESSAGE_TYPE_URL: &str =
	"/ibc.lightclients.tendermint.v1.ClientMessage";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Misbehaviour {
	pub client_id: ClientId,
	pub header1: Header,
	pub header2: Header,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientMessage {
	Header(Header),
	Misbehaviour(Misbehaviour),
}

impl ics02_client::client_message::ClientMessage for ClientMessage {
	fn encode_to_vec(&self) -> Result<Vec<u8>, tendermint_proto::Error> {
		self.encode_vec()
	}
}

impl Protobuf<Any> for ClientMessage {}

impl TryFrom<Any> for ClientMessage {
	type Error = Error;

	fn try_from(any: Any) -> Result<Self, Self::Error> {
		let msg = match &*any.type_url {
			TENDERMINT_HEADER_TYPE_URL => Self::Header(
				Header::decode(&*any.value).map_err(|e| Error::validation(format!("{e:?}")))?,
			),
			TENDERMINT_MISBEHAVIOUR_TYPE_URL => Self::Misbehaviour(
				Misbehaviour::decode(&*any.value)
					.map_err(|e| Error::validation(format!("{e:?}")))?,
			),
			_ => Err(Error::validation(format!("Unknown type: {}", any.type_url)))?,
		};

		Ok(msg)
	}
}

impl From<ClientMessage> for Any {
	fn from(msg: ClientMessage) -> Self {
		match msg {
			ClientMessage::Header(header) => Any {
				value: header.encode_vec().expect("failed to encode ClientMessage.header"),
				type_url: TENDERMINT_HEADER_TYPE_URL.to_string(),
			},
			ClientMessage::Misbehaviour(misbheaviour) => Any {
				value: misbheaviour
					.encode_vec()
					.expect("failed to encode ClientMessage.misbehaviour"),
				type_url: TENDERMINT_MISBEHAVIOUR_TYPE_URL.to_string(),
			},
		}
	}
}

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
	type Error = Error;

	fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
		Ok(Self {
			client_id: Default::default(),
			header1: raw
				.header_1
				.ok_or_else(|| Error::invalid_raw_misbehaviour("missing header1".into()))?
				.try_into()?,
			header2: raw
				.header_2
				.ok_or_else(|| Error::invalid_raw_misbehaviour("missing header2".into()))?
				.try_into()?,
		})
	}
}

impl From<Misbehaviour> for RawMisbehaviour {
	fn from(value: Misbehaviour) -> Self {
		RawMisbehaviour {
			client_id: value.client_id.to_string(),
			header_1: Some(value.header1.into()),
			header_2: Some(value.header2.into()),
		}
	}
}

impl core::fmt::Display for Misbehaviour {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(
			f,
			"{:?} h1: {:?}-{:?} h2: {:?}-{:?}",
			self.client_id,
			self.header1.height(),
			self.header1.trusted_height,
			self.header2.height(),
			self.header2.trusted_height,
		)
	}
}

/// Tendermint consensus header
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header {
	pub signed_header: SignedHeader, // contains the commitment root
	pub validator_set: ValidatorSet, // the validator set that signed Header
	pub trusted_height: Height,      /* the height of a trusted header seen by client less than
	                                  * or equal to Header */
	// TODO(thane): Rename this to trusted_next_validator_set?
	pub trusted_validator_set: ValidatorSet, // the last trusted validator set at trusted height
}

impl core::fmt::Debug for Header {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, " Header {{...}}")
	}
}

impl Header {
	pub fn height(&self) -> Height {
		Height::new(
			ChainId::chain_version(self.signed_header.header.chain_id.as_str()),
			u64::from(self.signed_header.header.height),
		)
	}

	pub fn timestamp(&self) -> Timestamp {
		self.signed_header.header.time.into()
	}

	pub fn compatible_with(&self, other_header: &Header) -> bool {
		headers_compatible(&self.signed_header, &other_header.signed_header)
	}

	pub fn get_zk_input(&self, size: usize) -> Result<(Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>, u64), Error> {
		#[derive(Clone)]
		struct ZKInput {
			pub_key: Vec<u8>,
			signature: Vec<u8>,
			message: Vec<u8>,
			voting_power: u64,
		}

		let mut pre_input: Vec<ZKInput> = vec![];

		let validator_set = &self.validator_set;
		let signed_header = &self.signed_header;
		let non_absent_votes =
			signed_header.commit.signatures.iter().enumerate().flat_map(|(idx, signature)| {
				non_absent_vote(
					signature,
					ValidatorIndex::try_from(idx).unwrap(),
					&signed_header.commit,
				)
				.map(|vote| (signature, vote))
			});

		let mut seen_validators = HashSet::new();
		let total_voting_power = self.validator_set.total_voting_power().value();

		for (signature, vote) in non_absent_votes {
			// Ensure we only count a validator's power once
			if seen_validators.contains(&vote.validator_address) {
				return Err(Error::validation("validator seen twice".to_string()));
			} else {
				seen_validators.insert(vote.validator_address);
			}

			let validator = match validator_set.validator(vote.validator_address) {
				Some(validator) => validator,
				None => continue, // Cannot find matching validator, so we skip the vote
			};

			let signed_vote =
				match SignedVote::from_vote(vote.clone(), signed_header.header.chain_id.clone()) {
					Some(signed_vote) => signed_vote,
					None =>
						return Err(Error::validation("signed vote cannot be converted".to_string())),
				};

			// If the vote is neither absent nor nil, tally its power
			if !signature.is_commit() {
				continue
			}

			// Check vote is valid
			let sign_bytes = signed_vote.sign_bytes();

			let signature = signed_vote.signature();
			if validator
				.verify_signature::<tendermint::crypto::default::signature::Verifier>(
					&sign_bytes,
					signed_vote.signature(),
				)
				.is_err()
			{}

			let mut zk_input = ZKInput {
				// pub_key: vote.validator_address.into(),
				pub_key: vec![],
				signature: signature.as_bytes().to_vec(),
				message: sign_bytes,
				voting_power: validator.power(),
			};

			let pub_key = &self.validator_set.validators().iter().find(|x| x.address == vote.validator_address).unwrap().pub_key;
			let p = pub_key;
			match p {
				PublicKey::Ed25519(e) => {
					zk_input.pub_key = e.as_bytes().to_vec();
				},
				_ => {}
			};
			pre_input.push(zk_input);
		}

		if pre_input.len() < size {
			// TODO: return error as there aren't enough votes
			return Err(Error::validation(
				"not enough validators have successfully signed".to_string(),
			))
		}

		let not_sorted_pre_input = pre_input.clone();

		// sort by voting power increased
		pre_input.sort_by(
			|ZKInput { voting_power: voting_power_1, .. },
			 ZKInput { voting_power: voting_power_2, .. }| { voting_power_2.cmp(voting_power_1) },
		);

		// count voting power
		let voting_power_amount_validator_size =
			pre_input.iter().take(size).fold(0_u64, |mut acc, x| {
				acc += x.voting_power;
				acc
			});

		// signed votes haven't
		if voting_power_amount_validator_size * 2 <= total_voting_power * 3 {
			return Err(Error::validation("voting power is not > 2/3 + 1".to_string()))
		}

		


		let ret: Vec::<(Vec<u8>, Vec<u8>, Vec<u8>)> = pre_input
			.into_iter()
			.take(size)
			.map(|ZKInput { pub_key, signature, message, .. }| (pub_key, signature, message))
			.collect();

		let validators: Vec<u64> = not_sorted_pre_input
			.iter()
			.map(|element| {
				if ret.iter().any(|x| x.0 == element.pub_key) {
					1
				} else {
					0
				}
			})
			.collect();

		let mut bitmask: u64 = 0;
		for (index, &validator) in validators.iter().enumerate() {
			if validator == 1 {
				bitmask |= 1 << index;
			}
		}

		Ok((ret, bitmask))
	}
}

pub fn headers_compatible(header: &SignedHeader, other: &SignedHeader) -> bool {
	let ibc_client_height = other.header.height;
	let self_header_height = header.header.height;

	match self_header_height.cmp(&ibc_client_height) {
		Ordering::Equal => {
			// 1 - fork
			header.commit.block_id == other.commit.block_id
		},
		Ordering::Greater => {
			// 2 - BFT time violation
			header.header.time > other.header.time
		},
		Ordering::Less => {
			// 3 - BFT time violation
			header.header.time < other.header.time
		},
	}
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
	type Error = Error;

	fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
		let header = Self {
			signed_header: raw
				.signed_header
				.ok_or_else(Error::missing_signed_header)?
				.try_into()
				.map_err(|e| {
				Error::invalid_header("signed header conversion".to_string(), e)
			})?,
			validator_set: raw
				.validator_set
				.ok_or_else(Error::missing_validator_set)?
				.try_into()
				.map_err(Error::invalid_raw_header)?,
			trusted_height: raw.trusted_height.ok_or_else(Error::missing_trusted_height)?.into(),
			trusted_validator_set: raw
				.trusted_validators
				.ok_or_else(Error::missing_trusted_validator_set)?
				.try_into()
				.map_err(Error::invalid_raw_header)?,
		};

		Ok(header)
	}
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
	RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
	fn from(value: Header) -> Self {
		RawHeader {
			signed_header: Some(value.signed_header.into()),
			validator_set: Some(value.validator_set.into()),
			trusted_height: Some(value.trusted_height.into()),
			trusted_validators: Some(value.trusted_validator_set.into()),
		}
	}
}

fn non_absent_vote(
	commit_sig: &CommitSig,
	validator_index: ValidatorIndex,
	commit: &Commit,
) -> Option<Vote> {
	let (validator_address, timestamp, signature, block_id) = match commit_sig {
		CommitSig::BlockIdFlagAbsent { .. } => return None,
		CommitSig::BlockIdFlagCommit { validator_address, timestamp, signature } =>
			(*validator_address, *timestamp, signature, Some(commit.block_id)),
		CommitSig::BlockIdFlagNil { validator_address, timestamp, signature } =>
			(*validator_address, *timestamp, signature, None),
	};

	Some(Vote {
		vote_type: tendermint::vote::Type::Precommit,
		height: commit.height,
		round: commit.round,
		block_id,
		timestamp: Some(timestamp),
		validator_address,
		validator_index,
		signature: signature.clone(),
	})
}


#[cfg(test)]
pub mod test_util {
	use alloc::vec;

	use subtle_encoding::hex;
	use tendermint::{
		block::signed_header::SignedHeader,
		validator::{Info as ValidatorInfo, Set as ValidatorSet},
		PublicKey,
	};

	use crate::client_message::Header;
	use ibc::Height;

	pub fn get_dummy_tendermint_header() -> tendermint::block::Header {
		serde_json::from_str::<SignedHeader>(include_str!("mock/signed_header.json"))
			.unwrap()
			.header
	}

	// TODO: This should be replaced with a ::default() or ::produce().
	// The implementation of this function comprises duplicate code (code borrowed from
	// `tendermint-rs` for assembling a Header).
	// See https://github.com/informalsystems/tendermint-rs/issues/381.
	//
	// The normal flow is:
	// - get the (trusted) signed header and the `trusted_validator_set` at a `trusted_height`
	// - get the `signed_header` and the `validator_set` at latest height
	// - build the ics07 Header
	// For testing purposes this function does:
	// - get the `signed_header` from a .json file
	// - create the `validator_set` with a single validator that is also the proposer
	// - assume a `trusted_height` of 1 and no change in the validator set since height 1, i.e.
	//   `trusted_validator_set` = `validator_set`
	pub fn get_dummy_ics07_header() -> Header {
		// Build a SignedHeader from a JSON file.
		let shdr =
			serde_json::from_str::<SignedHeader>(include_str!("mock/signed_header.json")).unwrap();

		// Build a set of validators.
		// Below are test values inspired form `test_validator_set()` in tendermint-rs.
		let v1: ValidatorInfo = ValidatorInfo::new(
			PublicKey::from_raw_ed25519(
				&hex::decode_upper(
					"F349539C7E5EF7C49549B09C4BFC2335318AB0FE51FBFAA2433B4F13E816F4A7",
				)
				.unwrap(),
			)
			.unwrap(),
			281_815_u64.try_into().unwrap(),
		);

		let vs = ValidatorSet::new(vec![v1.clone()], Some(v1));

		Header {
			signed_header: shdr,
			validator_set: vs.clone(),
			trusted_height: Height::new(0, 1),
			trusted_validator_set: vs,
		}
	}
}
