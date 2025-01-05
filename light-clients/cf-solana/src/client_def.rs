use core::str::FromStr;

use crate::alloc::string::ToString;
use alloc::vec::Vec;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_state::ClientState as OtherClientState,
			error::Error as Ics02ClientError,
		},
		ics26_routing::context::ReaderContext,
	},
	protobuf::Protobuf,
};
use prost::Message;

use crate::{error::Error, ClientMessage, ClientState, ConsensusState as ClientConsensusState};

type Result<T = (), E = ibc::core::ics02_client::error::Error> = ::core::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SolanaClient {}

impl ClientDef for SolanaClient {
	type ClientMessage = ClientMessage;
	type ClientState = ClientState;
	type ConsensusState = ClientConsensusState;

	fn verify_client_message<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<(), Ics02ClientError> {
		let ctx = CommonContext::new(_ctx);
		client_state.0.do_verify_client_message(ctx, client_msg.0).map_err(convert)
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Ics02ClientError,
	> {
		let header = match client_msg.0 {
			cf_solana_upstream::ClientMessage::Header(header) => header,
			_ => unreachable!("02-client will check for Header before calling update_state; qed"),
		};
		let header_consensus_state = ClientConsensusState::from(&header);
		let cs = Ctx::AnyConsensusState::wrap(&header_consensus_state).ok_or_else(|| {
			Error::UnknownConsensusStateType { description: "Ctx::AnyConsensusState".to_string() }
		})?;
		Ok((client_state.with_header(&header), ConsensusUpdateResult::Single(cs)))
	}

	fn update_state_on_misbehaviour(
		&self,
		client_state: Self::ClientState,
		_client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02ClientError> {
		Ok(client_state.frozen())
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<bool, Ics02ClientError> {
		let client_id = convert(client_id);
		let ctx = CommonContext::new(ctx);
		client_state
			.0
			.do_check_for_misbehaviour(ctx, &client_id, client_msg.0)
			.map_err(convert)
	}

	fn verify_upgrade_and_update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		_old_client_state: &Self::ClientState,
		_upgrade_client_state: &Self::ClientState,
		_upgrade_consensus_state: &Self::ConsensusState,
		_proof_upgrade_client: ibc::prelude::Vec<u8>,
		_proof_upgrade_consensus_state: ibc::prelude::Vec<u8>,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Ics02ClientError,
	> {
		// TODO: tendermint verify_upgrade_and_update_state
		Err(Ics02ClientError::implementation_specific("Not implemented".to_string()))
	}

	fn check_substitute_and_update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_subject_client_id: ibc::core::ics24_host::identifier::ClientId,
		_substitute_client_id: ibc::core::ics24_host::identifier::ClientId,
		_old_client_state: Self::ClientState,
		_substitute_client_state: Self::ClientState,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Ics02ClientError,
	> {
		// TODO: tendermint check_substitute_and_update_state
		Err(Ics02ClientError::implementation_specific("Not implemented".to_string()))
	}

	fn verify_client_consensus_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		consensus_height: ibc::Height,
		expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ClientConsensusStatePath {
			client_id: convert(client_id),
			revision_number: consensus_height.revision_number,
			revision_height: consensus_height.revision_height,
		};
		let value = expected_consensus_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_connection_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		connection_id: &ibc::core::ics24_host::identifier::ConnectionId,
		expected_connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ConnectionPath(convert(connection_id));
		let value = expected_connection_end.clone().encode_vec();
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_channel_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		expected_channel_end: &ibc::core::ics04_channel::channel::ChannelEnd,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ChannelEndPath(convert(port_id), convert(channel_id));
		let value = expected_channel_end.clone().encode_vec();
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_client_full_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		_prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;

		let path = ibc_core_host_types::path::ClientStatePath(convert(client_id));
		let value = expected_client_state.encode_to_vec().map_err(Ics02ClientError::encode)?;
		verify(proof, root, path.into(), Some(value))
	}

	fn verify_packet_data<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
		commitment: ibc::core::ics04_channel::commitment::PacketCommitment,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::CommitmentPath {
			port_id: convert(port_id),
			channel_id: convert(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), Some(commitment.into_vec()))
	}

	fn verify_packet_acknowledgement<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
		ack: ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
	) -> Result<(), Ics02ClientError> {
		// client state height = consensus state height
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::AckPath {
			port_id: convert(port_id),
			channel_id: convert(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), Some(ack.into_vec()))
	}

