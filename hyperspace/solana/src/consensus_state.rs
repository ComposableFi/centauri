use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc_proto_new::{
	google::protobuf::Timestamp, ibc::lightclients::tendermint::v1::ConsensusState,
};
use pallet_ibc::light_clients::AnyConsensusState;
use tendermint::Hash;

pub fn convert_new_consensus_state_to_old(
	consensus_state: solana_ibc::consensus_state::AnyConsensusState,
) -> AnyConsensusState {
	match consensus_state {
		solana_ibc::consensus_state::AnyConsensusState::Tendermint(cs) => {
			let timestamp_in_secs = cs.timestamp().unix_timestamp();
			let remaining_timestamp_in_nano =
				(cs.timestamp().unix_timestamp_nanos() % 1_000_000_000) as u32;
			AnyConsensusState::Tendermint(ics07_tendermint::consensus_state::ConsensusState {
				timestamp: tendermint::time::Time::from_unix_timestamp(
					timestamp_in_secs,
					remaining_timestamp_in_nano,
				)
				.unwrap(),
				root: CommitmentRoot { bytes: cs.inner().root.as_bytes().to_vec() },
				next_validators_hash: Hash::try_from(cs.next_validators_hash().as_bytes().to_vec())
					.unwrap(),
			})
		},
		// solana_ibc::consensus_state::AnyConsensusState::Mock(cs) =>
		// 	AnyConsensusState::Mock(ibc::mock::client_state::MockConsensusState {
		// 		header: MockHeader {
		// 			height: Height::new(
		// 				cs.header.height.revision_number(),
		// 				cs.header.height.revision_height(),
		// 			),
		// 			timestamp: ibc::timestamp::Timestamp::from_nanoseconds(
		// 				cs.header.timestamp.nanoseconds(),
		// 			)
		// 			.unwrap(),
		// 		},
		// 		root: CommitmentRoot { bytes: cs.root.into_vec() },
		// 	}),
		solana_ibc::consensus_state::AnyConsensusState::Wasm(_) => {
			panic!("Guest consensus not supported")
		},
		solana_ibc::consensus_state::AnyConsensusState::Rollup(cs) =>
			AnyConsensusState::Rollup(cf_solana::ConsensusState(cf_solana_og::ConsensusState {
				trie_root: cs.trie_root,
				timestamp_sec: cs.timestamp_sec,
			})),
		_ => unimplemented!(),
	}
}

pub fn convert_old_consensus_state_to_new(
	consensus_state: AnyConsensusState,
) -> solana_ibc::consensus_state::AnyConsensusState {
	match consensus_state {
		AnyConsensusState::Tendermint(cs) => {
			let timestamp_in_secs = cs.timestamp.unix_timestamp();
			let remaining_timestamp_in_nano =
				(cs.timestamp.unix_timestamp_nanos() % 1_000_000_000) as i32;
			solana_ibc::consensus_state::AnyConsensusState::Tendermint(
				ConsensusState {
					timestamp: Some(Timestamp {
						seconds: timestamp_in_secs,
						nanos: remaining_timestamp_in_nano,
					}),
					root: Some(ibc_proto_new::ibc::core::commitment::v1::MerkleRoot {
						hash: cs.root.bytes,
					}),
					next_validators_hash: cs.next_validators_hash.as_bytes().to_vec(),
				}
				.try_into()
				.unwrap(),
			)
		},
		AnyConsensusState::Rollup(cs) => {
			let cs = cs.0;
			solana_ibc::consensus_state::AnyConsensusState::Rollup(cf_solana_og::ConsensusState {
				trie_root: cs.trie_root,
				timestamp_sec: cs.timestamp_sec,
			})
		},
		// AnyConsensusState::Mock(cs) => solana_ibc::consensus_state::AnyConsensusState::Mock(
		// 	ibc_testkit::testapp::ibc::clients::mock::consensus_state::MockConsensusState {
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
		// 		root: ibc_core_commitment_types::commitment::CommitmentRoot::from_bytes(
		// 			cs.root.as_bytes(),
		// 		),
		// 	},
		// ),
		_ => panic!("Client state not supported"),
	}
}
