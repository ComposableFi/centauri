use alloc::string::ToString;

macro_rules! import_proto {
	($Msg:ident) => {
		$crate::wrap!(cf_solana_upstream::proto::$Msg as $Msg);
		$crate::wrap!(impl Default for $Msg);

		impl prost::Message for $Msg {
			fn encode_raw<B: prost::bytes::BufMut>(&self, buf: &mut B) {
				prost_12::Message::encode_raw(&self.0, buf)
			}

			fn merge_field<B: prost::bytes::Buf>(
				&mut self,
				tag: u32,
				wire_type: prost::encoding::WireType,
				buf: &mut B,
				_ctx: prost::encoding::DecodeContext,
			) -> Result<(), prost::DecodeError> {
				// SAFETY: The types are identical in prost 0.11 and prost.12.
				let wire_type = unsafe {
					core::mem::transmute(wire_type as u8)
				};
				prost_12::Message::merge_field(&mut self.0, tag, wire_type, buf, Default::default())
					.map_err(|err| {
						// SAFETY: The types are identical in prost 0.11 and prost.12.
						unsafe {
							core::mem::transmute(err)
						}
					})
			}

			fn encoded_len(&self) -> usize {
				prost_12::Message::encoded_len(&self.0)
			}

			fn clear(&mut self) {
				prost_12::Message::clear(&mut self.0)
			}
		}
	}
}

import_proto!(ClientMessage);
import_proto!(ClientState);
import_proto!(ConsensusState);
import_proto!(Header);
import_proto!(Misbehaviour);

/// Error during decoding of a protocol message.
#[derive(Clone, PartialEq, Eq, derive_more::From)]
pub enum DecodeError {
	/// Failed decoding the wire encoded protocol message.
	///
	/// This means that the supplied bytes weren’t a valid protocol buffer or
	/// they didn’t correspond to the expected message.
	BadProto(alloc::string::String),

	/// Protocol message represents invalid state; see [`BadMessage`].
	#[from(ignore)]
	BadMessage,

	/// When decoding an `Any` message, the type URL doesn’t equal the expected
	/// one.
	#[from(ignore)]
	BadType,
}

impl From<cf_solana_upstream::proto::DecodeError> for DecodeError {
	fn from(err: cf_solana_upstream::proto::DecodeError) -> Self {
		match err {
			cf_solana_upstream::proto::DecodeError::BadProto(err) =>
				Self::BadProto(err.to_string()),
			cf_solana_upstream::proto::DecodeError::BadMessage => Self::BadMessage,
			cf_solana_upstream::proto::DecodeError::BadType => Self::BadType,
		}
	}
}

/// Error during validation of a protocol message.
///
/// Typing in protocol messages is less descriptive than in Rust.  It’s possible
/// to represent state in the protocol message which doesn’t correspond to
/// a valid state.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BadMessage;

impl From<cf_solana_upstream::proto::BadMessage> for BadMessage {
	fn from(_: cf_solana_upstream::proto::BadMessage) -> Self {
		Self
	}
}

impl From<BadMessage> for DecodeError {
	fn from(_: BadMessage) -> Self {
		Self::BadMessage
	}
}

impl core::fmt::Debug for DecodeError {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::BadProto(err) => err.fmt(fmtr),
			Self::BadMessage => fmtr.write_str("BadMessage"),
			Self::BadType => fmtr.write_str("BadType"),
		}
	}
}

impl core::fmt::Display for DecodeError {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Debug::fmt(self, fmtr)
	}
}

impl core::fmt::Display for BadMessage {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Debug::fmt(self, fmtr)
	}
}

impl From<Header> for ClientMessage {
	#[inline]
	fn from(msg: Header) -> Self {
		Self(cf_solana_upstream::proto::ClientMessage::from(msg.0))
	}
}

impl From<Misbehaviour> for ClientMessage {
	#[inline]
	fn from(msg: Misbehaviour) -> Self {
		Self(cf_solana_upstream::proto::ClientMessage::from(msg.0))
	}
}
