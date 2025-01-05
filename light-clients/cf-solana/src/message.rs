use prost::Message as _;

use crate::proto;

super::wrap!(cf_solana_upstream::ClientMessage as ClientMessage);
super::wrap!(impl proto for ClientMessage);

impl ClientMessage {
	pub fn maybe_header_height(&self) -> Option<ibc::Height> {
		if let cf_solana_upstream::ClientMessage::Header(hdr) = &self.0 {
			let height = hdr.slot;
			Some(ibc::Height::new(1, height.into()))
		} else {
			None
		}
	}
}

impl ibc::core::ics02_client::client_message::ClientMessage for ClientMessage {
	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, ibc::protobuf::Error> {
		Ok(proto::ClientMessage::from(self).encode_to_vec())
	}
}

impl From<cf_solana_upstream::Header> for ClientMessage {
	fn from(hdr: cf_solana_upstream::Header) -> Self {
		Self(cf_solana_upstream::ClientMessage::Header(hdr))
	}
}

impl From<crate::Header> for ClientMessage {
	fn from(hdr: crate::Header) -> Self {
		Self(cf_solana_upstream::ClientMessage::Header(hdr.0))
	}
}

impl From<cf_solana_upstream::Misbehaviour> for ClientMessage {
	fn from(msg: cf_solana_upstream::Misbehaviour) -> Self {
		Self(cf_solana_upstream::ClientMessage::Misbehaviour(msg))
	}
}

impl From<crate::Misbehaviour> for ClientMessage {
	fn from(msg: crate::Misbehaviour) -> Self {
		Self(cf_solana_upstream::ClientMessage::Misbehaviour(msg.0))
	}
}
