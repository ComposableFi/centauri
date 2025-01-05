#![allow(clippy::unit_arg, clippy::comparison_chain)]
#![no_std]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

pub mod client;
pub mod client_def;
mod client_impls;
pub mod consensus;
mod error;
pub mod header;
pub mod message;
mod misbehaviour;
pub mod proto;

use alloc::string::ToString;
// pub use client::impls::{CommonContext, Neighbourhood};
pub use client::ClientState;
pub use consensus::ConsensusState;
pub use header::Header;
pub use message::ClientMessage;
pub use misbehaviour::Misbehaviour;
// pub use proof::IbcProof;

use ibc::core::ics02_client::error::Error as ClientError;

/// Client type of the Solana blockchain’s light client.
pub const CLIENT_TYPE: &str = "cf-solana";

pub use crate::proto::{BadMessage, DecodeError};

impl From<DecodeError> for ClientError {
	fn from(err: DecodeError) -> Self {
		ClientError::implementation_specific(err.to_string())
	}
}

impl From<BadMessage> for ClientError {
	fn from(_: BadMessage) -> Self {
		ClientError::implementation_specific("BadMessage".to_string())
	}
}

/// Returns digest of the value.
///
/// This is used, among other places, as packet commitment.
#[inline]
pub fn digest(value: &[u8]) -> lib::hash::CryptoHash {
	lib::hash::CryptoHash::digest(value)
}

/// Returns digest of the value with client id mixed in.
///
/// We don’t store full client id in the trie key for paths which include
/// client id.  To avoid accepting malicious proofs, we must include it in
/// some other way.  We do this by mixing in the client id into the hash of
/// the value stored at the path.
///
/// Specifically, this calculates `digest(client_id || b'0' || serialised)`.
#[inline]
pub fn digest_with_client_id(
	client_id: &ibc::core::ics24_host::identifier::ClientId,
	value: &[u8],
) -> lib::hash::CryptoHash {
	lib::hash::CryptoHash::digestv(&[client_id.as_bytes(), b"\0", value])
}

