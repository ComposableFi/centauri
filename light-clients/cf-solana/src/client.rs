use core::num::NonZeroU64;

use crate::{client_def::SolanaClient, error::Error, CLIENT_TYPE};
use alloc::string::{String, ToString};
use ibc::{
	core::{ics02_client::height::Height, ics24_host::identifier::ClientId},
	timestamp::Timestamp,
};
use serde::{Deserialize, Serialize};

super::wrap!(cf_solana_upstream::ClientState as ClientState);
super::wrap!(impl proto for ClientState);
super::wrap!(impl Eq for ClientState);

impl ClientState {
	pub fn with_header(&self, header: &cf_solana_upstream::Header) -> Self {
		Self(self.0.with_header(header))
	}

	pub fn frozen(&self) -> Self {
		Self(self.0.frozen())
	}

	/// Verify the time and height delays
	pub fn verify_delay_passed(
		current_time: Timestamp,
		current_height: Height,
		processed_time: u64,
		processed_height: u64,
		delay_period_time: u64,
		delay_period_blocks: u64,
	) -> Result<(), Error> {
		let earliest_time = processed_time + delay_period_time;
		// NOTE: delay time period is inclusive, so if current_time is earliest_time, then we
		// return no error https://github.com/cosmos/ibc-go/blob/9ebc2f81049869bc40c443ffb72d9f3e47afb4fc/modules/light-clients/07-tendermint/client_state.go#L306
		if current_time.nanoseconds() < earliest_time {
			return Err(Error::NotEnoughTimeElapsed { current_time, earliest_time })
		}

		let earliest_height = processed_height + delay_period_blocks;
		if current_height.revision_height < earliest_height {
			return Err(Error::NotEnoughBlocksElapsed { current_height, earliest_height })
		}

		Ok(())
	}

	pub fn verify_height(&self, client_id: &ClientId, height: ibc::Height) -> Result<(), Error> {
		if u64::from(self.0.latest_slot) < height.revision_height {
			return Err(Error::InsufficientHeight {
				latest_height: Height::new(1, self.0.latest_slot.into()),
				target_height: height,
			})
		}

		if self.0.is_frozen {
			return Err(Error::ClientFrozen { client_id: client_id.clone() })
		}
		Ok(())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {}

impl ibc::core::ics02_client::client_state::ClientState for ClientState {
	type UpgradeOptions = UpgradeOptions;

	type ClientDef = SolanaClient;

	fn chain_id(&self) -> ibc::core::ics24_host::identifier::ChainId {
		ibc::core::ics24_host::identifier::ChainId::new(String::from("Mantis"), 0)
	}

	fn client_def(&self) -> Self::ClientDef {
		SolanaClient::default()
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		CLIENT_TYPE.to_string()
	}

	fn latest_height(&self) -> ibc::Height {
		Height::new(1, u64::from(self.0.latest_slot))
	}

	fn frozen_height(&self) -> Option<ibc::Height> {
		self.0.is_frozen.then(|| Height::new(1, u64::from(self.0.latest_slot)))
	}

	fn upgrade(
		mut self,
		upgrade_height: ibc::Height,
		_upgrade_options: Self::UpgradeOptions,
		_chain_id: ibc::core::ics24_host::identifier::ChainId,
	) -> Self {
		self.0.latest_slot = NonZeroU64::new(upgrade_height.revision_height).unwrap();
		self
	}

	fn expired(&self, elapsed: core::time::Duration) -> bool {
		elapsed.as_nanos() as u64 > self.0.trusting_period_ns
	}

	fn encode_to_vec(&self) -> Result<ibc::prelude::Vec<u8>, ibc::protobuf::Error> {
		Ok(self.0.encode())
	}
}