	fn verify_next_sequence_recv<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::SeqRecvPath(convert(port_id), convert(channel_id));
		let mut seq_bytes = Vec::new();
		u64::from(sequence).encode(&mut seq_bytes).expect("buffer size too small");
		verify(proof, root, path.into(), Some(seq_bytes))
	}

	fn verify_packet_receipt_absence<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
	) -> Result<(), Ics02ClientError> {
		client_state.verify_height(client_id, height)?;
		verify_delay_passed::<Ctx>(ctx, height, connection_end)?;

		let path = ibc_core_host_types::path::ReceiptPath {
			port_id: convert(port_id),
			channel_id: convert(channel_id),
			sequence: sequence.0.into(),
		};
		verify(proof, root, path.into(), None)
	}
}

fn verify_delay_passed<Ctx: ReaderContext>(
	ctx: &Ctx,
	height: ibc::Height,
	connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
) -> Result<(), Ics02ClientError> {
	let current_timestamp = ctx.host_timestamp();
	let current_height = ctx.host_height();

	let client_id = connection_end.client_id();
	let processed_time = ctx
		.client_update_time(client_id, height)
		.map_err(|_| Error::ProcessedTimeNotFound { height })?;
	let processed_height = ctx
		.client_update_height(client_id, height)
		.map_err(|_| Error::ProcessedHeightNotFound { height })?;

	let delay_period_time = connection_end.delay_period();
	let delay_period_height = ctx.block_delay(delay_period_time);
	let delay_period_time_u64 = u64::try_from(delay_period_time.as_nanos()).unwrap();

	ClientState::verify_delay_passed(
		current_timestamp,
		current_height,
		processed_time.nanoseconds(),
		processed_height.revision_height,
		delay_period_time_u64,
		delay_period_height,
	)
	.map_err(|e| e.into())
}

// impl Verifier for SolanaClient {
// 	fn verify(&self, message: &[u8], pubkey: &PK, signature: &PK::Signature) -> bool {
// 		(|| {
// 			let pubkey = pubkey.as_bytes();
// 			let pubkey = ed25519_consensus::VerificationKey::try_from(&pubkey[..]).ok()?;
// 			let signature = signature.as_bytes();
// 			let sig = ed25519_consensus::Signature::try_from(&signature[..]).ok()?;
// 			pubkey.verify(&sig, message).ok()?;
// 			Some(())
// 		})()
// 		.is_some()
// 	}
// }

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[transparent(Ctx)]
struct CommonContext<Ctx> {
	ctx: Ctx,
}

impl<Ctx> CommonContext<Ctx> {
	fn new(ctx: &Ctx) -> &Self {
		bytemuck::TransparentWrapper::wrap_ref(ctx)
	}
}

type NewResult<T = ()> = Result<T, ibc_core_client_types::error::ClientError>;

impl<Ctx: ReaderContext> cf_solana_upstream::CommonContext for CommonContext<Ctx> {
	type ConversionError = core::convert::Infallible;
	type AnyClientState = ClientState;
	type AnyConsensusState = ClientConsensusState;

	fn host_metadata(
		&self,
	) -> NewResult<(ibc_primitives::Timestamp, ibc_core_client_types::Height)> {
		unimplemented!("host_metadata")
	}

	fn set_client_state(
		&mut self,
		_client_id: &ibc_core_host_types::identifiers::ClientId,
		_state: ClientState,
	) -> NewResult<()> {
		unimplemented!("set_client_state")
	}

	fn consensus_state(
		&self,
		_client_id: &ibc_core_host_types::identifiers::ClientId,
		_height: ibc_core_client_types::Height,
	) -> NewResult<Self::AnyConsensusState> {
		unimplemented!("consensus_state")
	}

	fn consensus_state_neighbourhood(
		&self,
		client_id: &ibc_core_host_types::identifiers::ClientId,
		height: ibc_core_client_types::Height,
	) -> NewResult<cf_solana_upstream::Neighbourhood<Self::AnyConsensusState>> {
		use cf_solana_upstream::Neighbourhood;

		let res: Result<_, Ics02ClientError> = (|| {
			let client_id = convert(client_id);
			let height = convert(height);
			Ok(if let Some(state) = self.ctx.maybe_consensus_state(&client_id, height)? {
				Neighbourhood::This(state)
			} else {
				let prev = self.ctx.prev_consensus_state(&client_id, height)?;
				let next = self.ctx.next_consensus_state(&client_id, height)?;
				Neighbourhood::Neighbours(prev, next)
			})
		})();
		match res {
			Ok(res) => Ok(res.map(|state: Ctx::AnyConsensusState| {
				// TODO(mina86): propagate error rather than unwrapping
				let state: Self::AnyConsensusState = state.downcast().unwrap();
				state
			})),
			Err(err) => Err(convert(err)),
		}
	}

