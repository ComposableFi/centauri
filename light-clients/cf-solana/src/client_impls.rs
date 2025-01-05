use core::num::NonZeroU64;

use alloc::{string::ToString, vec::Vec};

use ibc_proto::google::protobuf::Any;

use super::{ClientMessage, ClientState, ConsensusState};
use cf_solana_upstream::Neighbourhood;

mod ibc {
	pub use ibc::{
		core::{
			ics02_client::{error::Error as ClientError, height::Height},
			ics24_host::identifier::ClientId,
		},
		timestamp::Timestamp,
	};
}

type Result<T = (), E = ibc::ClientError> = ::core::result::Result<T, E>;

pub trait CommonContext {
	type ConversionError: ToString;
	type AnyClientState: From<ClientState>;
	type AnyConsensusState: TryInto<ConsensusState, Error = Self::ConversionError>
		+ From<ConsensusState>;

	fn host_metadata(&self) -> Result<(ibc::Timestamp, ibc::Height)>;

	fn set_client_state(
		&mut self,
		client_id: &ibc::ClientId,
		state: Self::AnyClientState,
	) -> Result<()>;

	fn consensus_state(
		&self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
	) -> Result<Self::AnyConsensusState>;

	/// Returns consensus at given height or its neighbours.
	///
	/// If consensus state at given height returns `This(state)` for that state.
	/// Otherwise, returns `Neighbours(prev, next)` where `prev` and `next` are
	/// states with lower and greater height respectively if they exist.
	fn consensus_state_neighbourhood(
		&self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
	) -> Result<Neighbourhood<Self::AnyConsensusState>>;

	fn store_consensus_state_and_metadata(
		&mut self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
		consensus: Self::AnyConsensusState,
		host_timestamp: ibc::Timestamp,
		host_height: ibc::Height,
	) -> Result;

	fn delete_consensus_state_and_metadata(
		&mut self,
		client_id: &ibc::ClientId,
		height: ibc::Height,
	) -> Result;

	/// Returns earliest consensus state for given client.
	fn earliest_consensus_state(
		&self,
		client_id: &ibc::ClientId,
	) -> Result<Option<(ibc::Height, Self::AnyConsensusState)>>;
}

// impl<PK: PubKey> ibc::ClientStateCommon for ClientState<PK> {
//     fn verify_consensus_state(&self, consensus_state: Any) -> Result {
//         ConsensusState::try_from(consensus_state)?;
//         Ok(())
//     }

//     fn client_type(&self) -> ibc::ClientType {
//         ibc::ClientType::new(super::CLIENT_TYPE).unwrap()
//     }

//     fn latest_height(&self) -> ibc::Height {
//         ibc::Height::new(0, self.latest_height.into()).unwrap()
//     }

//     fn validate_proof_height(&self, proof_height: ibc::Height) -> Result {
//         let latest_height = self.latest_height();
//         if proof_height <= latest_height {
//             Ok(())
//         } else {
//             Err(ibc::ClientError::InvalidProofHeight {
//                 latest_height,
//                 proof_height,
//             })
//         }
//     }

//     /// Panics since client upgrades aren’t supported.
//     fn verify_upgrade_client(
//         &self,
//         _upgraded_client_state: Any,
//         _upgraded_consensus_state: Any,
//         _proof_upgrade_client: ibc::CommitmentProofBytes,
//         _proof_upgrade_consensus_state: ibc::CommitmentProofBytes,
//         _root: &ibc::CommitmentRoot,
//     ) -> Result { unimplemented!("IBC cilent upgrades are currently not supported")
//     }

//     /// Verifies membership proof.
//     ///
//     /// See [`proof::verify`] for documentation of the proof format.
//     fn verify_membership(
//         &self,
//         prefix: &ibc::CommitmentPrefix,
//         proof: &ibc::CommitmentProofBytes,
//         root: &ibc::CommitmentRoot,
//         path: ibc::path::Path,
//         value: Vec<u8>,
//     ) -> Result { let value = Some(value.as_slice()); proof::verify(prefix, proof, root, path,
//       value).map_err(Into::into)
//     }

