use core::str::FromStr;

use guestchain::BlockHeader;
use ibc_core_host_types::path::{
	AckPath, ChannelEndPath, ClientConnectionPath, CommitmentPath, ConnectionPath, PortPath,
	ReceiptPath, SeqAckPath, SeqRecvPath, SeqSendPath,
};

mod ibc {
	pub use ibc::core::{
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::path,
	};
}

pub use cf_guest_upstream::proof::{GenerateError, IbcProof, VerifyError};

/// Generates a proof for given path.
///
/// `block_header` is header whose hash will be the commitment root.  It’s
/// state root must correspond to `trie`’s root.  `path` specifies IBC path
/// of the value that needs proof.
///
/// # Proof format
///
/// In most cases, proof is Borsh-serialised `(guestchain::BlockHeader,
/// sealable_trie::proof::Proof)` pair.  The header at the front is necessary to
/// determine state root (recall that `root` is the block hash and not state
/// root).
///
/// However, if `path` is one of `SeqSend`, `SeqRecv` or `SeqAck` than proof
/// further contain two big-endian encoded `u64` numbers holding the other
/// two sequence numbers.
///
/// For example, if `path` is `SeqRecv`, the `proof` must at the end include
/// send sequence number and ack sequence number.  For example, if next send
/// sequence is `7`, next ack sequence is `5` and path is `SeqRecv` the
/// proof will end with `be(7) || be(5)` (where `be` denotes encoding 64-bit
/// number as big endian).
///
/// This addition is necessary because sequence numbers are stored together
/// within a single trie value.  For example, proving the next receive
/// sequence is `4` requires proving `be(7), be(4), be(5), be(0)].  For
/// verifier to know what value it checks, it needs to be provided all of
/// the sequence numbers.
///
/// (Note that Borsh uses little endian to encode integers so the sequence
/// numbers cannot be simply borsh deserialised.)
pub fn generate<A: sealable_trie::Allocator>(
	block_header: &BlockHeader,
	trie: &sealable_trie::Trie<A>,
	path: ibc::path::Path,
) -> Result<IbcProof, GenerateError> {
	let path = convert_old_path_to_new(path);
	cf_guest_upstream::proof::generate_for_block(block_header, trie, path)
}

/// Verifies a proof for given entry or lack of entry.
///
/// `prefix` must be empty, `proof` and `root` must follow format described in
/// [`generate`] function.  `path` indicates IBC path the proof is for and
/// `value` determines value or lack thereof expected at the path.
///
/// # Value hash
///
/// Since sealable trie doesn’t store values but only hashes, when verifying
/// membership proofs the value needs to be converted into a hash.  There are
/// three cases:
///
/// 1. If `path` includes client id, the hash of the value is calculated with the client id mixed
///    in; see [`super::digest_with_client_id`] function.
///
/// 2. If `path` is `SeqSend`, `SeqRecv` or `SeqAck`, the `value` must be
///    `google.protobuf.UInt64Value` protobuf and hash is calculated as concatenation of the three
///    sequence numbers as described in [`generate`].
///
/// 3. Otherwise, the value is simply hashed.
pub fn verify(
	prefix: &ibc::CommitmentPrefix,
	proof: &ibc::CommitmentProofBytes,
	root: &ibc::CommitmentRoot,
	path: ibc::path::Path,
	value: Option<&[u8]>,
) -> Result<(), VerifyError> {
	verify_bytes(prefix.as_bytes(), proof.as_bytes(), root.as_bytes(), path, value)
}

/// Verifies a proof for given entry or lack of entry.
///
/// Like [`verify`] but takes slice arguments rather than IBC types.
pub fn verify_bytes(
	prefix: &[u8],
	proof: &[u8],
	root: &[u8],
	path: ibc::path::Path,
	value: Option<&[u8]>,
) -> Result<(), VerifyError> {
	cf_guest_upstream::proof::verify_for_block(
		prefix,
		proof,
		root,
		convert_old_path_to_new(path),
		value,
	)
}