	fn store_consensus_state_and_metadata(
		&mut self,
		_client_id: &ibc_core_host_types::identifiers::ClientId,
		_height: ibc_core_client_types::Height,
		_consensus: Self::AnyConsensusState,
		_host_timestamp: ibc_primitives::Timestamp,
		_host_height: ibc_core_client_types::Height,
	) -> NewResult {
		unimplemented!("store_consensus_state_and_metadata")
	}

	fn delete_consensus_state_and_metadata(
		&mut self,
		_client_id: &ibc_core_host_types::identifiers::ClientId,
		_height: ibc_core_client_types::Height,
	) -> NewResult {
		unimplemented!("delete_consensus_state_and_metadata")
	}

	fn earliest_consensus_state(
		&self,
		_client_id: &ibc_core_host_types::identifiers::ClientId,
	) -> NewResult<Option<(ibc_core_client_types::Height, Self::AnyConsensusState)>> {
		unimplemented!("earliest_consensus_state")
	}
}

// Helper wrappers

fn verify(
	proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
	root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
	path: ibc_core_host_types::path::Path,
	value: Option<Vec<u8>>,
) -> Result<(), Ics02ClientError> {
	cf_solana_upstream::proof::verify_for_trie(
		&[],
		proof.as_bytes(),
		root.bytes.as_slice(),
		path,
		value.as_deref(),
	)
	.map_err(|err| Ics02ClientError::implementation_specific(err.to_string()))
}

// Conversion between old and new.

fn convert<F, T: ConvertFrom<F>>(value: F) -> T {
	T::convert(value)
}

trait ConvertFrom<From>: Sized {
	fn convert(value: From) -> Self;
}

macro_rules! conv {
	($( $value:ident: $From:ty => $To:ty { $($body:tt)*} )*) => {
		$(
			impl ConvertFrom<$From> for $To {
				fn convert($value: $From) -> Self {
					Self::convert(&$value)
				}
			}

			impl ConvertFrom<&$From> for $To {
				fn convert($value: &$From) -> Self {
					$($body)*
				}
			}
		)*
	}
}

conv! {
	client_id: ibc::core::ics24_host::identifier::ClientId =>
		ibc_core_host_types::identifiers::ClientId {
			FromStr::from_str(client_id.as_str()).unwrap()
		}
	client_id: ibc_core_host_types::identifiers::ClientId => ibc::core::ics24_host::identifier::ClientId {
			FromStr::from_str(client_id.as_str()).unwrap()
		}

	connection_id: ibc::core::ics24_host::identifier::ConnectionId =>
		ibc_core_host_types::identifiers::ConnectionId {
			FromStr::from_str(connection_id.as_str()).unwrap()
		}

	port_id: ibc::core::ics24_host::identifier::PortId =>
		ibc_core_host_types::identifiers::PortId {
			FromStr::from_str(port_id.as_str()).unwrap()
		}

	channel_id: ibc::core::ics24_host::identifier::ChannelId =>
		ibc_core_host_types::identifiers::ChannelId {
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence())
		}


	timestamp: ibc::timestamp::Timestamp => ibc_primitives::Timestamp {
		Self::from_nanoseconds(timestamp.nanoseconds()).unwrap()
	}

	height: ibc::core::ics02_client::height::Height => ibc_core_client_types::Height {
		Self::new(height.revision_number, height.revision_height).unwrap()
	}

	height: ibc_core_client_types::Height => ibc::core::ics02_client::height::Height {
		Self::new(height.revision_number(), height.revision_height())
	}


	err: ibc_core_client_context::types::error::ClientError => Ics02ClientError {
		// TODO(mina86): Create better mapping.
		Self::implementation_specific(err.to_string())
	}
	err: Ics02ClientError => ibc_core_client_context::types::error::ClientError {
		// TODO(mina86): Create better mapping.
		Self::ClientSpecific {
			description: err.to_string()
		}
	}
}

impl<FT, TT: ConvertFrom<FT>> ConvertFrom<Option<FT>> for Option<TT> {
	fn convert(value: Option<FT>) -> Self {
		value.map(convert)
	}
}

impl<FT, FE, TT: ConvertFrom<FT>, TE: ConvertFrom<FE>> ConvertFrom<Result<FT, FE>>
	for Result<TT, TE>
{
	fn convert(value: Result<FT, FE>) -> Self {
		value.map(convert).map_err(convert)
	}
}