//     /// Verifies membership proof.
//     ///
//     /// See [`proof::verify`] for documentation of the proof format.
//     fn verify_non_membership(
//         &self,
//         prefix: &ibc::CommitmentPrefix,
//         proof: &ibc::CommitmentProofBytes,
//         root: &ibc::CommitmentRoot,
//         path: ibc::path::Path,
//     ) -> Result { proof::verify(prefix, proof, root, path, None).map_err(Into::into)
//     }
// }

// impl From<proof::VerifyError> for ibc::ClientError {
// 	fn from(err: proof::VerifyError) -> Self {
// 		use ::ibc::core::ics23_commitment::error::Error;
// 		use proof::VerifyError::*;

// 		Self::invalid_commitment_proof(match err {
// 			ProofDecodingFailure(msg) =>
// 				Error::commitment_proof_decoding_failed(DecodeError::new(msg)),
// 			WrongSequenceNumber(err) => Error::commitment_proof_decoding_failed(err),
// 			_ => ibc::CommitmentError::invalid_merkle_proof(),
// 		})
// 	}
// }
// impl<PK: PubKey, E> ibc::ClientStateExecution<E> for ClientState<PK>
// where
//     E: ibc::ExecutionContext + ibc::ClientExecutionContext + CommonContext,
//     <E as ibc::ClientExecutionContext>::AnyClientState: From<ClientState<PK>>,
//     <E as ibc::ClientExecutionContext>::AnyConsensusState: From<ConsensusState>,
// {
//     fn initialise(
//         &self,
//         ctx: &mut E,
//         client_id: &ibc::ClientId,
//         consensus_state: Any,
//     ) -> Result { parse_client_id(client_id)?; let consensus_state =
//       super::ConsensusState::try_from(consensus_state)?;

//         ctx.store_client_state(
//             ibc::path::ClientStatePath::new(client_id.clone()),
//             self.clone().into(),
//         )?;
//         ctx.store_consensus_state(
//             ibc::path::ClientConsensusStatePath::new(
//                 client_id.clone(),
//                 0,
//                 u64::from(self.latest_height),
//             ),
//             consensus_state.into(),
//         )?;

//         Ok(())
//     }

//     fn update_state(
//         &self,
//         ctx: &mut E,
//         client_id: &ibc::ClientId,
//         header: Any,
//     ) -> Result<Vec<ibc::Height>> { let header = crate::proto::Header::try_from(header)?; let
//       header = crate::Header::<PK>::try_from(header)?; let header_height = ibc::Height::new(0,
//       header.block_header.block_height.into())?;

//         let (host_timestamp, host_height) = CommonContext::host_metadata(ctx)?;
//         self.prune_oldest_consensus_state(ctx, client_id, host_timestamp)?;

//         let maybe_existing_consensus =
//             CommonContext::consensus_state(ctx, client_id, header_height).ok();
//         if maybe_existing_consensus.is_none() {
//             let new_consensus_state = ConsensusState::from(&header);
//             let new_client_state = self.with_header(&header);

//             ctx.store_client_state(
//                 ibc::path::ClientStatePath::new(client_id.clone()),
//                 new_client_state.into(),
//             )?;
//             ctx.store_consensus_state_and_metadata(
//                 client_id,
//                 header_height,
//                 new_consensus_state.into(),
//                 host_timestamp,
//                 host_height,
//             )?;
//         }

//         Ok(alloc::vec![header_height])
//     }

//     fn update_state_on_misbehaviour(
//         &self,
//         ctx: &mut E,
//         client_id: &ibc::ClientId,
//         _client_message: Any,
//     ) -> Result { ctx.store_client_state( ibc::path::ClientStatePath::new(client_id.clone()),
//       self.frozen().into(), )?; Ok(())
//     }

//     fn update_state_on_upgrade(
//         &self,
//         _ctx: &mut E,
//         _client_id: &ibc::ClientId,
//         _upgraded_client_state: Any,
//         _upgraded_consensus_state: Any,
//     ) -> Result<ibc::Height> { Err(ibc::UpgradeClientError::Other { reason: "upgrade not
//       supported".into(), } .into())
//     }
// }

// impl<PK: PubKey, V> ibc::ClientStateValidation<V> for ClientState<PK>
// where
//     V: ibc::ValidationContext
//         + ibc::ClientValidationContext
//         + CommonContext
//         + guestchain::Verifier<PK>,
// {
//     fn verify_client_message(
//         &self,
//         ctx: &V,
//         client_id: &ibc::ClientId,
//         client_message: Any,
//     ) -> Result { self.verify_client_message(ctx, client_id, client_message)
//     }