fn convert_old_path_to_new(path: ibc::path::Path) -> ibc_core_host_types::path::Path {
	match path {
		::ibc::core::ics24_host::Path::ClientType(_) => panic!("Not supported"),
		::ibc::core::ics24_host::Path::ClientState(e) =>
			ibc_core_host_types::path::Path::ClientState(
				ibc_core_host_types::path::ClientStatePath(
					ibc_core_host_types::identifiers::ClientId::from_str(e.0.as_str()).unwrap(),
				),
			),
		::ibc::core::ics24_host::Path::ClientConsensusState(e) =>
			ibc_core_host_types::path::Path::ClientConsensusState(
				ibc_core_host_types::path::ClientConsensusStatePath {
					client_id: ibc_core_host_types::identifiers::ClientId::from_str(
						e.client_id.as_str(),
					)
					.unwrap(),
					revision_number: e.epoch,
					revision_height: e.height,
				},
			),
		::ibc::core::ics24_host::Path::ClientConnections(e) =>
			ibc_core_host_types::path::Path::ClientConnection(ClientConnectionPath(
				ibc_core_host_types::identifiers::ClientId::from_str(e.0.as_str()).unwrap(),
			)),
		::ibc::core::ics24_host::Path::Connections(e) =>
			ibc_core_host_types::path::Path::Connection(ConnectionPath(
				ibc_core_host_types::identifiers::ConnectionId::from_str(e.0.as_str()).unwrap(),
			)),
		::ibc::core::ics24_host::Path::Ports(e) => ibc_core_host_types::path::Path::Ports(
			PortPath(ibc_core_host_types::identifiers::PortId::from_str(e.0.as_str()).unwrap()),
		),
		::ibc::core::ics24_host::Path::ChannelEnds(e) =>
			ibc_core_host_types::path::Path::ChannelEnd(ChannelEndPath(
				ibc_core_host_types::identifiers::PortId::from_str(e.0.as_str()).unwrap(),
				ibc_core_host_types::identifiers::ChannelId::new(e.1.sequence()),
			)),
		::ibc::core::ics24_host::Path::SeqSends(e) =>
			ibc_core_host_types::path::Path::SeqSend(SeqSendPath(
				ibc_core_host_types::identifiers::PortId::from_str(e.0.as_str()).unwrap(),
				ibc_core_host_types::identifiers::ChannelId::new(e.1.sequence()),
			)),
		::ibc::core::ics24_host::Path::SeqRecvs(e) =>
			ibc_core_host_types::path::Path::SeqRecv(SeqRecvPath(
				ibc_core_host_types::identifiers::PortId::from_str(e.0.as_str()).unwrap(),
				ibc_core_host_types::identifiers::ChannelId::new(e.1.sequence()),
			)),
		::ibc::core::ics24_host::Path::SeqAcks(e) =>
			ibc_core_host_types::path::Path::SeqAck(SeqAckPath(
				ibc_core_host_types::identifiers::PortId::from_str(e.0.as_str()).unwrap(),
				ibc_core_host_types::identifiers::ChannelId::new(e.1.sequence()),
			)),
		::ibc::core::ics24_host::Path::Commitments(e) =>
			ibc_core_host_types::path::Path::Commitment(CommitmentPath {
				port_id: ibc_core_host_types::identifiers::PortId::from_str(e.port_id.as_str())
					.unwrap(),
				channel_id: ibc_core_host_types::identifiers::ChannelId::new(
					e.channel_id.sequence(),
				),
				sequence: e.sequence.0.into(),
			}),
		::ibc::core::ics24_host::Path::Acks(e) => ibc_core_host_types::path::Path::Ack(AckPath {
			port_id: ibc_core_host_types::identifiers::PortId::from_str(e.port_id.as_str())
				.unwrap(),
			channel_id: ibc_core_host_types::identifiers::ChannelId::new(e.channel_id.sequence()),
			sequence: e.sequence.0.into(),
		}),
		::ibc::core::ics24_host::Path::Receipts(e) =>
			ibc_core_host_types::path::Path::Receipt(ReceiptPath {
				port_id: ibc_core_host_types::identifiers::PortId::from_str(e.port_id.as_str())
					.unwrap(),
				channel_id: ibc_core_host_types::identifiers::ChannelId::new(
					e.channel_id.sequence(),
				),
				sequence: e.sequence.0.into(),
			}),
		::ibc::core::ics24_host::Path::Upgrade(path) => {
			use ::ibc::core::ics24_host::ClientUpgradePath;
			use ibc_core_host_types::path::UpgradeClientPath;
			match path {
				ClientUpgradePath::UpgradedClientState(height) =>
					UpgradeClientPath::UpgradedClientState(height),
				ClientUpgradePath::UpgradedClientConsensusState(height) =>
					UpgradeClientPath::UpgradedClientConsensusState(height),
			}
			.into()
		},
		::ibc::core::ics24_host::Path::Outside(e) => panic!("Not supported {:?}", e),
	}
}