macro_rules! wrap {
	($($Inner:ident)::* as $Outer:ident) => {
		#[derive(Clone, derive_more::From, derive_more::Into)]
		#[repr(transparent)]
		pub struct $Outer(pub $($Inner)::*);

		impl core::fmt::Debug for $Outer {
			fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
				self.0.fmt(fmtr)
			}
		}

		impl From<$Outer> for ibc_proto::google::protobuf::Any {
			fn from(msg: $Outer) -> Self {
				Self::from(&msg)
			}
		}

		impl From<&$Outer> for ibc_proto::google::protobuf::Any {
			fn from(msg: &$Outer) -> Self {
				let any = cf_solana_upstream::proto::Any::from(&msg.0);
				Self {
					type_url: any.type_url,
					value: any.value
				}
			}
		}

		impl TryFrom<ibc_proto::google::protobuf::Any> for $Outer {
			type Error = $crate::DecodeError;
			fn try_from(any: ibc_proto::google::protobuf::Any) -> Result<Self, Self::Error> {
				Self::try_from(&any)
			}
		}

		impl TryFrom<&ibc_proto::google::protobuf::Any> for $Outer {
			type Error = $crate::DecodeError;
			fn try_from(any: &ibc_proto::google::protobuf::Any) -> Result<Self, Self::Error> {
				Ok(Self(cf_solana_upstream::proto::AnyConvert::try_from_any(&any.type_url, &any.value)?))
			}
		}
	};

	($($Inner:ident)::* as $Outer:ident) => {
		#[derive(Clone, PartialEq, Eq, derive_more::From, derive_more::Into)]
		#[repr(transparent)]
		pub struct $Outer(pub $($Inner)::*);

		impl<PK: guestchain::PubKey + core::fmt::Debug> core::fmt::Debug for $Outer {
			fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
				self.0.fmt(fmtr)
			}
		}

		impl From<$Outer> for ibc_proto::google::protobuf::Any {
			fn from(msg: $Outer) -> Self {
				Self::from(&msg)
			}
		}

		impl From<&$Outer> for ibc_proto::google::protobuf::Any {
			fn from(msg: &$Outer) -> Self {
				let any = cf_solana_upstream::proto::Any::from(&msg.0);
				Self {
					type_url: any.type_url,
					value: any.value
				}
			}
		}

		impl TryFrom<ibc_proto::google::protobuf::Any> for $Outer {
			type Error = $crate::DecodeError;
			fn try_from(any: ibc_proto::google::protobuf::Any) -> Result<Self, Self::Error> {
				Self::try_from(&any)
			}
		}

		impl TryFrom<&ibc_proto::google::protobuf::Any> for $Outer {
			type Error = $crate::DecodeError;
			fn try_from(any: &ibc_proto::google::protobuf::Any) -> Result<Self, Self::Error> {
				Ok(Self(cf_solana_upstream::proto::AnyConvert::try_from_any(&any.type_url, &any.value)?))
			}
		}
	};

	(impl Default for $Outer:ident) => {
		impl Default for $Outer {
			fn default() -> Self { Self(Default::default()) }
		}
	};

	(impl Default for $Outer:ident) => {
		impl<PK: Default> Default for $Outer {
			fn default() -> Self { Self(Default::default()) }
		}
	};

	(impl Eq for $Outer:ident) => {
		impl core::cmp::PartialEq for $Outer {
			fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
		}
		impl core::cmp::Eq for $Outer { }
	};

	(impl proto for $Type:ident) => {
		impl $crate::proto::$Type {
			pub const IBC_TYPE_URL: &'static str =
				cf_solana_upstream::proto::$Type::IBC_TYPE_URL;
		}

		impl From<$Type> for $crate::proto::$Type {
			fn from(msg: $Type) -> Self {
				Self(cf_solana_upstream::proto::$Type::from(&msg.0))
			}
		}

		impl From<&$Type> for $crate::proto::$Type {
			fn from(msg: &$Type) -> Self {
				Self(cf_solana_upstream::proto::$Type::from(&msg.0))
			}
		}

		impl TryFrom<$crate::proto::$Type> for $Type {
			type Error = $crate::proto::BadMessage;
			fn try_from(msg: $crate::proto::$Type) -> Result<Self, Self::Error> {
				Self::try_from(&msg)
			}
		}

		impl TryFrom<&$crate::proto::$Type> for $Type {
			type Error = $crate::proto::BadMessage;
			fn try_from(msg: &$crate::proto::$Type) -> Result<Self, Self::Error> {
				Ok(Self(cf_solana_upstream::$Type::try_from(&msg.0)?))
			}
		}

		impl ibc::protobuf::Protobuf<$crate::proto::$Type> for $Type {}
	};

	(impl proto for $Type:ident) => {
		impl $crate::proto::$Type {
			pub const IBC_TYPE_URL: &'static str =
				cf_solana_upstream::proto::$Type::IBC_TYPE_URL;
		}

		impl From<$Type> for $crate::proto::$Type {
			fn from(msg: $Type) -> Self {
				Self(cf_solana_upstream::proto::$Type::from(&msg.0))
			}
		}

		impl From<&$Type> for $crate::proto::$Type {
			fn from(msg: &$Type) -> Self {
				Self(cf_solana_upstream::proto::$Type::from(&msg.0))
			}
		}

		impl TryFrom<$crate::proto::$Type> for $Type {
			type Error = $crate::proto::BadMessage;
			fn try_from(msg: $crate::proto::$Type) -> Result<Self, Self::Error> {
				Self::try_from(&msg)
			}
		}

		impl TryFrom<&$crate::proto::$Type> for $Type {
			type Error = $crate::proto::BadMessage;
			fn try_from(msg: &$crate::proto::$Type) -> Result<Self, Self::Error> {
				Ok(Self(cf_solana_upstream::$Type::try_from(&msg.0)?))
			}
		}

		impl ibc::protobuf::Protobuf<$crate::proto::$Type> for $Type {}
	};
}

use wrap;
