use std::str::FromStr;

use ibc::{
	core::{
		ics02_client::trust_threshold::TrustThreshold, ics23_commitment::specs::ProofSpecs,
		ics24_host::identifier::ChainId,
	},
	mock::header::MockHeader,
	Height,
};
use ibc_proto_new::ibc::lightclients::tendermint::v1::{ClientState, Fraction};
use pallet_ibc::light_clients::AnyClientState;
use tendermint_proto::Protobuf;

pub fn convert_new_client_state_to_old(
	client_state: solana_ibc::client_state::AnyClientState,
) -> AnyClientState {
	match client_state {
		solana_ibc::client_state::AnyClientState::Tendermint(client) => {
			let inner_client = client.inner();
			log::info!("This is latest height on solana {:?}", inner_client.latest_height);
			AnyClientState::Tendermint(ics07_tendermint::client_state::ClientState {
				chain_id: ChainId::from_str(inner_client.chain_id.as_str()).unwrap(),
				trust_level: TrustThreshold::new(
					inner_client.trust_level.numerator(),
					inner_client.trust_level.denominator(),
				)
				.unwrap(),
				trusting_period: inner_client.trusting_period,
				unbonding_period: inner_client.unbonding_period,
				max_clock_drift: inner_client.max_clock_drift,
				latest_height: Height::new(
					inner_client.latest_height.revision_number(),
					inner_client.latest_height.revision_height(),
				),
				proof_specs: ProofSpecs::cosmos(), // Not sure about this
				upgrade_path: inner_client.upgrade_path.clone(),
				frozen_height: inner_client.frozen_height.and_then(|height| {
					Some(Height::new(height.revision_number(), height.revision_height()))
				}),
				_phantom: std::marker::PhantomData,
			})
		},
		// solana_ibc::client_state::AnyClientState::Mock(client) =>
		// 	AnyClientState::Mock(ibc::mock::client_state::MockClientState {
		// 		header: MockHeader {
		// 			height: Height::new(
		// 				client.header.height.revision_number(),
		// 				client.header.height.revision_height(),
		// 			),
		// 			timestamp: ibc::timestamp::Timestamp::from_nanoseconds(
		// 				client.header.timestamp.nanoseconds(),
		// 			)
		// 			.unwrap(),
		// 		},
		// 		frozen_height: client.frozen_height.and_then(|height| {
		// 			Some(Height::new(height.revision_number(), height.revision_height()))
		// 		}),
		// 	}),
		// solana_ibc::client_state::AnyClientState::Mock(_) => unimplemented!(),
		solana_ibc::client_state::AnyClientState::Wasm(_) => unimplemented!(),
		solana_ibc::client_state::AnyClientState::Rollup(cs) =>
			AnyClientState::Rollup(cf_solana::ClientState(cf_solana_og::ClientState {
				latest_slot: cs.latest_slot,
				witness_account: cs.witness_account,
				trusting_period_ns: cs.trusting_period_ns,
				is_frozen: cs.is_frozen,
			})),
		solana_ibc::client_state::AnyClientState::Guest(cs) =>
			AnyClientState::Guest(cf_guest::ClientState(cf_guest_og::ClientState::new(
				cs.genesis_hash,
				cs.latest_height,
				cs.trusting_period_ns,
				cs.epoch_commitment,
				Some(cs.prev_epoch_commitment),
				cs.is_frozen,
			))),
		_ => unimplemented!(),
	}
}

