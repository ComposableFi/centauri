use cosmwasm_std::StdError;
use derive_more::{Display, From};
use icsxx_ethereum::error::Error as GrandpaError;
use std::error::Error;
// use thiserror::Error;

#[derive(From, Display, Debug)]
pub enum ContractError {
	Std(StdError),
	#[display(fmt = "Unauthorized")]
	Unauthorized {},
	// Add any other custom errors you like here.
	// Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
	#[display(fmt = "Storage error")]
	StorageError,
	// TODO: use `ics10-grandpa`'s error type here
	#[display(fmt = "Grandpa error: {_0}")]
	#[from(ignore)]
	Client(String),
	#[display(fmt = "Protobuf error: {_0}")]
	Protobuf(ibc::protobuf::Error),
	#[display(fmt = "IBC validation error: {_0}")]
	Validation(ibc::core::ics24_host::error::ValidationError),
	#[display(fmt = "IBC path error: {_0}")]
	Path(ibc::core::ics24_host::path::PathError),
	#[display(fmt = "IBC proof error: {_0}")]
	Proof(ibc::proofs::ProofError),
	#[display(fmt = "IBC commitment error: {_0}")]
	Commitment(ibc::core::ics23_commitment::error::Error),
	#[display(fmt = "Proto decode error: {_0}")]
	ProtoDecode(prost::DecodeError),
	#[display(fmt = "From UTF8 error: {_0}")]
	FromUtf8(alloc::string::FromUtf8Error),
}

impl Error for ContractError {}

impl From<GrandpaError> for ContractError {
	fn from(e: GrandpaError) -> Self {
		ContractError::Client(e.to_string())
	}
}