//     fn check_for_misbehaviour(
//         &self,
//         ctx: &V,
//         client_id: &ibc::ClientId,
//         client_message: Any,
//     ) -> Result<bool> { self.check_for_misbehaviour(ctx, client_id, client_message)
//     }

//     fn status(
//         &self,
//         ctx: &V,
//         client_id: &ibc::ClientId,
//     ) -> Result<ibc::Status> { if self.is_frozen { return Ok(ibc::Status::Frozen); }

//         let height = ibc::Height::new(0, self.latest_height.into())?;
//         let consensus = CommonContext::consensus_state(ctx, client_id, height)
//             .and_then(|state| state.try_into().map_err(error));
//         let consensus = match consensus {
//             Ok(consensus) => consensus,
//             Err(ibc::ClientError::ConsensusStateNotFound { .. }) => {
//                 return Ok(ibc::Status::Expired)
//             }
//             Err(err) => return Err(err),
//         };

//         let (host_timestamp, _height) = CommonContext::host_metadata(ctx)?;
//         Ok(if self.consensus_has_expired(&consensus, host_timestamp) {
//             ibc::Status::Expired
//         } else {
//             ibc::Status::Active
//         })
//     }
// }

impl ClientState {
	pub fn do_update_state(
		&self,
		ctx: &mut impl CommonContext,
		client_id: &ibc::ClientId,
		header: cf_solana_upstream::Header,
	) -> Result<Vec<ibc::Height>> {
		let header_height = ibc::Height::new(1, header.slot.into());
		let (host_timestamp, host_height) = CommonContext::host_metadata(ctx)?;
		self.prune_oldest_consensus_state(ctx, client_id, host_timestamp)?;

		let maybe_existing_consensus =
			CommonContext::consensus_state(ctx, client_id, header_height).ok();
		if maybe_existing_consensus.is_none() {
			let new_consensus_state = ConsensusState::from(&header);
			let new_client_state = self.with_header(&header);

			ctx.set_client_state(client_id, new_client_state.into())?;
			ctx.store_consensus_state_and_metadata(
				client_id,
				header_height,
				new_consensus_state.into(),
				host_timestamp,
				host_height,
			)?;
		}

		Ok(alloc::vec![header_height])
	}

	pub fn do_verify_client_message(
		&self,
		_ctx: &impl CommonContext,
		client_message: cf_solana_upstream::ClientMessage,
	) -> Result<()> {
		match client_message {
			cf_solana_upstream::ClientMessage::Header(header) => self.verify_header(header),
			cf_solana_upstream::ClientMessage::Misbehaviour(misbehaviour) => {
				let cf_solana_upstream::Misbehaviour { header1, header2 } = misbehaviour;
				self.verify_header(header1)?;
				self.verify_header(header2)?;
				Ok(())
			},
		}
	}

	pub fn check_for_misbehaviour(
		&self,
		ctx: &impl CommonContext,
		client_id: &ibc::ClientId,
		client_message: Any,
	) -> Result<bool> {
		let client_message = ClientMessage::try_from(client_message)?;
		self.do_check_for_misbehaviour(ctx, client_id, client_message)
	}

	pub fn do_check_for_misbehaviour(
		&self,
		ctx: &impl CommonContext,
		client_id: &ibc::ClientId,
		client_message: ClientMessage,
	) -> Result<bool> {
		match client_message.0 {
			cf_solana_upstream::ClientMessage::Header(header) =>
				self.check_for_misbehaviour_in_header(ctx, client_id, header),
			cf_solana_upstream::ClientMessage::Misbehaviour(misbehaviour) =>
				self.check_for_misbehaviour_in_misbehavior(ctx, client_id, misbehaviour),
		}
	}

	fn verify_header(&self, header: cf_solana_upstream::Header) -> Result<()> {
		let cf_solana_upstream::Header { bank_hash, delta_hash_proof, witness_proof, .. } = header;
		if bank_hash != delta_hash_proof.calculate_bank_hash() {
			Err(error("Invalid accounts delta hash proof"))
		} else if delta_hash_proof.accounts_delta_hash != witness_proof.expected_root() {
			Err(error("Invalid witness proof"))
		} else if witness_proof.account_hash_data.key() != &self.0.witness_account {
			Err(error("Invalid witness account"))
		} else {
			Ok(())
		}
	}

