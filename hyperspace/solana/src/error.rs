use anchor_client::solana_sdk::signature::ParseSignatureError;
use ibc::timestamp::ParseTimestampError;
use prost::DecodeError;

/// Error definitions for the cosmos client in accordance with the parachain's Error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// An error from the rpc interface
	#[error("Rpc client error: {0}")]
	RpcError(String),
	/// Custom error
	#[error("{0}")]
	Custom(String),
	/// Decode error
	#[error("Decode error: {0}")]
	DecodeError(#[from] DecodeError),
	/// Encode error
	#[error("Encode error: {0}")]
	EncodeError(#[from] prost::EncodeError),
	/// Encode error
	#[error("Encode error: {0}")]
	EncodeError2(#[from] prost_12::EncodeError),
	/// Decode error
	#[error("Decode error: {0}")]
	DecodeError2(#[from] prost_12::DecodeError),
	/// Parse timestamp error
	#[error("Parse timestamp error: {0}")]
	ParseTimestampError(#[from] ParseTimestampError),
	/// Transfer error
	#[error("IBC transfer error: {0}")]
	TransferError(#[from] ibc::applications::transfer::error::Error),
	/// Client error
	#[error("IBC client error: {0}")]
	ClientError(#[from] ibc::core::ics02_client::error::Error),
	/// Tendermint error
	#[error("Tendermint error: {0}")]
	TendermintError(#[from] tendermint::Error),
	/// SQLx error
	#[error("SQLx error: {0}")]
	SQLxError(#[from] sqlx::Error),
	/// Anchor error
	#[error("Anchor client error: {0}")]
	AnchorError(#[from] anchor_client::ClientError),
	/// Anchor error
	#[error("Solana client client error: {0}")]
	SolanaClientError(#[from] anchor_client::solana_client::client_error::ClientError),
	/// Tonic Status error
	#[error("Tonic status error: {0}")]
	TonicStatusError(#[from] tonic_0_10::Status),
	/// Parse signature error
	#[error("Parse signature error: {0}")]
	ParseSignatureError(#[from] ParseSignatureError),
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
