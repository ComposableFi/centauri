use base64::{engine::general_purpose, Engine as _};

use crate::error::Error;
use ics07_tendermint::client_message::Header;
use tendermint::{
	block::{signed_header::SignedHeader, Commit, CommitSig},
	crypto::{default::signature::Verifier, signature::Verifier as _},
	signature::Signature,
	PublicKey,
};

/// Utilty function that:
/// - collects {signature, pubkey, msg} from every validator that successfuly signed a block so that
/// - there's consensus (voting power > 2/3) and
/// - size of the validators that succesfully voted equals the circuit size

pub fn collect_signatures_for_finalized_block(
	header: Header,
	circuit_size: usize,
) -> Option<Vec<(PublicKey, Signature, Vec<u8>)>> {
	let Header {
		validator_set,
		signed_header: SignedHeader { commit: Commit { signatures, .. }, .. },
		..
	} = header;

	let total_voting_power = validator_set.total_voting_power().value();

	// filter by valid signatures
	let mut validator_info_signed_block = signatures
		.iter()
		.filter(|commit| matches!(commit, CommitSig::BlockIdFlagCommit { .. }))
		.flat_map(|commit| match commit {
			CommitSig::BlockIdFlagCommit { validator_address, signature, .. }
				if signature.is_some() =>
			{
				match validator_set
					.validators()
					.iter()
					.find(|info| *validator_address == info.address)
				{
					None => None,
					Some(info) =>
						Some((info.pub_key, signature.clone().unwrap(), commit, info.power)),
				}
			},
			_ => unreachable!(),
		})
		.collect::<Vec<_>>();

	// order by power DESC
	validator_info_signed_block
		.sort_by(|(_, _, _, power_a), (_, _, _, power_b)| power_b.value().cmp(&power_a.value()));

	let result = validator_info_signed_block.into_iter().fold(
		(0u64, vec![]),
		|mut acc, (pubkey, signature, _commit, power)| {
			if acc.0 * 3 > 2 * total_voting_power && acc.1.len() == circuit_size {
				acc
			} else {
				// TOOD: commit has to become bytes, I guess? How to do so?
				// let msg = commit;
				let msg = vec![0u8];

				if Verifier::verify(pubkey, msg.as_ref(), &signature).is_ok() {
					acc.1.push((pubkey, signature, msg.to_vec()));
					acc.0 += power.value();
				}
				acc
			}
		},
	);

	if result.0 * 3 > 2 * total_voting_power && result.1.len() == circuit_size {
		Some(result.1)
	} else {
		None
	}
}

// TODO: this should be async
pub fn call_zk_prover(
	zk_prover_url: String,
	signatures: Vec<u8>,
	public_keys: Vec<u8>,
	messages: Vec<u8>,
) -> Result<Vec<u8>, Error> {
	let body: ProverResponse = ureq::get(zk_prover_url.as_ref())
		.call()
		.map_err(|e| Error::Custom(e.to_string()))?
		.into_json()
		.map_err(|e| Error::Custom(e.to_string()))?;

	if body.message.as_str() != "prover is busy" {
		let resp: ProverResponse = ureq::post(zk_prover_url.as_ref())
			.send_json(ureq::json!({
				"public_keys": public_keys,
				"signatures": signatures,
				"messages": messages,
			}))
			.map_err(|e| Error::Custom(e.to_string()))?
			.into_json()
			.map_err(|e| Error::Custom(e.to_string()))?;

		if &resp.message == "proof submitted" {
			let body: ProverResponse = ureq::get(zk_prover_url.as_ref())
				.call()
				.map_err(|e| Error::Custom(e.to_string()))?
				.into_json()
				.map_err(|e| Error::Custom(e.to_string()))?;

			// assume here that we got the proof
			// TODO: do proper re-check as the proof takes 5 minutes to be build
			return Ok(general_purpose::STANDARD_NO_PAD
				.decode(body.proof)
				.map_err(|e| Error::Custom(e.to_string()))?)
		}
	}
	Err(Error::Custom("could not get a proof".to_string()))
}

// note that we know the circuit size
#[derive(Debug, serde::Deserialize)]
struct ProverResponse {
	message: String,
	proof: String,
}