	fn check_for_misbehaviour_in_header(
		&self,
		ctx: &impl CommonContext,
		client_id: &ibc::ClientId,
		header: cf_solana_upstream::Header,
	) -> Result<bool> {
		fn check_timestamp<T: TryInto<ConsensusState, Error = E>, E: ToString>(
			state: Option<T>,
			test: impl FnOnce(NonZeroU64) -> bool,
		) -> Result<bool> {
			match state.map(|state| state.try_into()) {
				None => Ok(false),
				Some(Ok(state)) => Ok(test(state.0.timestamp_sec)),
				Some(Err(err)) => Err(error(err)),
			}
		}

		let height = ibc::Height::new(1, header.slot.into());
		Ok(match ctx.consensus_state_neighbourhood(client_id, height)? {
			Neighbourhood::This(state) => {
				// If we already have existing consensus for given height, check
				// that what we’ve been sent is the same thing we have.  If it
				// isn’t, that’s evidence of misbehaviour.
				let existing_state = state.try_into().map_err(error)?;
				let header_state = ConsensusState::from(&header);
				existing_state != header_state
			},

			Neighbourhood::Neighbours(prev, next) => {
				// Otherwise, first try to decode witness in the header.  If
				// it’s invalid that this doesn't prove misbehaviour (though it
				// also won’t update the consensus but that’s handled in
				// do_update_state).
				let current = match header.decode_witness() {
					Some((_, timestamp_sec)) => timestamp_sec,
					None => return Ok(false),
				};

				// Make sure that timestamp of each consensus is non-decreasing.
				// If it isn’t, that’s evidence of misbehaviour.  Solana uses
				// timestamps with second-granularity with sub-second blocks so
				// consecutive slots may have the same timestamp.
				check_timestamp(prev, |prev| current < prev)? ||
					check_timestamp(next, |next| next < current)?
			},
		})
	}

	fn check_for_misbehaviour_in_misbehavior(
		&self,
		_ctx: &impl CommonContext,
		_client_id: &ibc::ClientId,
		misbehaviour: cf_solana_upstream::Misbehaviour,
	) -> Result<bool> {
		let cf_solana_upstream::Misbehaviour { header1, header2 } = misbehaviour;
		if header1.slot == header2.slot {
			// If blocks have the same height they must be the same, i.e. have
			// the same witness account.
			Ok(header1.witness_proof.account_hash_data != header2.witness_proof.account_hash_data)
		} else {
			// Otherwise, if blocks have different heights, their ordering must
			// match ordering of their timestamps (with the exception that it’s
			// valid for timestamps to be equal).
			let mut first = header1.decode_witness().map(|(_, ts)| ts);
			let mut second = header2.decode_witness().map(|(_, ts)| ts);
			if header2.slot < header1.slot {
				core::mem::swap(&mut first, &mut second);
			}
			Ok(first > second)
		}
	}

	/// Checks whether consensus state has expired.
	fn consensus_has_expired(
		&self,
		consensus: &ConsensusState,
		host_timestamp: ibc::Timestamp,
	) -> bool {
		let expiry_ns = consensus.0.timestamp_sec.get().saturating_add(self.0.trusting_period_ns);
		let host_timestamp_ns = host_timestamp.nanoseconds();
		expiry_ns <= host_timestamp_ns
	}

	/// Removes all expired consensus states.
	fn prune_oldest_consensus_state(
		&self,
		ctx: &mut impl CommonContext,
		client_id: &ibc::ClientId,
		host_timestamp: ibc::Timestamp,
	) -> Result {
		if let Some((height, state)) = ctx.earliest_consensus_state(client_id)? {
			let state = state.try_into().map_err(error)?;
			if self.consensus_has_expired(&state, host_timestamp) {
				ctx.delete_consensus_state_and_metadata(client_id, height)?;
			}
		}
		Ok(())
	}
}

fn error(msg: impl ToString) -> ibc::ClientError {
	ibc::ClientError::implementation_specific(msg.to_string())
}