pub fn convert_old_client_state_to_new(
	client_state: AnyClientState,
) -> solana_ibc::client_state::AnyClientState {
	match client_state {
		#[allow(deprecated)]
		AnyClientState::Tendermint(cs) => solana_ibc::client_state::AnyClientState::Tendermint(
			ClientState {
				chain_id: cs.chain_id.to_string(),
				trust_level: Some(Fraction {
					numerator: cs.trust_level.numerator(),
					denominator: cs.trust_level.denominator(),
				}),
				trusting_period: Some(cs.trusting_period.into()),
				unbonding_period: Some(cs.unbonding_period.into()),
				max_clock_drift: Some(cs.max_clock_drift.into()),
				frozen_height: cs.frozen_height.map_or(
					Some(ibc_proto_new::ibc::core::client::v1::Height {
						revision_height: 0,
						revision_number: 0,
					}),
					|height| {
						Some(ibc_proto_new::ibc::core::client::v1::Height {
							revision_number: height.revision_number,
							revision_height: height.revision_height,
						})
					},
				),
				latest_height: Some(ibc_proto_new::ibc::core::client::v1::Height {
					revision_number: cs.latest_height.revision_number,
					revision_height: cs.latest_height.revision_height,
				}),
				proof_specs: ibc_core_commitment_types::specs::ProofSpecs::cosmos().into(),
				upgrade_path: cs.upgrade_path,
				allow_update_after_expiry: false,
				allow_update_after_misbehaviour: false,
			}
			.try_into()
			.unwrap(),
		),
		#[allow(deprecated)]
		AnyClientState::Rollup(cs) => {
			let cs = cs.0;
			solana_ibc::client_state::AnyClientState::Rollup(cf_solana_og::ClientState {
				latest_slot: cs.latest_slot,
				witness_account: cs.witness_account,
				trusting_period_ns: cs.trusting_period_ns,
				is_frozen: cs.is_frozen,
			})
		},
		AnyClientState::Guest(cs) => {
			let cs = cs.0;
			solana_ibc::client_state::AnyClientState::Guest(cf_guest_og::ClientState::new(
				cs.genesis_hash,
				cs.latest_height,
				cs.trusting_period_ns,
				cs.epoch_commitment,
				Some(cs.prev_epoch_commitment),
				cs.is_frozen,
			))
		},
		// AnyClientState::Mock(cs) =>
		// 	solana_ibc::client_state::AnyClientState::Mock(MockClientState {
		// 		header: ibc_testkit::testapp::ibc::clients::mock::header::MockHeader {
		// 			height: ibc_core_client_types::Height::new(
		// 				cs.header.height().revision_number,
		// 				cs.header.height().revision_height,
		// 			)
		// 			.unwrap(),
		// 			timestamp: ibc_new_primitives::Timestamp::from_nanoseconds(
		// 				cs.header.timestamp.nanoseconds(),
		// 			)
		// 			.unwrap(),
		// 		},
		// 		frozen_height: cs.frozen_height.and_then(|height| {
		// 			Some(
		// 				ibc_core_client_types::Height::new(
		// 					height.revision_number,
		// 					height.revision_height,
		// 				)
		// 				.unwrap(),
		// 			)
		// 		}),
		// 	}),
		#[allow(deprecated)]
		AnyClientState::Wasm(cs) => {
			let cs = AnyClientState::decode_vec(&cs.data).unwrap();
			println!("This is tendermint\n {:?}", cs);
			match cs {
				AnyClientState::Tendermint(e) => {
					log::info!(
						"This is default {:?}",
						ibc_core_commitment_types::specs::ProofSpecs::cosmos()
					);
					log::info!("This is from client state {:?}", e.proof_specs);
					solana_ibc::client_state::AnyClientState::Tendermint(
						ClientState {
							chain_id: e.chain_id.to_string(),
							trust_level: Some(Fraction {
								numerator: e.trust_level.numerator(),
								denominator: e.trust_level.denominator(),
							}),
							trusting_period: Some(e.trusting_period.into()),
							unbonding_period: Some(e.unbonding_period.into()),
							max_clock_drift: Some(e.max_clock_drift.into()),
							frozen_height: e.frozen_height.and_then(|height| {
								Some(ibc_proto_new::ibc::core::client::v1::Height {
									revision_number: height.revision_number,
									revision_height: height.revision_height,
								})
							}),
							latest_height: Some(ibc_proto_new::ibc::core::client::v1::Height {
								revision_number: e.latest_height.revision_number,
								revision_height: e.latest_height.revision_height,
							}),
							proof_specs: ibc_core_commitment_types::specs::ProofSpecs::cosmos()
								.into(),
							upgrade_path: e.upgrade_path,
							allow_update_after_expiry: false,
							allow_update_after_misbehaviour: false,
						}
						.try_into()
						.unwrap(),
					)
				},
				_ => panic!("Invalid state {:?}", cs),
			}
		},
		_ => panic!("Client state not supported"),
	}
}
