#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 20usize] = [
		"System",
		"Timestamp",
		"ParachainSystem",
		"ParachainInfo",
		"Balances",
		"TransactionPayment",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"XcmpQueue",
		"PolkadotXcm",
		"CumulusXcm",
		"DmpQueue",
		"Sudo",
		"IbcPing",
		"Assets",
		"AssetRegistry",
		"Ibc",
	];
	#[doc = r" The error type returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Debug,
	)]
	# [codec (crate = :: subxt :: ext :: codec)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 2)]
		ParachainSystem(parachain_system::Event),
		#[codec(index = 10)]
		Balances(balances::Event),
		#[codec(index = 11)]
		TransactionPayment(transaction_payment::Event),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Event),
		#[codec(index = 22)]
		Session(session::Event),
		#[codec(index = 30)]
		XcmpQueue(xcmp_queue::Event),
		#[codec(index = 31)]
		PolkadotXcm(polkadot_xcm::Event),
		#[codec(index = 32)]
		CumulusXcm(cumulus_xcm::Event),
		#[codec(index = 33)]
		DmpQueue(dmp_queue::Event),
		#[codec(index = 35)]
		Sudo(sudo::Event),
		#[codec(index = 36)]
		IbcPing(ibc_ping::Event),
		#[codec(index = 37)]
		Assets(assets::Event),
		#[codec(index = 38)]
		AssetRegistry(asset_registry::Event),
		#[codec(index = 255)]
		Ibc(ibc::Event),
	}
	impl ::subxt::events::RootEvent for Event {
		fn root_event(
			pallet_bytes: &[u8],
			pallet_name: &str,
			pallet_ty: u32,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			if pallet_name == "System" {
				return Ok(Event::System(system::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "ParachainSystem" {
				return Ok(Event::ParachainSystem(parachain_system::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Balances" {
				return Ok(Event::Balances(balances::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "TransactionPayment" {
				return Ok(Event::TransactionPayment(
					transaction_payment::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "CollatorSelection" {
				return Ok(Event::CollatorSelection(
					collator_selection::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "Session" {
				return Ok(Event::Session(session::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "XcmpQueue" {
				return Ok(Event::XcmpQueue(xcmp_queue::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "PolkadotXcm" {
				return Ok(Event::PolkadotXcm(polkadot_xcm::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CumulusXcm" {
				return Ok(Event::CumulusXcm(cumulus_xcm::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "DmpQueue" {
				return Ok(Event::DmpQueue(dmp_queue::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Sudo" {
				return Ok(Event::Sudo(sudo::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "IbcPing" {
				return Ok(Event::IbcPing(ibc_ping::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Assets" {
				return Ok(Event::Assets(assets::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "AssetRegistry" {
				return Ok(Event::AssetRegistry(asset_registry::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Ibc" {
				return Ok(Event::Ibc(ibc::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Event enum",
				pallet_name
			))
			.into())
		}
	}
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Debug,
	)]
	# [codec (crate = :: subxt :: ext :: codec)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Call {
		#[codec(index = 0)]
		System(system::Call),
		#[codec(index = 1)]
		Timestamp(timestamp::Call),
		#[codec(index = 2)]
		ParachainSystem(parachain_system::Call),
		#[codec(index = 3)]
		ParachainInfo(parachain_info::Call),
		#[codec(index = 10)]
		Balances(balances::Call),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Call),
		#[codec(index = 22)]
		Session(session::Call),
		#[codec(index = 30)]
		XcmpQueue(xcmp_queue::Call),
		#[codec(index = 31)]
		PolkadotXcm(polkadot_xcm::Call),
		#[codec(index = 32)]
		CumulusXcm(cumulus_xcm::Call),
		#[codec(index = 33)]
		DmpQueue(dmp_queue::Call),
		#[codec(index = 35)]
		Sudo(sudo::Call),
		#[codec(index = 36)]
		IbcPing(ibc_ping::Call),
		#[codec(index = 37)]
		Assets(assets::Call),
		#[codec(index = 38)]
		AssetRegistry(asset_registry::Call),
		#[codec(index = 255)]
		Ibc(ibc::Call),
	}
	impl ::subxt::blocks::RootExtrinsic for Call {
		fn root_extrinsic(
			pallet_bytes: &[u8],
			pallet_name: &str,
			pallet_ty: u32,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			if pallet_name == "System" {
				return Ok(Call::System(system::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Timestamp" {
				return Ok(Call::Timestamp(timestamp::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "ParachainSystem" {
				return Ok(Call::ParachainSystem(parachain_system::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "ParachainInfo" {
				return Ok(Call::ParachainInfo(parachain_info::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Balances" {
				return Ok(Call::Balances(balances::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CollatorSelection" {
				return Ok(Call::CollatorSelection(collator_selection::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Session" {
				return Ok(Call::Session(session::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "XcmpQueue" {
				return Ok(Call::XcmpQueue(xcmp_queue::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "PolkadotXcm" {
				return Ok(Call::PolkadotXcm(polkadot_xcm::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CumulusXcm" {
				return Ok(Call::CumulusXcm(cumulus_xcm::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "DmpQueue" {
				return Ok(Call::DmpQueue(dmp_queue::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Sudo" {
				return Ok(Call::Sudo(sudo::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "IbcPing" {
				return Ok(Call::IbcPing(ibc_ping::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Assets" {
				return Ok(Call::Assets(assets::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "AssetRegistry" {
				return Ok(Call::AssetRegistry(asset_registry::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Ibc" {
				return Ok(Call::Ibc(ibc::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Call enum",
				pallet_name
			))
			.into())
		}
	}
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Debug,
	)]
	# [codec (crate = :: subxt :: ext :: codec)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Error {
		#[codec(index = 0)]
		System(system::Error),
		#[codec(index = 2)]
		ParachainSystem(parachain_system::Error),
		#[codec(index = 10)]
		Balances(balances::Error),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Error),
		#[codec(index = 22)]
		Session(session::Error),
		#[codec(index = 30)]
		XcmpQueue(xcmp_queue::Error),
		#[codec(index = 31)]
		PolkadotXcm(polkadot_xcm::Error),
		#[codec(index = 32)]
		CumulusXcm(cumulus_xcm::Error),
		#[codec(index = 33)]
		DmpQueue(dmp_queue::Error),
		#[codec(index = 35)]
		Sudo(sudo::Error),
		#[codec(index = 36)]
		IbcPing(ibc_ping::Error),
		#[codec(index = 37)]
		Assets(assets::Error),
		#[codec(index = 38)]
		AssetRegistry(asset_registry::Error),
		#[codec(index = 255)]
		Ibc(ibc::Error),
	}
	impl ::subxt::error::RootError for Error {
		fn root_error(
			pallet_bytes: &[u8],
			pallet_name: &str,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			let cursor = &mut &pallet_bytes[..];
			if pallet_name == "System" {
				let variant_error = system::Error::decode_with_metadata(cursor, 143u32, metadata)?;
				return Ok(Error::System(variant_error))
			}
			if pallet_name == "ParachainSystem" {
				let variant_error =
					parachain_system::Error::decode_with_metadata(cursor, 174u32, metadata)?;
				return Ok(Error::ParachainSystem(variant_error))
			}
			if pallet_name == "Balances" {
				let variant_error =
					balances::Error::decode_with_metadata(cursor, 189u32, metadata)?;
				return Ok(Error::Balances(variant_error))
			}
			if pallet_name == "CollatorSelection" {
				let variant_error =
					collator_selection::Error::decode_with_metadata(cursor, 197u32, metadata)?;
				return Ok(Error::CollatorSelection(variant_error))
			}
			if pallet_name == "Session" {
				let variant_error = session::Error::decode_with_metadata(cursor, 207u32, metadata)?;
				return Ok(Error::Session(variant_error))
			}
			if pallet_name == "XcmpQueue" {
				let variant_error =
					xcmp_queue::Error::decode_with_metadata(cursor, 225u32, metadata)?;
				return Ok(Error::XcmpQueue(variant_error))
			}
			if pallet_name == "PolkadotXcm" {
				let variant_error =
					polkadot_xcm::Error::decode_with_metadata(cursor, 266u32, metadata)?;
				return Ok(Error::PolkadotXcm(variant_error))
			}
			if pallet_name == "CumulusXcm" {
				let variant_error =
					cumulus_xcm::Error::decode_with_metadata(cursor, 268u32, metadata)?;
				return Ok(Error::CumulusXcm(variant_error))
			}
			if pallet_name == "DmpQueue" {
				let variant_error =
					dmp_queue::Error::decode_with_metadata(cursor, 274u32, metadata)?;
				return Ok(Error::DmpQueue(variant_error))
			}
			if pallet_name == "Sudo" {
				let variant_error = sudo::Error::decode_with_metadata(cursor, 291u32, metadata)?;
				return Ok(Error::Sudo(variant_error))
			}
			if pallet_name == "IbcPing" {
				let variant_error =
					ibc_ping::Error::decode_with_metadata(cursor, 292u32, metadata)?;
				return Ok(Error::IbcPing(variant_error))
			}
			if pallet_name == "Assets" {
				let variant_error = assets::Error::decode_with_metadata(cursor, 303u32, metadata)?;
				return Ok(Error::Assets(variant_error))
			}
			if pallet_name == "AssetRegistry" {
				let variant_error =
					asset_registry::Error::decode_with_metadata(cursor, 304u32, metadata)?;
				return Ok(Error::AssetRegistry(variant_error))
			}
			if pallet_name == "Ibc" {
				let variant_error = ibc::Error::decode_with_metadata(cursor, 315u32, metadata)?;
				return Ok(Error::Ibc(variant_error))
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Error enum",
				pallet_name
			))
			.into())
		}
	}
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub fn apis() -> runtime_apis::RuntimeApi {
		runtime_apis::RuntimeApi
	}
	pub mod runtime_apis {
		use super::{root_mod, runtime_types};
		use ::subxt::ext::codec::Encode;
		pub struct RuntimeApi;
		impl RuntimeApi {}
	}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn assets(&self) -> assets::constants::ConstantsApi {
			assets::constants::ConstantsApi
		}
		pub fn ibc(&self) -> ibc::constants::ConstantsApi {
			ibc::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
			parachain_system::storage::StorageApi
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
			parachain_info::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn collator_selection(&self) -> collator_selection::storage::StorageApi {
			collator_selection::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn aura(&self) -> aura::storage::StorageApi {
			aura::storage::StorageApi
		}
		pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
			aura_ext::storage::StorageApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
			xcmp_queue::storage::StorageApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi {
			polkadot_xcm::storage::StorageApi
		}
		pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi {
			dmp_queue::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn assets(&self) -> assets::storage::StorageApi {
			assets::storage::StorageApi
		}
		pub fn asset_registry(&self) -> asset_registry::storage::StorageApi {
			asset_registry::storage::StorageApi
		}
		pub fn ibc(&self) -> ibc::storage::StorageApi {
			ibc::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
			parachain_system::calls::TransactionApi
		}
		pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
			parachain_info::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
			collator_selection::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
			xcmp_queue::calls::TransactionApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi {
			polkadot_xcm::calls::TransactionApi
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi {
			cumulus_xcm::calls::TransactionApi
		}
		pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi {
			dmp_queue::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn ibc_ping(&self) -> ibc_ping::calls::TransactionApi {
			ibc_ping::calls::TransactionApi
		}
		pub fn assets(&self) -> assets::calls::TransactionApi {
			assets::calls::TransactionApi
		}
		pub fn asset_registry(&self) -> asset_registry::calls::TransactionApi {
			asset_registry::calls::TransactionApi
		}
		pub fn ibc(&self) -> ibc::calls::TransactionApi {
			ibc::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().hasher().only_these_pallets(&PALLETS).hash();
		if runtime_metadata_hash !=
			[
				112u8, 228u8, 60u8, 181u8, 133u8, 168u8, 190u8, 51u8, 84u8, 37u8, 201u8, 141u8,
				51u8, 115u8, 233u8, 19u8, 78u8, 195u8, 253u8, 94u8, 51u8, 83u8, 4u8, 167u8, 166u8,
				49u8, 104u8, 33u8, 65u8, 165u8, 198u8, 18u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleCodegen)
		} else {
			Ok(())
		}
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::frame_system::pallet::Error;
		pub type Call = runtime_types::frame_system::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Remark {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Remark {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetHeapPages {
					pub pages: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_heap_pages";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCode {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCode {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCodeWithoutChecks {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code_without_checks";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetStorage {
					pub items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillStorage {
					pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillPrefix {
					pub prefix: ::std::vec::Vec<::core::primitive::u8>,
					pub subkeys: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_prefix";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemarkWithEvent {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark_with_event";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::Remark> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark",
						types::Remark { remark },
						[
							43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
							216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
							250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
							13u8,
						],
					)
				}
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::SetHeapPages> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_heap_pages",
						types::SetHeapPages { pages },
						[
							188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
							215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
							134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
							57u8, 147u8,
						],
					)
				}
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCode> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code",
						types::SetCode { code },
						[
							233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
							203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
							27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
						],
					)
				}
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code_without_checks",
						types::SetCodeWithoutChecks { code },
						[
							82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
							157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
							147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
							115u8,
						],
					)
				}
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::Payload<types::SetStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_storage",
						types::SetStorage { items },
						[
							184u8, 169u8, 248u8, 68u8, 40u8, 193u8, 190u8, 151u8, 96u8, 159u8,
							19u8, 237u8, 241u8, 156u8, 5u8, 158u8, 191u8, 237u8, 9u8, 13u8, 86u8,
							213u8, 77u8, 58u8, 48u8, 139u8, 1u8, 85u8, 220u8, 233u8, 139u8, 164u8,
						],
					)
				}
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::Payload<types::KillStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_storage",
						types::KillStorage { keys },
						[
							73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
							234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
							156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
							35u8,
						],
					)
				}
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::KillPrefix> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_prefix",
						types::KillPrefix { prefix, subkeys },
						[
							184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
							175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
							67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
							85u8,
						],
					)
				}
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark_with_event",
						types::RemarkWithEvent { remark },
						[
							120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
							228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
							147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KilledAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Remarked {
				pub sender: ::subxt::utils::AccountId32,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							234u8, 12u8, 167u8, 96u8, 2u8, 244u8, 235u8, 62u8, 153u8, 200u8, 96u8,
							74u8, 135u8, 8u8, 35u8, 188u8, 146u8, 249u8, 246u8, 40u8, 224u8, 22u8,
							15u8, 99u8, 150u8, 222u8, 82u8, 85u8, 123u8, 123u8, 19u8, 110u8,
						],
					)
				}
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						Vec::new(),
						[
							234u8, 12u8, 167u8, 96u8, 2u8, 244u8, 235u8, 62u8, 153u8, 200u8, 96u8,
							74u8, 135u8, 8u8, 35u8, 188u8, 146u8, 249u8, 246u8, 40u8, 224u8, 22u8,
							15u8, 99u8, 150u8, 222u8, 82u8, 85u8, 123u8, 123u8, 19u8, 110u8,
						],
					)
				}
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
							153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
							120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
							159u8, 79u8,
						],
					)
				}
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockWeight",
						vec![],
						[
							52u8, 191u8, 212u8, 137u8, 26u8, 39u8, 239u8, 35u8, 182u8, 32u8, 39u8,
							103u8, 56u8, 184u8, 60u8, 159u8, 167u8, 232u8, 193u8, 116u8, 105u8,
							56u8, 98u8, 127u8, 124u8, 188u8, 214u8, 154u8, 160u8, 41u8, 20u8,
							162u8,
						],
					)
				}
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"AllExtrinsicsLen",
						vec![],
						[
							117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
							243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
							101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
							242u8, 65u8,
						],
					)
				}
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						Vec::new(),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				pub fn number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Number",
						vec![],
						[
							30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
							9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
							200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
						],
					)
				}
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ParentHash",
						vec![],
						[
							26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
							192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
							71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
						],
					)
				}
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_runtime::generic::digest::Digest,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Digest",
						vec![],
						[
							70u8, 156u8, 127u8, 89u8, 115u8, 250u8, 103u8, 62u8, 185u8, 153u8,
							26u8, 72u8, 39u8, 226u8, 181u8, 97u8, 137u8, 225u8, 45u8, 158u8, 212u8,
							254u8, 142u8, 136u8, 90u8, 22u8, 243u8, 125u8, 226u8, 49u8, 235u8,
							215u8,
						],
					)
				}
				pub fn events(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::parachain_runtime::RuntimeEvent,
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Events",
						vec![],
						[
							118u8, 221u8, 162u8, 93u8, 22u8, 181u8, 30u8, 212u8, 195u8, 75u8, 73u8,
							2u8, 159u8, 0u8, 211u8, 104u8, 213u8, 239u8, 117u8, 162u8, 253u8, 18u8,
							167u8, 21u8, 253u8, 173u8, 67u8, 231u8, 77u8, 205u8, 233u8, 17u8,
						],
					)
				}
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventCount",
						vec![],
						[
							175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
							151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
							254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
							133u8,
						],
					)
				}
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							154u8, 29u8, 31u8, 148u8, 254u8, 7u8, 124u8, 251u8, 241u8, 77u8, 24u8,
							37u8, 28u8, 75u8, 205u8, 17u8, 159u8, 79u8, 239u8, 62u8, 67u8, 60u8,
							252u8, 112u8, 215u8, 145u8, 103u8, 170u8, 110u8, 186u8, 221u8, 76u8,
						],
					)
				}
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						Vec::new(),
						[
							154u8, 29u8, 31u8, 148u8, 254u8, 7u8, 124u8, 251u8, 241u8, 77u8, 24u8,
							37u8, 28u8, 75u8, 205u8, 17u8, 159u8, 79u8, 239u8, 62u8, 67u8, 60u8,
							252u8, 112u8, 215u8, 145u8, 103u8, 170u8, 110u8, 186u8, 221u8, 76u8,
						],
					)
				}
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::LastRuntimeUpgradeInfo,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
							148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
							194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
						],
					)
				}
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
							130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
							107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
						],
					)
				}
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
							101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
							167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
							151u8,
						],
					)
				}
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::Phase,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExecutionPhase",
						vec![],
						[
							191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
							0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
							35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockWeights",
						[
							238u8, 20u8, 221u8, 11u8, 146u8, 236u8, 47u8, 103u8, 8u8, 239u8, 13u8,
							176u8, 202u8, 10u8, 151u8, 68u8, 110u8, 162u8, 99u8, 40u8, 211u8,
							136u8, 71u8, 82u8, 50u8, 80u8, 244u8, 211u8, 231u8, 198u8, 36u8, 152u8,
						],
					)
				}
				pub fn block_length(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockLength",
						[
							117u8, 144u8, 154u8, 125u8, 106u8, 34u8, 224u8, 228u8, 80u8, 76u8,
							126u8, 0u8, 177u8, 223u8, 116u8, 244u8, 167u8, 23u8, 253u8, 44u8,
							128u8, 116u8, 155u8, 245u8, 163u8, 20u8, 21u8, 222u8, 174u8, 237u8,
							162u8, 240u8,
						],
					)
				}
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight> {
					::subxt::constants::Address::new_static(
						"System",
						"DbWeight",
						[
							206u8, 53u8, 134u8, 247u8, 42u8, 38u8, 197u8, 59u8, 191u8, 83u8, 160u8,
							9u8, 207u8, 133u8, 108u8, 152u8, 150u8, 103u8, 109u8, 228u8, 218u8,
							24u8, 27u8, 210u8, 106u8, 252u8, 74u8, 93u8, 27u8, 63u8, 109u8, 252u8,
						],
					)
				}
				pub fn version(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion> {
					::subxt::constants::Address::new_static(
						"System",
						"Version",
						[
							134u8, 0u8, 23u8, 0u8, 199u8, 213u8, 89u8, 240u8, 194u8, 186u8, 239u8,
							157u8, 168u8, 211u8, 223u8, 156u8, 138u8, 140u8, 194u8, 23u8, 167u8,
							158u8, 195u8, 233u8, 25u8, 165u8, 27u8, 237u8, 198u8, 206u8, 233u8,
							28u8,
						],
					)
				}
				pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::{root_mod, runtime_types};
		pub type Call = runtime_types::pallet_timestamp::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Set {
					#[codec(compact)]
					pub now: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for Set {
					const PALLET: &'static str = "Timestamp";
					const CALL: &'static str = "set";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
					::subxt::tx::Payload::new_static(
						"Timestamp",
						"set",
						types::Set { now },
						[
							37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
							199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
							200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn now(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"Now",
						vec![],
						[
							44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
							92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
							141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
						],
					)
				}
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
							205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
							248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
							214u8, 140u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_system {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_parachain_system::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_parachain_system::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetValidationData {
					pub data:
						runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetValidationData {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "set_validation_data";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoSendUpwardMessage {
					pub message: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoSendUpwardMessage {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "sudo_send_upward_message";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AuthorizeUpgrade {
					pub code_hash: ::subxt::utils::H256,
					pub check_version: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgrade {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "authorize_upgrade";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct EnactAuthorizedUpgrade {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for EnactAuthorizedUpgrade {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "enact_authorized_upgrade";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> ::subxt::tx::Payload<types::SetValidationData> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"set_validation_data",
						types::SetValidationData { data },
						[
							54u8, 35u8, 155u8, 170u8, 67u8, 182u8, 121u8, 231u8, 153u8, 219u8,
							30u8, 2u8, 151u8, 82u8, 109u8, 79u8, 13u8, 181u8, 80u8, 148u8, 110u8,
							179u8, 38u8, 204u8, 137u8, 144u8, 145u8, 54u8, 20u8, 26u8, 31u8, 26u8,
						],
					)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SudoSendUpwardMessage> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"sudo_send_upward_message",
						types::SudoSendUpwardMessage { message },
						[
							1u8, 231u8, 11u8, 78u8, 127u8, 117u8, 248u8, 67u8, 230u8, 199u8, 126u8,
							47u8, 20u8, 62u8, 252u8, 138u8, 199u8, 48u8, 41u8, 21u8, 28u8, 157u8,
							218u8, 143u8, 4u8, 253u8, 62u8, 192u8, 94u8, 252u8, 92u8, 180u8,
						],
					)
				}
				pub fn authorize_upgrade(
					&self,
					code_hash: ::subxt::utils::H256,
					check_version: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::AuthorizeUpgrade> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"authorize_upgrade",
						types::AuthorizeUpgrade { code_hash, check_version },
						[
							213u8, 114u8, 107u8, 169u8, 223u8, 147u8, 205u8, 204u8, 3u8, 81u8,
							228u8, 0u8, 82u8, 57u8, 43u8, 95u8, 12u8, 59u8, 241u8, 176u8, 143u8,
							131u8, 253u8, 166u8, 98u8, 187u8, 94u8, 235u8, 177u8, 110u8, 162u8,
							218u8,
						],
					)
				}
				pub fn enact_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::EnactAuthorizedUpgrade> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"enact_authorized_upgrade",
						types::EnactAuthorizedUpgrade { code },
						[
							232u8, 135u8, 114u8, 87u8, 196u8, 146u8, 244u8, 19u8, 106u8, 73u8,
							88u8, 193u8, 48u8, 14u8, 72u8, 133u8, 247u8, 147u8, 50u8, 95u8, 252u8,
							213u8, 192u8, 47u8, 244u8, 102u8, 195u8, 120u8, 179u8, 87u8, 94u8, 8u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ValidationFunctionStored;
			impl ::subxt::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpgradeAuthorized {
				pub code_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DownwardMessagesReceived {
				pub count: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DownwardMessagesProcessed {
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub dmq_head: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpwardMessageSent {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for UpwardMessageSent {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpwardMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn pending_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingValidationCode",
						vec![],
						[
							78u8, 159u8, 219u8, 211u8, 177u8, 80u8, 102u8, 93u8, 83u8, 146u8, 90u8,
							233u8, 232u8, 11u8, 104u8, 172u8, 93u8, 68u8, 44u8, 228u8, 99u8, 197u8,
							254u8, 28u8, 181u8, 215u8, 247u8, 238u8, 49u8, 49u8, 195u8, 249u8,
						],
					)
				}
				pub fn new_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"NewValidationCode",
						vec![],
						[
							185u8, 123u8, 152u8, 122u8, 230u8, 136u8, 79u8, 73u8, 206u8, 19u8,
							59u8, 57u8, 75u8, 250u8, 83u8, 185u8, 29u8, 76u8, 89u8, 137u8, 77u8,
							163u8, 25u8, 125u8, 182u8, 67u8, 2u8, 180u8, 48u8, 237u8, 49u8, 171u8,
						],
					)
				}
				pub fn validation_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v4::PersistedValidationData<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ValidationData",
						vec![],
						[
							230u8, 88u8, 233u8, 102u8, 235u8, 97u8, 16u8, 116u8, 155u8, 118u8,
							82u8, 147u8, 113u8, 70u8, 191u8, 139u8, 146u8, 95u8, 184u8, 27u8,
							197u8, 131u8, 170u8, 97u8, 158u8, 73u8, 2u8, 73u8, 70u8, 109u8, 216u8,
							207u8,
						],
					)
				}
				pub fn did_set_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"DidSetValidationCode",
						vec![],
						[
							233u8, 228u8, 48u8, 111u8, 200u8, 35u8, 30u8, 139u8, 251u8, 77u8,
							196u8, 252u8, 35u8, 222u8, 129u8, 235u8, 7u8, 19u8, 156u8, 82u8, 126u8,
							173u8, 29u8, 62u8, 20u8, 67u8, 166u8, 116u8, 108u8, 182u8, 57u8, 246u8,
						],
					)
				}
				pub fn last_relay_chain_block_number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastRelayChainBlockNumber",
						vec![],
						[
							17u8, 65u8, 131u8, 169u8, 195u8, 243u8, 195u8, 93u8, 220u8, 174u8,
							75u8, 216u8, 214u8, 227u8, 96u8, 40u8, 8u8, 153u8, 116u8, 160u8, 79u8,
							255u8, 35u8, 232u8, 242u8, 42u8, 100u8, 150u8, 208u8, 210u8, 142u8,
							186u8,
						],
					)
				}
				pub fn upgrade_restriction_signal(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<
						runtime_types::polkadot_primitives::v4::UpgradeRestriction,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpgradeRestrictionSignal",
						vec![],
						[
							235u8, 240u8, 37u8, 44u8, 181u8, 52u8, 7u8, 216u8, 20u8, 139u8, 69u8,
							124u8, 21u8, 173u8, 237u8, 64u8, 105u8, 88u8, 49u8, 69u8, 123u8, 55u8,
							181u8, 167u8, 112u8, 183u8, 190u8, 231u8, 231u8, 127u8, 77u8, 148u8,
						],
					)
				}
				pub fn relay_state_proof(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_trie::storage_proof::StorageProof,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"RelayStateProof",
						vec![],
						[
							46u8, 115u8, 163u8, 190u8, 246u8, 47u8, 200u8, 159u8, 206u8, 204u8,
							94u8, 250u8, 127u8, 112u8, 109u8, 111u8, 210u8, 195u8, 244u8, 41u8,
							36u8, 187u8, 71u8, 150u8, 149u8, 253u8, 143u8, 33u8, 83u8, 189u8,
							182u8, 238u8,
						],
					)
				}				pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"RelevantMessagingState",
						vec![],
						[
							255u8, 164u8, 6u8, 175u8, 141u8, 49u8, 164u8, 244u8, 249u8, 53u8,
							189u8, 75u8, 185u8, 156u8, 18u8, 159u8, 245u8, 253u8, 188u8, 187u8,
							178u8, 245u8, 113u8, 214u8, 143u8, 57u8, 96u8, 28u8, 243u8, 247u8,
							15u8, 153u8,
						],
					)
				}
				pub fn host_configuration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v4::AbridgedHostConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HostConfiguration",
						vec![],
						[
							148u8, 148u8, 250u8, 200u8, 179u8, 26u8, 124u8, 183u8, 69u8, 186u8,
							56u8, 177u8, 209u8, 27u8, 102u8, 88u8, 44u8, 106u8, 0u8, 137u8, 137u8,
							88u8, 220u8, 222u8, 94u8, 129u8, 250u8, 162u8, 61u8, 216u8, 155u8,
							253u8,
						],
					)
				}
				pub fn last_dmq_mqc_head(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastDmqMqcHead",
						vec![],
						[
							1u8, 70u8, 140u8, 40u8, 51u8, 127u8, 75u8, 80u8, 5u8, 49u8, 196u8,
							31u8, 30u8, 61u8, 54u8, 252u8, 0u8, 0u8, 100u8, 115u8, 177u8, 250u8,
							138u8, 48u8, 107u8, 41u8, 93u8, 87u8, 195u8, 107u8, 206u8, 227u8,
						],
					)
				}
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::KeyedVec<
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastHrmpMqcHeads",
						vec![],
						[
							131u8, 170u8, 142u8, 30u8, 101u8, 113u8, 131u8, 81u8, 38u8, 168u8,
							98u8, 3u8, 9u8, 109u8, 96u8, 179u8, 115u8, 177u8, 128u8, 11u8, 238u8,
							54u8, 81u8, 60u8, 97u8, 112u8, 224u8, 175u8, 86u8, 133u8, 182u8, 76u8,
						],
					)
				}
				pub fn processed_downward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ProcessedDownwardMessages",
						vec![],
						[
							151u8, 234u8, 196u8, 87u8, 130u8, 79u8, 4u8, 102u8, 47u8, 10u8, 33u8,
							132u8, 149u8, 118u8, 61u8, 141u8, 5u8, 1u8, 30u8, 120u8, 220u8, 156u8,
							16u8, 11u8, 14u8, 52u8, 126u8, 151u8, 244u8, 149u8, 197u8, 51u8,
						],
					)
				}
				pub fn hrmp_watermark(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HrmpWatermark",
						vec![],
						[
							77u8, 62u8, 59u8, 220u8, 7u8, 125u8, 98u8, 249u8, 108u8, 212u8, 223u8,
							99u8, 152u8, 13u8, 29u8, 80u8, 166u8, 65u8, 232u8, 113u8, 145u8, 128u8,
							123u8, 35u8, 238u8, 31u8, 113u8, 156u8, 220u8, 104u8, 217u8, 165u8,
						],
					)
				}
				pub fn hrmp_outbound_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
							runtime_types::polkadot_parachain::primitives::Id,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HrmpOutboundMessages",
						vec![],
						[
							42u8, 9u8, 96u8, 217u8, 25u8, 101u8, 129u8, 147u8, 150u8, 20u8, 164u8,
							186u8, 217u8, 178u8, 15u8, 201u8, 233u8, 104u8, 92u8, 120u8, 29u8,
							245u8, 196u8, 13u8, 141u8, 210u8, 102u8, 62u8, 216u8, 80u8, 246u8,
							145u8,
						],
					)
				}
				pub fn upward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpwardMessages",
						vec![],
						[
							179u8, 127u8, 8u8, 94u8, 194u8, 246u8, 53u8, 79u8, 80u8, 22u8, 18u8,
							75u8, 116u8, 163u8, 90u8, 161u8, 30u8, 140u8, 57u8, 126u8, 60u8, 91u8,
							23u8, 30u8, 120u8, 245u8, 125u8, 96u8, 152u8, 25u8, 248u8, 85u8,
						],
					)
				}
				pub fn pending_upward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingUpwardMessages",
						vec![],
						[
							239u8, 45u8, 18u8, 173u8, 148u8, 150u8, 55u8, 176u8, 173u8, 156u8,
							246u8, 226u8, 198u8, 214u8, 104u8, 187u8, 186u8, 13u8, 83u8, 194u8,
							153u8, 29u8, 228u8, 109u8, 26u8, 18u8, 212u8, 151u8, 246u8, 24u8,
							133u8, 216u8,
						],
					)
				}
				pub fn announced_hrmp_messages_per_candidate(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"AnnouncedHrmpMessagesPerCandidate",
						vec![],
						[
							93u8, 11u8, 229u8, 172u8, 73u8, 87u8, 13u8, 149u8, 15u8, 94u8, 163u8,
							107u8, 156u8, 22u8, 131u8, 177u8, 96u8, 247u8, 213u8, 224u8, 41u8,
							126u8, 157u8, 33u8, 154u8, 194u8, 95u8, 234u8, 65u8, 19u8, 58u8, 161u8,
						],
					)
				}
				pub fn reserved_xcmp_weight_override(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ReservedXcmpWeightOverride",
						vec![],
						[
							62u8, 170u8, 113u8, 106u8, 140u8, 95u8, 230u8, 246u8, 5u8, 190u8,
							191u8, 74u8, 192u8, 216u8, 148u8, 168u8, 108u8, 200u8, 245u8, 73u8,
							21u8, 12u8, 167u8, 23u8, 37u8, 63u8, 135u8, 0u8, 234u8, 130u8, 239u8,
							234u8,
						],
					)
				}
				pub fn reserved_dmp_weight_override(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ReservedDmpWeightOverride",
						vec![],
						[
							197u8, 213u8, 100u8, 102u8, 181u8, 184u8, 204u8, 122u8, 40u8, 6u8,
							29u8, 192u8, 180u8, 245u8, 28u8, 201u8, 3u8, 230u8, 30u8, 51u8, 205u8,
							102u8, 169u8, 241u8, 133u8, 169u8, 4u8, 158u8, 223u8, 212u8, 97u8,
							108u8,
						],
					)
				}
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_parachain_system::CodeUpgradeAuthorization,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"AuthorizedUpgrade",
						vec![],
						[
							165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
							169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
							214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
						],
					)
				}
				pub fn custom_validation_head_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"CustomValidationHeadData",
						vec![],
						[
							52u8, 186u8, 187u8, 57u8, 245u8, 171u8, 202u8, 23u8, 92u8, 80u8, 118u8,
							66u8, 251u8, 156u8, 175u8, 254u8, 141u8, 185u8, 115u8, 209u8, 170u8,
							165u8, 1u8, 242u8, 120u8, 234u8, 162u8, 24u8, 135u8, 105u8, 8u8, 177u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_info {
		use super::{root_mod, runtime_types};
		pub type Call = runtime_types::parachain_info::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn parachain_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::Id,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainInfo",
						"ParachainId",
						vec![],
						[
							160u8, 130u8, 74u8, 181u8, 231u8, 180u8, 246u8, 152u8, 204u8, 44u8,
							245u8, 91u8, 113u8, 246u8, 218u8, 50u8, 254u8, 248u8, 35u8, 219u8,
							83u8, 144u8, 228u8, 245u8, 122u8, 53u8, 194u8, 172u8, 222u8, 118u8,
							202u8, 91u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_balances::pallet::Error;
		pub type Call = runtime_types::pallet_balances::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAllowDeath {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_allow_death";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetBalanceDeprecated {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub new_free: ::core::primitive::u128,
					#[codec(compact)]
					pub old_reserved: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "set_balance_deprecated";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceTransfer {
					pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferKeepAlive {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAll {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_all";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceUnreserve {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_unreserve";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpgradeAccounts {
					pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "upgrade_accounts";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Transfer {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSetBalance {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub new_free: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_set_balance";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer_allow_death(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_allow_death",
						types::TransferAllowDeath { dest, value },
						[
							113u8, 146u8, 7u8, 108u8, 132u8, 222u8, 204u8, 107u8, 142u8, 47u8,
							46u8, 72u8, 140u8, 22u8, 128u8, 135u8, 132u8, 10u8, 116u8, 41u8, 253u8,
							65u8, 140u8, 1u8, 50u8, 153u8, 47u8, 243u8, 210u8, 62u8, 23u8, 181u8,
						],
					)
				}
				pub fn set_balance_deprecated(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					new_free: ::core::primitive::u128,
					old_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"set_balance_deprecated",
						types::SetBalanceDeprecated { who, new_free, old_reserved },
						[
							43u8, 86u8, 149u8, 26u8, 133u8, 41u8, 43u8, 16u8, 98u8, 65u8, 244u8,
							123u8, 233u8, 146u8, 76u8, 116u8, 48u8, 164u8, 23u8, 211u8, 42u8,
							111u8, 245u8, 16u8, 54u8, 92u8, 163u8, 66u8, 193u8, 58u8, 58u8, 185u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_transfer",
						types::ForceTransfer { source, dest, value },
						[
							105u8, 201u8, 253u8, 250u8, 151u8, 193u8, 29u8, 188u8, 132u8, 138u8,
							84u8, 243u8, 170u8, 22u8, 169u8, 161u8, 202u8, 233u8, 79u8, 122u8,
							77u8, 198u8, 84u8, 149u8, 151u8, 238u8, 174u8, 179u8, 201u8, 150u8,
							184u8, 20u8,
						],
					)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							31u8, 197u8, 106u8, 219u8, 164u8, 234u8, 37u8, 214u8, 112u8, 211u8,
							149u8, 238u8, 162u8, 234u8, 226u8, 11u8, 182u8, 178u8, 182u8, 19u8,
							43u8, 172u8, 122u8, 175u8, 16u8, 253u8, 193u8, 116u8, 199u8, 158u8,
							255u8, 188u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							167u8, 120u8, 58u8, 200u8, 65u8, 248u8, 240u8, 141u8, 208u8, 240u8,
							89u8, 13u8, 62u8, 169u8, 228u8, 29u8, 219u8, 56u8, 137u8, 224u8, 16u8,
							111u8, 13u8, 65u8, 65u8, 84u8, 123u8, 173u8, 12u8, 82u8, 101u8, 226u8,
						],
					)
				}
				pub fn force_unreserve(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							244u8, 207u8, 86u8, 147u8, 199u8, 152u8, 130u8, 38u8, 248u8, 32u8,
							97u8, 0u8, 243u8, 58u8, 74u8, 238u8, 68u8, 144u8, 213u8, 168u8, 21u8,
							151u8, 62u8, 78u8, 8u8, 159u8, 89u8, 1u8, 255u8, 23u8, 83u8, 18u8,
						],
					)
				}
				pub fn upgrade_accounts(
					&self,
					who: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"upgrade_accounts",
						types::UpgradeAccounts { who },
						[
							66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
							233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
							214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
						],
					)
				}
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer",
						types::Transfer { dest, value },
						[
							155u8, 139u8, 211u8, 134u8, 163u8, 226u8, 249u8, 125u8, 15u8, 236u8,
							254u8, 100u8, 49u8, 104u8, 68u8, 71u8, 121u8, 120u8, 66u8, 159u8, 49u8,
							1u8, 125u8, 223u8, 43u8, 174u8, 19u8, 186u8, 110u8, 190u8, 87u8, 6u8,
						],
					)
				}
				pub fn force_set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					new_free: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceSetBalance> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_set_balance",
						types::ForceSetBalance { who, new_free },
						[
							22u8, 6u8, 147u8, 252u8, 116u8, 39u8, 228u8, 198u8, 229u8, 92u8, 141u8,
							236u8, 192u8, 108u8, 45u8, 242u8, 100u8, 148u8, 47u8, 5u8, 249u8, 49u8,
							81u8, 156u8, 81u8, 79u8, 216u8, 6u8, 15u8, 192u8, 97u8, 65u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Endowed {
				pub account: ::subxt::utils::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DustLost {
				pub account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Transfer {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BalanceSet {
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Reserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Unreserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReserveRepatriated {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Deposit {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Withdraw {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Slashed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Minted {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Minted {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Minted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Burned {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Suspended {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Suspended {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Suspended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Restored {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Restored {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Restored";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Upgraded {
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Upgraded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Upgraded";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Issued {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Rescinded {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rescinded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Rescinded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Locked {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Unlocked {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Frozen {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Thawed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Thawed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
							171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
							255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
							185u8,
						],
					)
				}
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"InactiveIssuance",
						vec![],
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
							249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
							30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							47u8, 253u8, 83u8, 165u8, 18u8, 176u8, 62u8, 239u8, 78u8, 85u8, 231u8,
							235u8, 157u8, 145u8, 251u8, 35u8, 225u8, 171u8, 82u8, 167u8, 68u8,
							206u8, 28u8, 169u8, 8u8, 93u8, 169u8, 101u8, 180u8, 206u8, 231u8,
							143u8,
						],
					)
				}
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						Vec::new(),
						[
							47u8, 253u8, 83u8, 165u8, 18u8, 176u8, 62u8, 239u8, 78u8, 85u8, 231u8,
							235u8, 157u8, 145u8, 251u8, 35u8, 225u8, 171u8, 82u8, 167u8, 68u8,
							206u8, 28u8, 169u8, 8u8, 93u8, 169u8, 101u8, 180u8, 206u8, 231u8,
							143u8,
						],
					)
				}
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							44u8, 44u8, 48u8, 20u8, 121u8, 168u8, 200u8, 87u8, 205u8, 172u8, 111u8,
							208u8, 62u8, 243u8, 225u8, 223u8, 181u8, 36u8, 197u8, 9u8, 52u8, 182u8,
							113u8, 55u8, 126u8, 164u8, 82u8, 209u8, 151u8, 126u8, 186u8, 85u8,
						],
					)
				}
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						Vec::new(),
						[
							44u8, 44u8, 48u8, 20u8, 121u8, 168u8, 200u8, 87u8, 205u8, 172u8, 111u8,
							208u8, 62u8, 243u8, 225u8, 223u8, 181u8, 36u8, 197u8, 9u8, 52u8, 182u8,
							113u8, 55u8, 126u8, 164u8, 82u8, 209u8, 151u8, 126u8, 186u8, 85u8,
						],
					)
				}
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							192u8, 99u8, 91u8, 129u8, 195u8, 73u8, 153u8, 126u8, 82u8, 52u8, 56u8,
							85u8, 105u8, 178u8, 113u8, 101u8, 229u8, 37u8, 242u8, 174u8, 166u8,
							244u8, 68u8, 173u8, 14u8, 225u8, 172u8, 70u8, 181u8, 211u8, 165u8,
							134u8,
						],
					)
				}
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							192u8, 99u8, 91u8, 129u8, 195u8, 73u8, 153u8, 126u8, 82u8, 52u8, 56u8,
							85u8, 105u8, 178u8, 113u8, 101u8, 229u8, 37u8, 242u8, 174u8, 166u8,
							244u8, 68u8, 173u8, 14u8, 225u8, 172u8, 70u8, 181u8, 211u8, 165u8,
							134u8,
						],
					)
				}
				pub fn holds(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Holds",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
							95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
							231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
							202u8,
						],
					)
				}
				pub fn holds_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Holds",
						Vec::new(),
						[
							53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
							95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
							231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
							202u8,
						],
					)
				}
				pub fn freezes(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Freezes",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
							112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
							163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
						],
					)
				}
				pub fn freezes_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Freezes",
						Vec::new(),
						[
							69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
							112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
							163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxHolds",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxFreezes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransactionFeePaid {
				pub who: ::subxt::utils::AccountId32,
				pub actual_fee: ::core::primitive::u128,
				pub tip: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
							147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
							159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
							197u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_transaction_payment::Releases,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
							178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
							251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
							144u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn author(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Authorship",
						"Author",
						vec![],
						[
							247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
							220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
							61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
							18u8,
						],
					)
				}
			}
		}
	}
	pub mod collator_selection {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_collator_selection::pallet::Error;
		pub type Call = runtime_types::pallet_collator_selection::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetInvulnerables {
					pub new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetInvulnerables {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_invulnerables";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetDesiredCandidates {
					pub max: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetDesiredCandidates {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_desired_candidates";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCandidacyBond {
					pub bond: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCandidacyBond {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_candidacy_bond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RegisterAsCandidate;
				impl ::subxt::blocks::StaticExtrinsic for RegisterAsCandidate {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "register_as_candidate";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LeaveIntent;
				impl ::subxt::blocks::StaticExtrinsic for LeaveIntent {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "leave_intent";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::SetInvulnerables> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_invulnerables",
						types::SetInvulnerables { new },
						[
							113u8, 217u8, 14u8, 48u8, 6u8, 198u8, 8u8, 170u8, 8u8, 237u8, 230u8,
							184u8, 17u8, 181u8, 15u8, 126u8, 117u8, 3u8, 208u8, 215u8, 40u8, 16u8,
							150u8, 162u8, 37u8, 196u8, 235u8, 36u8, 247u8, 24u8, 187u8, 17u8,
						],
					)
				}
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetDesiredCandidates> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_desired_candidates",
						types::SetDesiredCandidates { max },
						[
							174u8, 44u8, 232u8, 155u8, 228u8, 219u8, 239u8, 75u8, 86u8, 150u8,
							135u8, 214u8, 58u8, 9u8, 25u8, 133u8, 245u8, 101u8, 85u8, 246u8, 15u8,
							248u8, 165u8, 87u8, 88u8, 28u8, 10u8, 196u8, 86u8, 89u8, 28u8, 165u8,
						],
					)
				}
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetCandidacyBond> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_candidacy_bond",
						types::SetCandidacyBond { bond },
						[
							250u8, 4u8, 185u8, 228u8, 101u8, 223u8, 49u8, 44u8, 172u8, 148u8,
							216u8, 242u8, 192u8, 88u8, 228u8, 59u8, 225u8, 222u8, 171u8, 40u8,
							23u8, 1u8, 46u8, 183u8, 189u8, 191u8, 156u8, 12u8, 218u8, 116u8, 76u8,
							59u8,
						],
					)
				}
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::tx::Payload<types::RegisterAsCandidate> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"register_as_candidate",
						types::RegisterAsCandidate {},
						[
							69u8, 222u8, 214u8, 106u8, 105u8, 168u8, 82u8, 239u8, 158u8, 117u8,
							224u8, 89u8, 228u8, 51u8, 221u8, 244u8, 88u8, 63u8, 72u8, 119u8, 224u8,
							111u8, 93u8, 39u8, 18u8, 66u8, 72u8, 105u8, 70u8, 66u8, 178u8, 173u8,
						],
					)
				}
				pub fn leave_intent(&self) -> ::subxt::tx::Payload<types::LeaveIntent> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"leave_intent",
						types::LeaveIntent {},
						[
							126u8, 57u8, 10u8, 67u8, 120u8, 229u8, 70u8, 23u8, 154u8, 215u8, 226u8,
							178u8, 203u8, 152u8, 195u8, 177u8, 157u8, 158u8, 40u8, 17u8, 93u8,
							225u8, 253u8, 217u8, 48u8, 165u8, 55u8, 79u8, 43u8, 123u8, 193u8,
							147u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewInvulnerables {
				pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewDesiredCandidates {
				pub desired_candidates: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewCandidacyBond {
				pub bond_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CandidateAdded {
				pub account_id: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CandidateRemoved {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for CandidateRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::utils::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"Invulnerables",
						vec![],
						[
							109u8, 180u8, 25u8, 41u8, 152u8, 158u8, 186u8, 214u8, 89u8, 222u8,
							103u8, 14u8, 91u8, 3u8, 65u8, 6u8, 255u8, 62u8, 47u8, 255u8, 132u8,
							164u8, 217u8, 200u8, 130u8, 29u8, 168u8, 23u8, 81u8, 217u8, 35u8,
							123u8,
						],
					)
				}
				pub fn candidates(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"Candidates",
						vec![],
						[
							95u8, 142u8, 119u8, 195u8, 123u8, 1u8, 212u8, 104u8, 23u8, 112u8,
							215u8, 11u8, 254u8, 30u8, 40u8, 19u8, 86u8, 187u8, 3u8, 179u8, 34u8,
							255u8, 215u8, 181u8, 162u8, 57u8, 23u8, 220u8, 223u8, 55u8, 180u8,
							88u8,
						],
					)
				}
				pub fn last_authored_block(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
							72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
							126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				pub fn last_authored_block_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						Vec::new(),
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
							72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
							126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				pub fn desired_candidates(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"DesiredCandidates",
						vec![],
						[
							69u8, 199u8, 130u8, 132u8, 10u8, 127u8, 204u8, 220u8, 59u8, 107u8,
							96u8, 180u8, 42u8, 235u8, 14u8, 126u8, 231u8, 242u8, 162u8, 126u8,
							63u8, 223u8, 15u8, 250u8, 22u8, 210u8, 54u8, 34u8, 235u8, 191u8, 250u8,
							21u8,
						],
					)
				}
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"CandidacyBond",
						vec![],
						[
							71u8, 134u8, 156u8, 102u8, 201u8, 83u8, 240u8, 251u8, 189u8, 213u8,
							211u8, 182u8, 126u8, 122u8, 41u8, 174u8, 105u8, 29u8, 216u8, 23u8,
							255u8, 55u8, 245u8, 187u8, 234u8, 234u8, 178u8, 155u8, 145u8, 49u8,
							196u8, 214u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_session::pallet::Error;
		pub type Call = runtime_types::pallet_session::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetKeys {
					pub keys: runtime_types::parachain_runtime::SessionKeys,
					pub proof: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "set_keys";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PurgeKeys;
				impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "purge_keys";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_keys(
					&self,
					keys: runtime_types::parachain_runtime::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"set_keys",
						types::SetKeys { keys, proof },
						[
							64u8, 163u8, 42u8, 125u8, 103u8, 172u8, 160u8, 250u8, 83u8, 158u8,
							242u8, 57u8, 97u8, 56u8, 198u8, 61u8, 143u8, 0u8, 23u8, 214u8, 20u8,
							184u8, 234u8, 42u8, 57u8, 114u8, 58u8, 36u8, 86u8, 79u8, 239u8, 90u8,
						],
					)
				}
				pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"purge_keys",
						types::PurgeKeys {},
						[
							215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
							151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
							67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
							209u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"Validators",
						vec![],
						[
							50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
							133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
							115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
							86u8,
						],
					)
				}
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"CurrentIndex",
						vec![],
						[
							167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
							135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
							134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
							221u8, 230u8,
						],
					)
				}
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedChanged",
						vec![],
						[
							184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
							198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
							36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
							153u8,
						],
					)
				}
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::parachain_runtime::SessionKeys,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedKeys",
						vec![],
						[
							175u8, 90u8, 122u8, 133u8, 2u8, 146u8, 102u8, 126u8, 214u8, 66u8,
							108u8, 86u8, 67u8, 51u8, 50u8, 22u8, 48u8, 87u8, 201u8, 204u8, 98u8,
							83u8, 42u8, 62u8, 106u8, 151u8, 233u8, 170u8, 118u8, 199u8, 246u8,
							93u8,
						],
					)
				}
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"DisabledValidators",
						vec![],
						[
							213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
							36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
							183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
						],
					)
				}
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::parachain_runtime::SessionKeys,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							85u8, 4u8, 20u8, 173u8, 6u8, 115u8, 27u8, 4u8, 148u8, 90u8, 90u8,
							226u8, 210u8, 21u8, 26u8, 46u8, 107u8, 107u8, 150u8, 211u8, 201u8,
							180u8, 202u8, 180u8, 128u8, 47u8, 142u8, 4u8, 244u8, 80u8, 168u8,
							248u8,
						],
					)
				}
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::parachain_runtime::SessionKeys,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							85u8, 4u8, 20u8, 173u8, 6u8, 115u8, 27u8, 4u8, 148u8, 90u8, 90u8,
							226u8, 210u8, 21u8, 26u8, 46u8, 107u8, 107u8, 150u8, 211u8, 201u8,
							180u8, 202u8, 180u8, 128u8, 47u8, 142u8, 4u8, 244u8, 80u8, 168u8,
							248u8,
						],
					)
				}
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							177u8, 90u8, 148u8, 24u8, 251u8, 26u8, 65u8, 235u8, 46u8, 25u8, 109u8,
							212u8, 208u8, 218u8, 58u8, 196u8, 29u8, 73u8, 145u8, 41u8, 30u8, 251u8,
							185u8, 26u8, 205u8, 50u8, 32u8, 200u8, 206u8, 178u8, 255u8, 146u8,
						],
					)
				}
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							177u8, 90u8, 148u8, 24u8, 251u8, 26u8, 65u8, 235u8, 46u8, 25u8, 109u8,
							212u8, 208u8, 218u8, 58u8, 196u8, 29u8, 73u8, 145u8, 41u8, 30u8, 251u8,
							185u8, 26u8, 205u8, 50u8, 32u8, 200u8, 206u8, 178u8, 255u8, 146u8,
						],
					)
				}
			}
		}
	}
	pub mod aura {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Aura",
						"Authorities",
						vec![],
						[
							232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
							129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
							94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
						],
					)
				}
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_slots::Slot,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Aura",
						"CurrentSlot",
						vec![],
						[
							112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
							236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
							201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
							43u8, 57u8,
						],
					)
				}
			}
		}
	}
	pub mod aura_ext {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AuraExt",
						"Authorities",
						vec![],
						[
							232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
							129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
							94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
						],
					)
				}
			}
		}
	}
	pub mod xcmp_queue {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_xcmp_queue::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_xcmp_queue::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ServiceOverweight {
					pub index: ::core::primitive::u64,
					pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ServiceOverweight {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "service_overweight";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SuspendXcmExecution;
				impl ::subxt::blocks::StaticExtrinsic for SuspendXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "suspend_xcm_execution";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ResumeXcmExecution;
				impl ::subxt::blocks::StaticExtrinsic for ResumeXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "resume_xcm_execution";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateSuspendThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateSuspendThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_suspend_threshold";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateDropThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateDropThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_drop_threshold";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateResumeThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateResumeThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_resume_threshold";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateThresholdWeight {
					pub new: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateThresholdWeight {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_threshold_weight";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateWeightRestrictDecay {
					pub new: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateWeightRestrictDecay {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_weight_restrict_decay";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateXcmpMaxIndividualWeight {
					pub new: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateXcmpMaxIndividualWeight {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_xcmp_max_individual_weight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ServiceOverweight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"service_overweight",
						types::ServiceOverweight { index, weight_limit },
						[
							191u8, 141u8, 229u8, 214u8, 12u8, 111u8, 251u8, 4u8, 8u8, 118u8, 217u8,
							193u8, 106u8, 182u8, 233u8, 130u8, 149u8, 201u8, 245u8, 3u8, 81u8,
							217u8, 33u8, 24u8, 198u8, 138u8, 206u8, 141u8, 105u8, 31u8, 81u8, 9u8,
						],
					)
				}
				pub fn suspend_xcm_execution(
					&self,
				) -> ::subxt::tx::Payload<types::SuspendXcmExecution> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"suspend_xcm_execution",
						types::SuspendXcmExecution {},
						[
							54u8, 120u8, 33u8, 251u8, 74u8, 56u8, 29u8, 76u8, 104u8, 218u8, 115u8,
							198u8, 148u8, 237u8, 9u8, 191u8, 241u8, 48u8, 33u8, 24u8, 60u8, 144u8,
							22u8, 78u8, 58u8, 50u8, 26u8, 188u8, 231u8, 42u8, 201u8, 76u8,
						],
					)
				}
				pub fn resume_xcm_execution(
					&self,
				) -> ::subxt::tx::Payload<types::ResumeXcmExecution> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"resume_xcm_execution",
						types::ResumeXcmExecution {},
						[
							173u8, 231u8, 78u8, 253u8, 108u8, 234u8, 199u8, 124u8, 184u8, 154u8,
							95u8, 194u8, 13u8, 77u8, 175u8, 7u8, 7u8, 112u8, 161u8, 72u8, 133u8,
							71u8, 63u8, 218u8, 97u8, 226u8, 133u8, 6u8, 93u8, 177u8, 247u8, 109u8,
						],
					)
				}
				pub fn update_suspend_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateSuspendThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_suspend_threshold",
						types::UpdateSuspendThreshold { new },
						[
							64u8, 91u8, 172u8, 51u8, 220u8, 174u8, 54u8, 47u8, 57u8, 89u8, 75u8,
							39u8, 126u8, 198u8, 143u8, 35u8, 70u8, 125u8, 167u8, 14u8, 17u8, 18u8,
							146u8, 222u8, 100u8, 92u8, 81u8, 239u8, 173u8, 43u8, 42u8, 174u8,
						],
					)
				}
				pub fn update_drop_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateDropThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_drop_threshold",
						types::UpdateDropThreshold { new },
						[
							123u8, 54u8, 12u8, 180u8, 165u8, 198u8, 141u8, 200u8, 149u8, 168u8,
							186u8, 237u8, 162u8, 91u8, 89u8, 242u8, 229u8, 16u8, 32u8, 254u8, 59u8,
							168u8, 31u8, 134u8, 217u8, 251u8, 0u8, 102u8, 113u8, 194u8, 175u8, 9u8,
						],
					)
				}
				pub fn update_resume_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateResumeThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_resume_threshold",
						types::UpdateResumeThreshold { new },
						[
							172u8, 136u8, 11u8, 106u8, 42u8, 157u8, 167u8, 183u8, 87u8, 62u8,
							182u8, 17u8, 184u8, 59u8, 215u8, 230u8, 18u8, 243u8, 212u8, 34u8, 54u8,
							188u8, 95u8, 119u8, 173u8, 20u8, 91u8, 206u8, 212u8, 57u8, 136u8, 77u8,
						],
					)
				}
				pub fn update_threshold_weight(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::UpdateThresholdWeight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_threshold_weight",
						types::UpdateThresholdWeight { new },
						[
							38u8, 151u8, 123u8, 195u8, 124u8, 92u8, 9u8, 0u8, 252u8, 249u8, 95u8,
							81u8, 104u8, 48u8, 246u8, 8u8, 192u8, 139u8, 155u8, 19u8, 51u8, 228u8,
							33u8, 168u8, 75u8, 205u8, 50u8, 195u8, 226u8, 13u8, 58u8, 6u8,
						],
					)
				}
				pub fn update_weight_restrict_decay(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::UpdateWeightRestrictDecay> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_weight_restrict_decay",
						types::UpdateWeightRestrictDecay { new },
						[
							152u8, 246u8, 197u8, 32u8, 91u8, 22u8, 8u8, 244u8, 126u8, 24u8, 152u8,
							14u8, 136u8, 191u8, 0u8, 133u8, 46u8, 41u8, 165u8, 29u8, 55u8, 120u8,
							41u8, 16u8, 140u8, 248u8, 78u8, 7u8, 48u8, 164u8, 55u8, 24u8,
						],
					)
				}
				pub fn update_xcmp_max_individual_weight(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::UpdateXcmpMaxIndividualWeight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_xcmp_max_individual_weight",
						types::UpdateXcmpMaxIndividualWeight { new },
						[
							132u8, 19u8, 30u8, 28u8, 188u8, 201u8, 203u8, 39u8, 38u8, 126u8, 9u8,
							39u8, 242u8, 73u8, 114u8, 48u8, 208u8, 236u8, 80u8, 110u8, 225u8, 32u8,
							164u8, 32u8, 66u8, 135u8, 155u8, 33u8, 86u8, 42u8, 85u8, 203u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Success {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Fail {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub error: runtime_types::xcm::v3::traits::Error,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BadVersion {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BadFormat {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct XcmpMessageSent {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OverweightEnqueued {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub sent_at: ::core::primitive::u32,
				pub index: ::core::primitive::u64,
				pub required: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OverweightServiced {
				pub index: ::core::primitive::u64,
				pub used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn inbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpStatus",
						vec![],
						[
							144u8, 90u8, 119u8, 123u8, 43u8, 209u8, 141u8, 230u8, 43u8, 53u8, 53u8,
							240u8, 243u8, 213u8, 125u8, 78u8, 143u8, 109u8, 167u8, 234u8, 63u8,
							7u8, 37u8, 86u8, 243u8, 153u8, 121u8, 120u8, 168u8, 112u8, 251u8,
							241u8,
						],
					)
				}
				pub fn inbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpMessages",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							237u8, 85u8, 41u8, 131u8, 194u8, 140u8, 140u8, 237u8, 154u8, 143u8,
							147u8, 109u8, 206u8, 54u8, 212u8, 169u8, 112u8, 43u8, 245u8, 249u8,
							9u8, 27u8, 252u8, 65u8, 16u8, 94u8, 147u8, 73u8, 55u8, 250u8, 88u8,
							231u8,
						],
					)
				}
				pub fn inbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpMessages",
						Vec::new(),
						[
							237u8, 85u8, 41u8, 131u8, 194u8, 140u8, 140u8, 237u8, 154u8, 143u8,
							147u8, 109u8, 206u8, 54u8, 212u8, 169u8, 112u8, 43u8, 245u8, 249u8,
							9u8, 27u8, 252u8, 65u8, 16u8, 94u8, 147u8, 73u8, 55u8, 250u8, 88u8,
							231u8,
						],
					)
				}
				pub fn outbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpStatus",
						vec![],
						[
							196u8, 87u8, 178u8, 148u8, 210u8, 8u8, 43u8, 99u8, 42u8, 216u8, 120u8,
							50u8, 198u8, 221u8, 184u8, 18u8, 102u8, 153u8, 43u8, 167u8, 121u8,
							231u8, 14u8, 156u8, 51u8, 53u8, 154u8, 214u8, 251u8, 195u8, 119u8,
							120u8,
						],
					)
				}
				pub fn outbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u16>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							156u8, 3u8, 202u8, 175u8, 175u8, 129u8, 38u8, 144u8, 35u8, 59u8, 228u8,
							159u8, 142u8, 25u8, 19u8, 73u8, 73u8, 6u8, 115u8, 19u8, 236u8, 235u8,
							144u8, 172u8, 31u8, 168u8, 24u8, 65u8, 115u8, 95u8, 77u8, 63u8,
						],
					)
				}
				pub fn outbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						Vec::new(),
						[
							156u8, 3u8, 202u8, 175u8, 175u8, 129u8, 38u8, 144u8, 35u8, 59u8, 228u8,
							159u8, 142u8, 25u8, 19u8, 73u8, 73u8, 6u8, 115u8, 19u8, 236u8, 235u8,
							144u8, 172u8, 31u8, 168u8, 24u8, 65u8, 115u8, 95u8, 77u8, 63u8,
						],
					)
				}
				pub fn signal_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"SignalMessages",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							182u8, 143u8, 233u8, 233u8, 111u8, 137u8, 174u8, 165u8, 166u8, 7u8,
							229u8, 183u8, 99u8, 108u8, 30u8, 162u8, 71u8, 55u8, 122u8, 124u8,
							249u8, 203u8, 142u8, 124u8, 158u8, 213u8, 182u8, 159u8, 206u8, 249u8,
							180u8, 24u8,
						],
					)
				}
				pub fn signal_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"SignalMessages",
						Vec::new(),
						[
							182u8, 143u8, 233u8, 233u8, 111u8, 137u8, 174u8, 165u8, 166u8, 7u8,
							229u8, 183u8, 99u8, 108u8, 30u8, 162u8, 71u8, 55u8, 122u8, 124u8,
							249u8, 203u8, 142u8, 124u8, 158u8, 213u8, 182u8, 159u8, 206u8, 249u8,
							180u8, 24u8,
						],
					)
				}
				pub fn queue_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"QueueConfig",
						vec![],
						[
							0u8, 129u8, 154u8, 51u8, 104u8, 59u8, 251u8, 184u8, 217u8, 78u8, 205u8,
							2u8, 34u8, 153u8, 67u8, 242u8, 113u8, 218u8, 130u8, 239u8, 196u8,
							236u8, 243u8, 107u8, 57u8, 197u8, 50u8, 43u8, 125u8, 104u8, 204u8,
							222u8,
						],
					)
				}
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"Overweight",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							80u8, 42u8, 2u8, 203u8, 202u8, 198u8, 174u8, 187u8, 130u8, 113u8, 42u8,
							50u8, 216u8, 24u8, 121u8, 65u8, 26u8, 164u8, 36u8, 189u8, 136u8, 71u8,
							237u8, 87u8, 179u8, 25u8, 221u8, 5u8, 19u8, 218u8, 97u8, 200u8,
						],
					)
				}
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"Overweight",
						Vec::new(),
						[
							80u8, 42u8, 2u8, 203u8, 202u8, 198u8, 174u8, 187u8, 130u8, 113u8, 42u8,
							50u8, 216u8, 24u8, 121u8, 65u8, 26u8, 164u8, 36u8, 189u8, 136u8, 71u8,
							237u8, 87u8, 179u8, 25u8, 221u8, 5u8, 19u8, 218u8, 97u8, 200u8,
						],
					)
				}
				pub fn counter_for_overweight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"CounterForOverweight",
						vec![],
						[
							44u8, 249u8, 133u8, 204u8, 169u8, 253u8, 23u8, 157u8, 132u8, 193u8,
							28u8, 178u8, 156u8, 176u8, 206u8, 46u8, 79u8, 254u8, 174u8, 236u8,
							143u8, 219u8, 59u8, 43u8, 36u8, 109u8, 244u8, 206u8, 48u8, 126u8,
							247u8, 0u8,
						],
					)
				}
				pub fn overweight_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OverweightCount",
						vec![],
						[
							28u8, 72u8, 218u8, 167u8, 253u8, 30u8, 10u8, 51u8, 49u8, 101u8, 86u8,
							26u8, 146u8, 2u8, 153u8, 232u8, 129u8, 38u8, 111u8, 105u8, 246u8, 84u8,
							192u8, 157u8, 193u8, 57u8, 222u8, 122u8, 38u8, 160u8, 56u8, 39u8,
						],
					)
				}
				pub fn queue_suspended(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"QueueSuspended",
						vec![],
						[
							165u8, 66u8, 105u8, 244u8, 113u8, 43u8, 177u8, 252u8, 212u8, 243u8,
							143u8, 184u8, 87u8, 51u8, 163u8, 104u8, 29u8, 84u8, 119u8, 74u8, 233u8,
							129u8, 203u8, 105u8, 2u8, 101u8, 19u8, 170u8, 69u8, 253u8, 80u8, 132u8,
						],
					)
				}
			}
		}
	}
	pub mod polkadot_xcm {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_xcm::pallet::Error;
		pub type Call = runtime_types::pallet_xcm::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Send {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Send {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "send";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TeleportAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for TeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "teleport_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveTransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "reserve_transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Execute {
					pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "execute";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceXcmVersion {
					pub location:
						::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
					pub xcm_version: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_xcm_version";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceDefaultXcmVersion {
					pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceDefaultXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_default_xcm_version";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSubscribeVersionNotify {
					pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_subscribe_version_notify";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceUnsubscribeVersionNotify {
					pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnsubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_unsubscribe_version_notify";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LimitedReserveTransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for LimitedReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_reserve_transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LimitedTeleportAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for LimitedTeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_teleport_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSuspension {
					pub suspended: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSuspension {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_suspension";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::Payload<types::Send> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"send",
						types::Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							179u8, 0u8, 46u8, 230u8, 210u8, 183u8, 91u8, 129u8, 120u8, 45u8, 156u8,
							37u8, 100u8, 175u8, 147u8, 10u8, 24u8, 70u8, 80u8, 8u8, 253u8, 107u8,
							168u8, 10u8, 46u8, 192u8, 226u8, 208u8, 102u8, 217u8, 25u8, 154u8,
						],
					)
				}
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::TeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"teleport_assets",
						types::TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							55u8, 134u8, 79u8, 159u8, 98u8, 83u8, 169u8, 45u8, 116u8, 191u8, 166u8,
							51u8, 27u8, 109u8, 114u8, 146u8, 82u8, 240u8, 159u8, 63u8, 35u8, 227u8,
							206u8, 69u8, 133u8, 194u8, 181u8, 110u8, 77u8, 30u8, 241u8, 210u8,
						],
					)
				}
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"reserve_transfer_assets",
						types::ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							12u8, 232u8, 11u8, 188u8, 202u8, 160u8, 42u8, 194u8, 91u8, 84u8, 167u8,
							83u8, 193u8, 198u8, 121u8, 60u8, 83u8, 34u8, 10u8, 1u8, 19u8, 9u8,
							115u8, 71u8, 242u8, 65u8, 139u8, 58u8, 35u8, 103u8, 241u8, 196u8,
						],
					)
				}
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm2,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"execute",
						types::Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							162u8, 211u8, 166u8, 126u8, 220u8, 232u8, 242u8, 251u8, 44u8, 206u8,
							7u8, 198u8, 89u8, 27u8, 26u8, 87u8, 151u8, 79u8, 38u8, 164u8, 141u8,
							81u8, 218u8, 162u8, 52u8, 238u8, 46u8, 207u8, 235u8, 124u8, 165u8, 8u8,
						],
					)
				}
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v3::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ForceXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_xcm_version",
						types::ForceXcmVersion {
							location: ::std::boxed::Box::new(location),
							xcm_version,
						},
						[
							31u8, 152u8, 219u8, 67u8, 212u8, 13u8, 162u8, 129u8, 151u8, 183u8,
							18u8, 2u8, 5u8, 229u8, 94u8, 37u8, 135u8, 69u8, 111u8, 100u8, 177u8,
							96u8, 57u8, 124u8, 53u8, 172u8, 245u8, 29u8, 156u8, 135u8, 42u8, 125u8,
						],
					)
				}
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<types::ForceDefaultXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_default_xcm_version",
						types::ForceDefaultXcmVersion { maybe_xcm_version },
						[
							43u8, 114u8, 102u8, 104u8, 209u8, 234u8, 108u8, 173u8, 109u8, 188u8,
							94u8, 214u8, 136u8, 43u8, 153u8, 75u8, 161u8, 192u8, 76u8, 12u8, 221u8,
							237u8, 158u8, 247u8, 41u8, 193u8, 35u8, 174u8, 183u8, 207u8, 79u8,
							213u8,
						],
					)
				}
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::Payload<types::ForceSubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_subscribe_version_notify",
						types::ForceSubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							47u8, 23u8, 248u8, 170u8, 109u8, 130u8, 223u8, 240u8, 219u8, 162u8,
							206u8, 216u8, 76u8, 17u8, 57u8, 145u8, 124u8, 40u8, 49u8, 226u8, 40u8,
							102u8, 163u8, 66u8, 42u8, 78u8, 15u8, 4u8, 169u8, 40u8, 247u8, 40u8,
						],
					)
				}
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::Payload<types::ForceUnsubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_unsubscribe_version_notify",
						types::ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							240u8, 140u8, 37u8, 245u8, 15u8, 214u8, 120u8, 250u8, 98u8, 219u8,
							199u8, 22u8, 244u8, 191u8, 74u8, 59u8, 86u8, 221u8, 20u8, 75u8, 20u8,
							97u8, 139u8, 71u8, 77u8, 157u8, 114u8, 198u8, 98u8, 72u8, 40u8, 227u8,
						],
					)
				}
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::LimitedReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"limited_reserve_transfer_assets",
						types::LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							212u8, 29u8, 239u8, 236u8, 39u8, 49u8, 85u8, 129u8, 164u8, 143u8,
							214u8, 84u8, 181u8, 63u8, 47u8, 244u8, 77u8, 149u8, 76u8, 247u8, 207u8,
							222u8, 216u8, 16u8, 191u8, 249u8, 165u8, 17u8, 56u8, 49u8, 165u8,
							137u8,
						],
					)
				}
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::LimitedTeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"limited_teleport_assets",
						types::LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							185u8, 108u8, 229u8, 144u8, 189u8, 81u8, 240u8, 175u8, 141u8, 24u8,
							153u8, 172u8, 220u8, 233u8, 25u8, 173u8, 158u8, 15u8, 150u8, 74u8,
							133u8, 5u8, 8u8, 227u8, 194u8, 62u8, 93u8, 166u8, 34u8, 67u8, 64u8,
							10u8,
						],
					)
				}
				pub fn force_suspension(
					&self,
					suspended: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceSuspension> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_suspension",
						types::ForceSuspension { suspended },
						[
							78u8, 125u8, 93u8, 55u8, 129u8, 44u8, 36u8, 227u8, 75u8, 46u8, 68u8,
							202u8, 81u8, 127u8, 111u8, 92u8, 149u8, 38u8, 225u8, 185u8, 183u8,
							154u8, 89u8, 159u8, 79u8, 10u8, 229u8, 1u8, 226u8, 243u8, 65u8, 238u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Attempted(pub runtime_types::xcm::v3::traits::Outcome);
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Sent(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::Xcm,
			);
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::Response,
			);
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for Notified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyOverweight(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
				pub runtime_types::sp_weights::weight_v2::Weight,
				pub runtime_types::sp_weights::weight_v2::Weight,
			);
			impl ::subxt::events::StaticEvent for NotifyOverweight {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyDecodeFailed(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidResponder(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetsTrapped(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u32,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::traits::Error,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidQuerierVersion(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidQuerierVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerierVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidQuerier(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidQuerier {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerier";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionNotifyStarted(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyStarted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionNotifyRequested(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyRequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyRequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionNotifyUnrequested(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyUnrequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyUnrequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FeesPaid(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for FeesPaid {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "FeesPaid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetsClaimed(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsClaimed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsClaimed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn query_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"QueryCounter",
						vec![],
						[
							216u8, 73u8, 160u8, 232u8, 60u8, 245u8, 218u8, 219u8, 152u8, 68u8,
							146u8, 219u8, 255u8, 7u8, 86u8, 112u8, 83u8, 49u8, 94u8, 173u8, 64u8,
							203u8, 147u8, 226u8, 236u8, 39u8, 129u8, 106u8, 209u8, 113u8, 150u8,
							50u8,
						],
					)
				}
				pub fn queries(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"Queries",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							177u8, 138u8, 131u8, 91u8, 46u8, 151u8, 115u8, 184u8, 120u8, 158u8,
							241u8, 43u8, 56u8, 121u8, 102u8, 204u8, 197u8, 31u8, 241u8, 131u8,
							199u8, 217u8, 129u8, 147u8, 255u8, 157u8, 71u8, 97u8, 153u8, 34u8,
							178u8, 223u8,
						],
					)
				}
				pub fn queries_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"Queries",
						Vec::new(),
						[
							177u8, 138u8, 131u8, 91u8, 46u8, 151u8, 115u8, 184u8, 120u8, 158u8,
							241u8, 43u8, 56u8, 121u8, 102u8, 204u8, 197u8, 31u8, 241u8, 131u8,
							199u8, 217u8, 129u8, 147u8, 255u8, 157u8, 71u8, 97u8, 153u8, 34u8,
							178u8, 223u8,
						],
					)
				}
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AssetTraps",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
							50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
							126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				pub fn asset_traps_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AssetTraps",
						Vec::new(),
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
							50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
							126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SafeXcmVersion",
						vec![],
						[
							187u8, 8u8, 74u8, 126u8, 80u8, 215u8, 177u8, 60u8, 223u8, 123u8, 196u8,
							155u8, 166u8, 66u8, 25u8, 164u8, 191u8, 66u8, 116u8, 131u8, 116u8,
							188u8, 224u8, 122u8, 75u8, 195u8, 246u8, 188u8, 83u8, 134u8, 49u8,
							143u8,
						],
					)
				}
				pub fn supported_version(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							184u8, 189u8, 75u8, 46u8, 146u8, 183u8, 140u8, 180u8, 228u8, 216u8,
							220u8, 210u8, 194u8, 138u8, 57u8, 248u8, 112u8, 110u8, 236u8, 113u8,
							9u8, 109u8, 112u8, 86u8, 37u8, 21u8, 133u8, 127u8, 86u8, 192u8, 107u8,
							161u8,
						],
					)
				}
				pub fn supported_version_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						Vec::new(),
						[
							184u8, 189u8, 75u8, 46u8, 146u8, 183u8, 140u8, 180u8, 228u8, 216u8,
							220u8, 210u8, 194u8, 138u8, 57u8, 248u8, 112u8, 110u8, 236u8, 113u8,
							9u8, 109u8, 112u8, 86u8, 37u8, 21u8, 133u8, 127u8, 86u8, 192u8, 107u8,
							161u8,
						],
					)
				}
				pub fn version_notifiers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							239u8, 151u8, 74u8, 252u8, 104u8, 241u8, 152u8, 244u8, 122u8, 1u8,
							144u8, 59u8, 172u8, 3u8, 96u8, 233u8, 233u8, 183u8, 152u8, 118u8, 82u8,
							57u8, 122u8, 86u8, 82u8, 99u8, 218u8, 82u8, 37u8, 236u8, 15u8, 212u8,
						],
					)
				}
				pub fn version_notifiers_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						Vec::new(),
						[
							239u8, 151u8, 74u8, 252u8, 104u8, 241u8, 152u8, 244u8, 122u8, 1u8,
							144u8, 59u8, 172u8, 3u8, 96u8, 233u8, 233u8, 183u8, 152u8, 118u8, 82u8,
							57u8, 122u8, 86u8, 82u8, 99u8, 218u8, 82u8, 37u8, 236u8, 15u8, 212u8,
						],
					)
				}
				pub fn version_notify_targets(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							86u8, 209u8, 212u8, 169u8, 60u8, 250u8, 121u8, 19u8, 190u8, 58u8,
							111u8, 103u8, 157u8, 186u8, 211u8, 46u8, 9u8, 111u8, 17u8, 73u8, 221u8,
							180u8, 230u8, 156u8, 4u8, 75u8, 114u8, 123u8, 3u8, 147u8, 237u8, 102u8,
						],
					)
				}
				pub fn version_notify_targets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						Vec::new(),
						[
							86u8, 209u8, 212u8, 169u8, 60u8, 250u8, 121u8, 19u8, 190u8, 58u8,
							111u8, 103u8, 157u8, 186u8, 211u8, 46u8, 9u8, 111u8, 17u8, 73u8, 221u8,
							180u8, 230u8, 156u8, 4u8, 75u8, 114u8, 123u8, 3u8, 147u8, 237u8, 102u8,
						],
					)
				}
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionDiscoveryQueue",
						vec![],
						[
							229u8, 127u8, 178u8, 181u8, 65u8, 123u8, 36u8, 101u8, 245u8, 168u8,
							192u8, 243u8, 57u8, 242u8, 169u8, 89u8, 80u8, 16u8, 28u8, 143u8, 124u8,
							224u8, 11u8, 153u8, 237u8, 58u8, 209u8, 186u8, 94u8, 135u8, 105u8,
							79u8,
						],
					)
				}
				pub fn current_migration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::VersionMigrationStage,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"CurrentMigration",
						vec![],
						[
							74u8, 138u8, 181u8, 162u8, 59u8, 251u8, 37u8, 28u8, 232u8, 51u8, 30u8,
							152u8, 252u8, 133u8, 95u8, 195u8, 47u8, 127u8, 21u8, 44u8, 62u8, 143u8,
							170u8, 234u8, 160u8, 37u8, 131u8, 179u8, 57u8, 241u8, 140u8, 124u8,
						],
					)
				}
				pub fn remote_locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_2: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedAssetId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
						],
						[
							100u8, 88u8, 111u8, 249u8, 153u8, 101u8, 193u8, 52u8, 201u8, 73u8,
							121u8, 226u8, 175u8, 108u8, 169u8, 225u8, 10u8, 105u8, 27u8, 5u8,
							174u8, 129u8, 53u8, 59u8, 80u8, 171u8, 133u8, 99u8, 102u8, 148u8,
							206u8, 159u8,
						],
					)
				}
				pub fn remote_locked_fungibles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						Vec::new(),
						[
							100u8, 88u8, 111u8, 249u8, 153u8, 101u8, 193u8, 52u8, 201u8, 73u8,
							121u8, 226u8, 175u8, 108u8, 169u8, 225u8, 10u8, 105u8, 27u8, 5u8,
							174u8, 129u8, 53u8, 59u8, 80u8, 171u8, 133u8, 99u8, 102u8, 148u8,
							206u8, 159u8,
						],
					)
				}
				pub fn locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedMultiLocation,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							240u8, 80u8, 205u8, 136u8, 21u8, 85u8, 0u8, 172u8, 9u8, 12u8, 36u8,
							16u8, 64u8, 213u8, 52u8, 227u8, 41u8, 150u8, 183u8, 135u8, 224u8,
							203u8, 66u8, 166u8, 115u8, 230u8, 245u8, 178u8, 194u8, 81u8, 35u8,
							245u8,
						],
					)
				}
				pub fn locked_fungibles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedMultiLocation,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						Vec::new(),
						[
							240u8, 80u8, 205u8, 136u8, 21u8, 85u8, 0u8, 172u8, 9u8, 12u8, 36u8,
							16u8, 64u8, 213u8, 52u8, 227u8, 41u8, 150u8, 183u8, 135u8, 224u8,
							203u8, 66u8, 166u8, 115u8, 230u8, 245u8, 178u8, 194u8, 81u8, 35u8,
							245u8,
						],
					)
				}
				pub fn xcm_execution_suspended(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"XcmExecutionSuspended",
						vec![],
						[
							182u8, 54u8, 69u8, 68u8, 78u8, 76u8, 103u8, 79u8, 47u8, 136u8, 99u8,
							104u8, 128u8, 129u8, 249u8, 54u8, 214u8, 136u8, 97u8, 48u8, 178u8,
							42u8, 26u8, 27u8, 82u8, 24u8, 33u8, 77u8, 33u8, 27u8, 20u8, 127u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_xcm::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_xcm::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 32usize],
				pub runtime_types::xcm::v3::traits::Outcome,
			);
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod dmp_queue {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_dmp_queue::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_dmp_queue::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ServiceOverweight {
					pub index: ::core::primitive::u64,
					pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ServiceOverweight {
					const PALLET: &'static str = "DmpQueue";
					const CALL: &'static str = "service_overweight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ServiceOverweight> {
					::subxt::tx::Payload::new_static(
						"DmpQueue",
						"service_overweight",
						types::ServiceOverweight { index, weight_limit },
						[
							191u8, 141u8, 229u8, 214u8, 12u8, 111u8, 251u8, 4u8, 8u8, 118u8, 217u8,
							193u8, 106u8, 182u8, 233u8, 130u8, 149u8, 201u8, 245u8, 3u8, 81u8,
							217u8, 33u8, 24u8, 198u8, 138u8, 206u8, 141u8, 105u8, 31u8, 81u8, 9u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidFormat {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnsupportedVersion {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecutedDownward {
				pub message_id: [::core::primitive::u8; 32usize],
				pub outcome: runtime_types::xcm::v3::traits::Outcome,
			}
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct WeightExhausted {
				pub message_id: [::core::primitive::u8; 32usize],
				pub remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for WeightExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "WeightExhausted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OverweightEnqueued {
				pub message_id: [::core::primitive::u8; 32usize],
				pub overweight_index: ::core::primitive::u64,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OverweightServiced {
				pub overweight_index: ::core::primitive::u64,
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MaxMessagesExhausted {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MaxMessagesExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "MaxMessagesExhausted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn configuration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_dmp_queue::ConfigData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Configuration",
						vec![],
						[
							249u8, 114u8, 102u8, 102u8, 237u8, 226u8, 224u8, 141u8, 68u8, 109u8,
							228u8, 80u8, 170u8, 50u8, 0u8, 68u8, 239u8, 140u8, 12u8, 221u8, 209u8,
							217u8, 0u8, 169u8, 192u8, 28u8, 181u8, 126u8, 178u8, 195u8, 140u8, 1u8,
						],
					)
				}
				pub fn page_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"PageIndex",
						vec![],
						[
							226u8, 180u8, 180u8, 204u8, 141u8, 130u8, 254u8, 63u8, 227u8, 205u8,
							59u8, 68u8, 229u8, 13u8, 144u8, 231u8, 239u8, 130u8, 33u8, 232u8,
							243u8, 57u8, 186u8, 164u8, 9u8, 114u8, 50u8, 28u8, 232u8, 160u8, 82u8,
							113u8,
						],
					)
				}
				pub fn pages(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Pages",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							88u8, 119u8, 86u8, 13u8, 104u8, 212u8, 85u8, 248u8, 75u8, 50u8, 254u8,
							228u8, 105u8, 48u8, 35u8, 177u8, 170u8, 145u8, 227u8, 90u8, 240u8,
							29u8, 241u8, 193u8, 175u8, 32u8, 244u8, 45u8, 129u8, 218u8, 175u8,
							190u8,
						],
					)
				}
				pub fn pages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Pages",
						Vec::new(),
						[
							88u8, 119u8, 86u8, 13u8, 104u8, 212u8, 85u8, 248u8, 75u8, 50u8, 254u8,
							228u8, 105u8, 48u8, 35u8, 177u8, 170u8, 145u8, 227u8, 90u8, 240u8,
							29u8, 241u8, 193u8, 175u8, 32u8, 244u8, 45u8, 129u8, 218u8, 175u8,
							190u8,
						],
					)
				}
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::std::vec::Vec<::core::primitive::u8>),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Overweight",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							86u8, 97u8, 243u8, 7u8, 134u8, 189u8, 7u8, 126u8, 8u8, 108u8, 152u8,
							48u8, 230u8, 8u8, 71u8, 83u8, 151u8, 125u8, 18u8, 168u8, 38u8, 38u8,
							117u8, 85u8, 143u8, 187u8, 122u8, 13u8, 104u8, 52u8, 198u8, 138u8,
						],
					)
				}
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::std::vec::Vec<::core::primitive::u8>),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Overweight",
						Vec::new(),
						[
							86u8, 97u8, 243u8, 7u8, 134u8, 189u8, 7u8, 126u8, 8u8, 108u8, 152u8,
							48u8, 230u8, 8u8, 71u8, 83u8, 151u8, 125u8, 18u8, 168u8, 38u8, 38u8,
							117u8, 85u8, 143u8, 187u8, 122u8, 13u8, 104u8, 52u8, 198u8, 138u8,
						],
					)
				}
				pub fn counter_for_overweight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"CounterForOverweight",
						vec![],
						[
							44u8, 249u8, 133u8, 204u8, 169u8, 253u8, 23u8, 157u8, 132u8, 193u8,
							28u8, 178u8, 156u8, 176u8, 206u8, 46u8, 79u8, 254u8, 174u8, 236u8,
							143u8, 219u8, 59u8, 43u8, 36u8, 109u8, 244u8, 206u8, 48u8, 126u8,
							247u8, 0u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_sudo::pallet::Error;
		pub type Call = runtime_types::pallet_sudo::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Sudo {
					pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Sudo {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoUncheckedWeight {
					pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_unchecked_weight";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetKey {
					pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "set_key";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoAs {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoAs {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_as";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn sudo(
					&self,
					call: runtime_types::parachain_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Sudo> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo",
						types::Sudo { call: ::std::boxed::Box::new(call) },
						[
							18u8, 32u8, 30u8, 32u8, 94u8, 83u8, 240u8, 30u8, 219u8, 211u8, 54u8,
							135u8, 74u8, 237u8, 110u8, 181u8, 254u8, 213u8, 230u8, 12u8, 141u8,
							131u8, 188u8, 37u8, 9u8, 170u8, 127u8, 111u8, 219u8, 148u8, 233u8,
							92u8,
						],
					)
				}
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::parachain_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						types::SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							94u8, 248u8, 124u8, 203u8, 3u8, 123u8, 125u8, 16u8, 81u8, 238u8, 205u8,
							40u8, 149u8, 56u8, 65u8, 79u8, 231u8, 158u8, 65u8, 252u8, 82u8, 86u8,
							20u8, 26u8, 66u8, 225u8, 103u8, 239u8, 36u8, 202u8, 67u8, 18u8,
						],
					)
				}
				pub fn set_key(
					&self,
					new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::SetKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"set_key",
						types::SetKey { new },
						[
							196u8, 87u8, 177u8, 124u8, 216u8, 66u8, 124u8, 56u8, 67u8, 36u8, 3u8,
							124u8, 2u8, 5u8, 175u8, 110u8, 34u8, 76u8, 108u8, 144u8, 19u8, 65u8,
							84u8, 173u8, 73u8, 179u8, 64u8, 70u8, 190u8, 141u8, 173u8, 195u8,
						],
					)
				}
				pub fn sudo_as(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					call: runtime_types::parachain_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::SudoAs> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_as",
						types::SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							120u8, 248u8, 241u8, 129u8, 142u8, 13u8, 146u8, 33u8, 17u8, 245u8,
							141u8, 244u8, 73u8, 29u8, 180u8, 144u8, 103u8, 49u8, 156u8, 171u8,
							45u8, 50u8, 38u8, 231u8, 135u8, 147u8, 228u8, 113u8, 75u8, 249u8, 91u8,
							33u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn key(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Sudo",
						"Key",
						vec![],
						[
							72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
							31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
							36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
						],
					)
				}
			}
		}
	}
	pub mod ibc_ping {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_ibc_ping::pallet::Error;
		pub type Call = runtime_types::pallet_ibc_ping::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SendPing {
					pub params: runtime_types::pallet_ibc_ping::SendPingParams,
				}
				impl ::subxt::blocks::StaticExtrinsic for SendPing {
					const PALLET: &'static str = "IbcPing";
					const CALL: &'static str = "send_ping";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send_ping(
					&self,
					params: runtime_types::pallet_ibc_ping::SendPingParams,
				) -> ::subxt::tx::Payload<types::SendPing> {
					::subxt::tx::Payload::new_static(
						"IbcPing",
						"send_ping",
						types::SendPing { params },
						[
							67u8, 246u8, 189u8, 119u8, 246u8, 27u8, 221u8, 218u8, 216u8, 213u8,
							0u8, 169u8, 51u8, 30u8, 220u8, 151u8, 227u8, 174u8, 227u8, 27u8, 195u8,
							104u8, 154u8, 196u8, 180u8, 52u8, 82u8, 111u8, 27u8, 180u8, 231u8,
							28u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_ibc_ping::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PacketSent;
			impl ::subxt::events::StaticEvent for PacketSent {
				const PALLET: &'static str = "IbcPing";
				const EVENT: &'static str = "PacketSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "IbcPing";
				const EVENT: &'static str = "ChannelOpened";
			}
		}
	}
	pub mod assets {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_assets::pallet::Error;
		pub type Call = runtime_types::pallet_assets::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Create {
					pub id: ::core::primitive::u128,
					pub admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub min_balance: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Create {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "create";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceCreate {
					pub id: ::core::primitive::u128,
					pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub is_sufficient: ::core::primitive::bool,
					#[codec(compact)]
					pub min_balance: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceCreate {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "force_create";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct StartDestroy {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for StartDestroy {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "start_destroy";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DestroyAccounts {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for DestroyAccounts {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "destroy_accounts";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DestroyApprovals {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for DestroyApprovals {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "destroy_approvals";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FinishDestroy {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for FinishDestroy {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "finish_destroy";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Mint {
					pub id: ::core::primitive::u128,
					pub beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Mint {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "mint";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Burn {
					pub id: ::core::primitive::u128,
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Burn {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "burn";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Transfer {
					pub id: ::core::primitive::u128,
					pub target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferKeepAlive {
					pub id: ::core::primitive::u128,
					pub target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceTransfer {
					pub id: ::core::primitive::u128,
					pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Freeze {
					pub id: ::core::primitive::u128,
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Freeze {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "freeze";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Thaw {
					pub id: ::core::primitive::u128,
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Thaw {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "thaw";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FreezeAsset {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for FreezeAsset {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "freeze_asset";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ThawAsset {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ThawAsset {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "thaw_asset";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferOwnership {
					pub id: ::core::primitive::u128,
					pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferOwnership {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "transfer_ownership";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetTeam {
					pub id: ::core::primitive::u128,
					pub issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetTeam {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "set_team";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetMetadata {
					pub id: ::core::primitive::u128,
					pub name: ::std::vec::Vec<::core::primitive::u8>,
					pub symbol: ::std::vec::Vec<::core::primitive::u8>,
					pub decimals: ::core::primitive::u8,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMetadata {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "set_metadata";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClearMetadata {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ClearMetadata {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "clear_metadata";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSetMetadata {
					pub id: ::core::primitive::u128,
					pub name: ::std::vec::Vec<::core::primitive::u8>,
					pub symbol: ::std::vec::Vec<::core::primitive::u8>,
					pub decimals: ::core::primitive::u8,
					pub is_frozen: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSetMetadata {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "force_set_metadata";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceClearMetadata {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceClearMetadata {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "force_clear_metadata";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceAssetStatus {
					pub id: ::core::primitive::u128,
					pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub min_balance: ::core::primitive::u128,
					pub is_sufficient: ::core::primitive::bool,
					pub is_frozen: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceAssetStatus {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "force_asset_status";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ApproveTransfer {
					pub id: ::core::primitive::u128,
					pub delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApproveTransfer {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "approve_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CancelApproval {
					pub id: ::core::primitive::u128,
					pub delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelApproval {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "cancel_approval";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceCancelApproval {
					pub id: ::core::primitive::u128,
					pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceCancelApproval {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "force_cancel_approval";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferApproved {
					pub id: ::core::primitive::u128,
					pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferApproved {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "transfer_approved";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Touch {
					pub id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Touch {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "touch";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Refund {
					pub id: ::core::primitive::u128,
					pub allow_burn: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Refund {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "refund";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetMinBalance {
					pub id: ::core::primitive::u128,
					pub min_balance: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMinBalance {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "set_min_balance";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TouchOther {
					pub id: ::core::primitive::u128,
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for TouchOther {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "touch_other";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RefundOther {
					pub id: ::core::primitive::u128,
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RefundOther {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "refund_other";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Block {
					pub id: ::core::primitive::u128,
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Block {
					const PALLET: &'static str = "Assets";
					const CALL: &'static str = "block";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn create(
					&self,
					id: ::core::primitive::u128,
					admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					min_balance: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Create> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"create",
						types::Create { id, admin, min_balance },
						[
							19u8, 199u8, 119u8, 195u8, 24u8, 5u8, 251u8, 83u8, 243u8, 176u8, 182u8,
							76u8, 238u8, 45u8, 85u8, 87u8, 242u8, 183u8, 175u8, 32u8, 21u8, 180u8,
							195u8, 77u8, 119u8, 205u8, 176u8, 29u8, 179u8, 54u8, 140u8, 162u8,
						],
					)
				}
				pub fn force_create(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					is_sufficient: ::core::primitive::bool,
					min_balance: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceCreate> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_create",
						types::ForceCreate { id, owner, is_sufficient, min_balance },
						[
							209u8, 118u8, 68u8, 42u8, 122u8, 75u8, 40u8, 204u8, 25u8, 123u8, 78u8,
							68u8, 121u8, 74u8, 44u8, 8u8, 156u8, 2u8, 49u8, 59u8, 182u8, 209u8,
							27u8, 203u8, 222u8, 229u8, 143u8, 114u8, 181u8, 134u8, 0u8, 21u8,
						],
					)
				}
				pub fn start_destroy(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::StartDestroy> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"start_destroy",
						types::StartDestroy { id },
						[
							129u8, 88u8, 175u8, 221u8, 30u8, 251u8, 209u8, 188u8, 68u8, 162u8,
							84u8, 243u8, 150u8, 149u8, 230u8, 85u8, 79u8, 116u8, 205u8, 66u8,
							218u8, 77u8, 117u8, 192u8, 35u8, 95u8, 157u8, 57u8, 102u8, 128u8,
							221u8, 255u8,
						],
					)
				}
				pub fn destroy_accounts(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::DestroyAccounts> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"destroy_accounts",
						types::DestroyAccounts { id },
						[
							22u8, 20u8, 76u8, 31u8, 99u8, 100u8, 53u8, 246u8, 233u8, 186u8, 67u8,
							208u8, 121u8, 28u8, 96u8, 137u8, 132u8, 228u8, 127u8, 49u8, 4u8, 208u8,
							182u8, 60u8, 123u8, 23u8, 234u8, 209u8, 109u8, 234u8, 18u8, 46u8,
						],
					)
				}
				pub fn destroy_approvals(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::DestroyApprovals> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"destroy_approvals",
						types::DestroyApprovals { id },
						[
							237u8, 25u8, 229u8, 187u8, 223u8, 24u8, 87u8, 66u8, 228u8, 188u8,
							187u8, 150u8, 151u8, 176u8, 97u8, 242u8, 145u8, 215u8, 91u8, 13u8,
							111u8, 164u8, 62u8, 104u8, 97u8, 245u8, 189u8, 251u8, 7u8, 202u8,
							242u8, 241u8,
						],
					)
				}
				pub fn finish_destroy(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::FinishDestroy> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"finish_destroy",
						types::FinishDestroy { id },
						[
							175u8, 176u8, 118u8, 85u8, 4u8, 48u8, 144u8, 145u8, 42u8, 18u8, 107u8,
							147u8, 174u8, 197u8, 33u8, 19u8, 172u8, 191u8, 175u8, 198u8, 202u8,
							36u8, 220u8, 240u8, 25u8, 72u8, 1u8, 28u8, 99u8, 76u8, 103u8, 18u8,
						],
					)
				}
				pub fn mint(
					&self,
					id: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Mint> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"mint",
						types::Mint { id, beneficiary, amount },
						[
							76u8, 243u8, 133u8, 209u8, 74u8, 57u8, 190u8, 186u8, 210u8, 21u8, 71u8,
							223u8, 87u8, 217u8, 56u8, 97u8, 12u8, 159u8, 155u8, 254u8, 163u8, 72u8,
							52u8, 105u8, 113u8, 1u8, 218u8, 214u8, 105u8, 62u8, 75u8, 204u8,
						],
					)
				}
				pub fn burn(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Burn> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"burn",
						types::Burn { id, who, amount },
						[
							55u8, 131u8, 176u8, 220u8, 166u8, 226u8, 111u8, 176u8, 16u8, 41u8,
							205u8, 55u8, 67u8, 163u8, 133u8, 147u8, 165u8, 20u8, 15u8, 222u8, 31u8,
							96u8, 248u8, 215u8, 7u8, 10u8, 108u8, 30u8, 115u8, 96u8, 253u8, 33u8,
						],
					)
				}
				pub fn transfer(
					&self,
					id: ::core::primitive::u128,
					target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer",
						types::Transfer { id, target, amount },
						[
							15u8, 228u8, 199u8, 255u8, 113u8, 158u8, 150u8, 218u8, 243u8, 97u8,
							164u8, 64u8, 233u8, 46u8, 205u8, 124u8, 149u8, 130u8, 241u8, 22u8,
							181u8, 33u8, 246u8, 198u8, 206u8, 11u8, 189u8, 144u8, 85u8, 192u8,
							247u8, 53u8,
						],
					)
				}
				pub fn transfer_keep_alive(
					&self,
					id: ::core::primitive::u128,
					target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer_keep_alive",
						types::TransferKeepAlive { id, target, amount },
						[
							126u8, 201u8, 31u8, 152u8, 225u8, 187u8, 147u8, 198u8, 119u8, 107u8,
							224u8, 236u8, 126u8, 149u8, 213u8, 102u8, 209u8, 31u8, 9u8, 37u8,
							253u8, 28u8, 83u8, 171u8, 113u8, 101u8, 17u8, 158u8, 75u8, 105u8,
							116u8, 14u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					id: ::core::primitive::u128,
					source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_transfer",
						types::ForceTransfer { id, source, dest, amount },
						[
							26u8, 194u8, 190u8, 247u8, 151u8, 214u8, 164u8, 41u8, 170u8, 135u8,
							221u8, 222u8, 109u8, 13u8, 210u8, 9u8, 196u8, 251u8, 116u8, 130u8,
							135u8, 73u8, 229u8, 73u8, 119u8, 251u8, 55u8, 66u8, 126u8, 169u8,
							216u8, 252u8,
						],
					)
				}
				pub fn freeze(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::Freeze> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"freeze",
						types::Freeze { id, who },
						[
							21u8, 41u8, 58u8, 137u8, 233u8, 182u8, 107u8, 239u8, 144u8, 217u8,
							194u8, 241u8, 49u8, 76u8, 194u8, 153u8, 26u8, 200u8, 21u8, 124u8, 2u8,
							212u8, 216u8, 139u8, 173u8, 205u8, 181u8, 78u8, 0u8, 73u8, 106u8, 32u8,
						],
					)
				}
				pub fn thaw(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::Thaw> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"thaw",
						types::Thaw { id, who },
						[
							119u8, 197u8, 175u8, 1u8, 73u8, 240u8, 200u8, 38u8, 82u8, 223u8, 221u8,
							211u8, 142u8, 178u8, 57u8, 90u8, 161u8, 228u8, 237u8, 177u8, 8u8,
							126u8, 23u8, 35u8, 219u8, 187u8, 48u8, 91u8, 184u8, 238u8, 120u8,
							246u8,
						],
					)
				}
				pub fn freeze_asset(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::FreezeAsset> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"freeze_asset",
						types::FreezeAsset { id },
						[
							254u8, 245u8, 1u8, 7u8, 18u8, 167u8, 140u8, 64u8, 118u8, 134u8, 75u8,
							132u8, 44u8, 24u8, 5u8, 145u8, 45u8, 78u8, 63u8, 219u8, 173u8, 222u8,
							93u8, 192u8, 212u8, 37u8, 107u8, 82u8, 161u8, 37u8, 254u8, 210u8,
						],
					)
				}
				pub fn thaw_asset(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ThawAsset> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"thaw_asset",
						types::ThawAsset { id },
						[
							236u8, 24u8, 198u8, 160u8, 160u8, 31u8, 235u8, 157u8, 82u8, 243u8,
							27u8, 139u8, 255u8, 15u8, 93u8, 41u8, 192u8, 150u8, 218u8, 124u8, 78u8,
							94u8, 84u8, 75u8, 38u8, 216u8, 237u8, 19u8, 186u8, 78u8, 136u8, 251u8,
						],
					)
				}
				pub fn transfer_ownership(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::TransferOwnership> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer_ownership",
						types::TransferOwnership { id, owner },
						[
							185u8, 193u8, 27u8, 95u8, 223u8, 82u8, 109u8, 241u8, 146u8, 99u8, 83u8,
							190u8, 16u8, 132u8, 163u8, 213u8, 38u8, 241u8, 23u8, 158u8, 227u8,
							76u8, 72u8, 89u8, 1u8, 102u8, 34u8, 165u8, 76u8, 43u8, 162u8, 51u8,
						],
					)
				}
				pub fn set_team(
					&self,
					id: ::core::primitive::u128,
					issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::SetTeam> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"set_team",
						types::SetTeam { id, issuer, admin, freezer },
						[
							205u8, 188u8, 157u8, 67u8, 76u8, 244u8, 24u8, 219u8, 65u8, 169u8,
							177u8, 43u8, 207u8, 197u8, 58u8, 147u8, 2u8, 153u8, 181u8, 90u8, 111u8,
							153u8, 254u8, 194u8, 57u8, 66u8, 240u8, 93u8, 217u8, 100u8, 14u8, 94u8,
						],
					)
				}
				pub fn set_metadata(
					&self,
					id: ::core::primitive::u128,
					name: ::std::vec::Vec<::core::primitive::u8>,
					symbol: ::std::vec::Vec<::core::primitive::u8>,
					decimals: ::core::primitive::u8,
				) -> ::subxt::tx::Payload<types::SetMetadata> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"set_metadata",
						types::SetMetadata { id, name, symbol, decimals },
						[
							91u8, 80u8, 32u8, 142u8, 194u8, 237u8, 213u8, 37u8, 211u8, 126u8,
							203u8, 162u8, 106u8, 172u8, 217u8, 8u8, 86u8, 116u8, 190u8, 211u8,
							28u8, 25u8, 123u8, 115u8, 45u8, 164u8, 0u8, 154u8, 8u8, 183u8, 120u8,
							212u8,
						],
					)
				}
				pub fn clear_metadata(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ClearMetadata> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"clear_metadata",
						types::ClearMetadata { id },
						[
							203u8, 99u8, 112u8, 180u8, 96u8, 110u8, 16u8, 87u8, 175u8, 99u8, 121u8,
							167u8, 63u8, 252u8, 54u8, 227u8, 94u8, 206u8, 102u8, 149u8, 235u8,
							237u8, 53u8, 32u8, 109u8, 70u8, 58u8, 30u8, 15u8, 1u8, 29u8, 17u8,
						],
					)
				}
				pub fn force_set_metadata(
					&self,
					id: ::core::primitive::u128,
					name: ::std::vec::Vec<::core::primitive::u8>,
					symbol: ::std::vec::Vec<::core::primitive::u8>,
					decimals: ::core::primitive::u8,
					is_frozen: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceSetMetadata> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_set_metadata",
						types::ForceSetMetadata { id, name, symbol, decimals, is_frozen },
						[
							178u8, 109u8, 199u8, 65u8, 73u8, 139u8, 214u8, 205u8, 255u8, 134u8,
							173u8, 68u8, 113u8, 247u8, 113u8, 44u8, 84u8, 127u8, 11u8, 186u8,
							120u8, 109u8, 60u8, 202u8, 4u8, 27u8, 95u8, 59u8, 92u8, 47u8, 247u8,
							209u8,
						],
					)
				}
				pub fn force_clear_metadata(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceClearMetadata> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_clear_metadata",
						types::ForceClearMetadata { id },
						[
							222u8, 60u8, 84u8, 196u8, 97u8, 61u8, 212u8, 47u8, 77u8, 212u8, 11u8,
							238u8, 0u8, 67u8, 37u8, 13u8, 243u8, 135u8, 31u8, 3u8, 245u8, 208u8,
							121u8, 131u8, 101u8, 213u8, 242u8, 136u8, 202u8, 112u8, 63u8, 229u8,
						],
					)
				}
				pub fn force_asset_status(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					min_balance: ::core::primitive::u128,
					is_sufficient: ::core::primitive::bool,
					is_frozen: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceAssetStatus> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_asset_status",
						types::ForceAssetStatus {
							id,
							owner,
							issuer,
							admin,
							freezer,
							min_balance,
							is_sufficient,
							is_frozen,
						},
						[
							240u8, 42u8, 153u8, 221u8, 89u8, 73u8, 252u8, 73u8, 104u8, 145u8,
							176u8, 0u8, 147u8, 181u8, 49u8, 12u8, 218u8, 190u8, 4u8, 4u8, 242u8,
							189u8, 49u8, 160u8, 96u8, 249u8, 16u8, 236u8, 160u8, 220u8, 112u8,
							202u8,
						],
					)
				}
				pub fn approve_transfer(
					&self,
					id: ::core::primitive::u128,
					delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ApproveTransfer> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"approve_transfer",
						types::ApproveTransfer { id, delegate, amount },
						[
							186u8, 147u8, 66u8, 101u8, 249u8, 60u8, 76u8, 66u8, 104u8, 95u8, 32u8,
							147u8, 192u8, 251u8, 145u8, 78u8, 4u8, 163u8, 59u8, 166u8, 56u8, 119u8,
							146u8, 33u8, 28u8, 174u8, 165u8, 119u8, 167u8, 244u8, 217u8, 143u8,
						],
					)
				}
				pub fn cancel_approval(
					&self,
					id: ::core::primitive::u128,
					delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::CancelApproval> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"cancel_approval",
						types::CancelApproval { id, delegate },
						[
							27u8, 201u8, 191u8, 69u8, 45u8, 175u8, 172u8, 75u8, 31u8, 35u8, 72u8,
							158u8, 107u8, 195u8, 58u8, 167u8, 174u8, 66u8, 66u8, 147u8, 202u8,
							23u8, 134u8, 173u8, 47u8, 254u8, 8u8, 103u8, 135u8, 250u8, 216u8,
							104u8,
						],
					)
				}
				pub fn force_cancel_approval(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::ForceCancelApproval> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_cancel_approval",
						types::ForceCancelApproval { id, owner, delegate },
						[
							205u8, 34u8, 210u8, 148u8, 134u8, 83u8, 126u8, 140u8, 7u8, 6u8, 145u8,
							35u8, 165u8, 197u8, 5u8, 226u8, 229u8, 155u8, 126u8, 91u8, 79u8, 81u8,
							204u8, 237u8, 66u8, 180u8, 112u8, 199u8, 218u8, 206u8, 88u8, 35u8,
						],
					)
				}
				pub fn transfer_approved(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferApproved> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer_approved",
						types::TransferApproved { id, owner, destination, amount },
						[
							209u8, 27u8, 162u8, 164u8, 240u8, 184u8, 44u8, 36u8, 59u8, 92u8, 229u8,
							32u8, 79u8, 168u8, 5u8, 183u8, 178u8, 26u8, 41u8, 10u8, 207u8, 65u8,
							242u8, 47u8, 10u8, 109u8, 105u8, 21u8, 25u8, 85u8, 64u8, 213u8,
						],
					)
				}
				pub fn touch(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Touch> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"touch",
						types::Touch { id },
						[
							168u8, 100u8, 155u8, 13u8, 105u8, 248u8, 4u8, 46u8, 72u8, 33u8, 89u8,
							176u8, 9u8, 149u8, 229u8, 153u8, 152u8, 231u8, 18u8, 239u8, 90u8,
							175u8, 166u8, 207u8, 175u8, 3u8, 165u8, 100u8, 121u8, 122u8, 5u8,
							127u8,
						],
					)
				}
				pub fn refund(
					&self,
					id: ::core::primitive::u128,
					allow_burn: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Refund> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"refund",
						types::Refund { id, allow_burn },
						[
							11u8, 149u8, 84u8, 174u8, 94u8, 77u8, 229u8, 83u8, 150u8, 22u8, 235u8,
							35u8, 35u8, 245u8, 114u8, 183u8, 13u8, 230u8, 150u8, 139u8, 103u8,
							251u8, 1u8, 167u8, 152u8, 157u8, 245u8, 101u8, 240u8, 38u8, 162u8,
							215u8,
						],
					)
				}
				pub fn set_min_balance(
					&self,
					id: ::core::primitive::u128,
					min_balance: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetMinBalance> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"set_min_balance",
						types::SetMinBalance { id, min_balance },
						[
							127u8, 41u8, 74u8, 32u8, 31u8, 0u8, 107u8, 210u8, 148u8, 70u8, 46u8,
							35u8, 115u8, 224u8, 243u8, 161u8, 127u8, 102u8, 33u8, 16u8, 238u8,
							204u8, 126u8, 161u8, 155u8, 61u8, 182u8, 198u8, 161u8, 219u8, 65u8,
							242u8,
						],
					)
				}
				pub fn touch_other(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::TouchOther> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"touch_other",
						types::TouchOther { id, who },
						[
							93u8, 140u8, 190u8, 48u8, 22u8, 21u8, 162u8, 123u8, 201u8, 54u8, 120u8,
							57u8, 209u8, 42u8, 83u8, 227u8, 52u8, 86u8, 9u8, 62u8, 224u8, 194u8,
							118u8, 165u8, 114u8, 116u8, 69u8, 228u8, 237u8, 123u8, 166u8, 53u8,
						],
					)
				}
				pub fn refund_other(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::RefundOther> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"refund_other",
						types::RefundOther { id, who },
						[
							113u8, 104u8, 179u8, 20u8, 158u8, 230u8, 122u8, 153u8, 170u8, 136u8,
							3u8, 144u8, 209u8, 6u8, 20u8, 196u8, 236u8, 81u8, 81u8, 76u8, 57u8,
							192u8, 234u8, 100u8, 173u8, 91u8, 156u8, 108u8, 176u8, 109u8, 36u8,
							143u8,
						],
					)
				}
				pub fn block(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::Block> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"block",
						types::Block { id, who },
						[
							212u8, 167u8, 27u8, 122u8, 81u8, 50u8, 179u8, 201u8, 39u8, 110u8, 37u8,
							93u8, 131u8, 51u8, 215u8, 93u8, 35u8, 199u8, 69u8, 222u8, 50u8, 253u8,
							77u8, 184u8, 252u8, 145u8, 194u8, 23u8, 220u8, 32u8, 98u8, 221u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_assets::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Created {
				pub asset_id: ::core::primitive::u128,
				pub creator: ::subxt::utils::AccountId32,
				pub owner: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Created {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Created";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Issued {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Transferred {
				pub asset_id: ::core::primitive::u128,
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transferred {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Transferred";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Burned {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TeamChanged {
				pub asset_id: ::core::primitive::u128,
				pub issuer: ::subxt::utils::AccountId32,
				pub admin: ::subxt::utils::AccountId32,
				pub freezer: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for TeamChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "TeamChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OwnerChanged {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for OwnerChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "OwnerChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Frozen {
				pub asset_id: ::core::primitive::u128,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Thawed {
				pub asset_id: ::core::primitive::u128,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Thawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetFrozen {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetFrozen {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetFrozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetThawed {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetThawed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetThawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AccountsDestroyed {
				pub asset_id: ::core::primitive::u128,
				pub accounts_destroyed: ::core::primitive::u32,
				pub accounts_remaining: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for AccountsDestroyed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AccountsDestroyed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ApprovalsDestroyed {
				pub asset_id: ::core::primitive::u128,
				pub approvals_destroyed: ::core::primitive::u32,
				pub approvals_remaining: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ApprovalsDestroyed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ApprovalsDestroyed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DestructionStarted {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DestructionStarted {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "DestructionStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Destroyed {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Destroyed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Destroyed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceCreated {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for ForceCreated {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ForceCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MetadataSet {
				pub asset_id: ::core::primitive::u128,
				pub name: ::std::vec::Vec<::core::primitive::u8>,
				pub symbol: ::std::vec::Vec<::core::primitive::u8>,
				pub decimals: ::core::primitive::u8,
				pub is_frozen: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for MetadataSet {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "MetadataSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MetadataCleared {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for MetadataCleared {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "MetadataCleared";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ApprovedTransfer {
				pub asset_id: ::core::primitive::u128,
				pub source: ::subxt::utils::AccountId32,
				pub delegate: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for ApprovedTransfer {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ApprovedTransfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ApprovalCancelled {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub delegate: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for ApprovalCancelled {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ApprovalCancelled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferredApproved {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub delegate: ::subxt::utils::AccountId32,
				pub destination: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransferredApproved {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "TransferredApproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetStatusChanged {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetStatusChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetStatusChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetMinBalanceChanged {
				pub asset_id: ::core::primitive::u128,
				pub new_min_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetMinBalanceChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetMinBalanceChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Touched {
				pub asset_id: ::core::primitive::u128,
				pub who: ::subxt::utils::AccountId32,
				pub depositor: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Touched {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Touched";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Blocked {
				pub asset_id: ::core::primitive::u128,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Blocked {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Blocked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn asset(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::AssetDetails<
						::core::primitive::u128,
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Asset",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							25u8, 6u8, 243u8, 98u8, 121u8, 56u8, 242u8, 220u8, 32u8, 220u8, 226u8,
							23u8, 43u8, 32u8, 221u8, 188u8, 231u8, 68u8, 243u8, 179u8, 123u8,
							113u8, 122u8, 217u8, 83u8, 71u8, 80u8, 95u8, 10u8, 224u8, 242u8, 249u8,
						],
					)
				}
				pub fn asset_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::AssetDetails<
						::core::primitive::u128,
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Asset",
						Vec::new(),
						[
							25u8, 6u8, 243u8, 98u8, 121u8, 56u8, 242u8, 220u8, 32u8, 220u8, 226u8,
							23u8, 43u8, 32u8, 221u8, 188u8, 231u8, 68u8, 243u8, 179u8, 123u8,
							113u8, 122u8, 217u8, 83u8, 71u8, 80u8, 95u8, 10u8, 224u8, 242u8, 249u8,
						],
					)
				}
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::AssetAccount<
						::core::primitive::u128,
						::core::primitive::u128,
						(),
						::subxt::utils::AccountId32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Account",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							68u8, 240u8, 33u8, 139u8, 110u8, 121u8, 187u8, 75u8, 21u8, 210u8,
							221u8, 39u8, 33u8, 163u8, 38u8, 35u8, 92u8, 159u8, 177u8, 48u8, 195u8,
							196u8, 115u8, 79u8, 39u8, 142u8, 218u8, 50u8, 77u8, 102u8, 9u8, 127u8,
						],
					)
				}
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::AssetAccount<
						::core::primitive::u128,
						::core::primitive::u128,
						(),
						::subxt::utils::AccountId32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Account",
						Vec::new(),
						[
							68u8, 240u8, 33u8, 139u8, 110u8, 121u8, 187u8, 75u8, 21u8, 210u8,
							221u8, 39u8, 33u8, 163u8, 38u8, 35u8, 92u8, 159u8, 177u8, 48u8, 195u8,
							196u8, 115u8, 79u8, 39u8, 142u8, 218u8, 50u8, 77u8, 102u8, 9u8, 127u8,
						],
					)
				}
				pub fn approvals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_2: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::Approval<
						::core::primitive::u128,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Approvals",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
						],
						[
							25u8, 10u8, 241u8, 35u8, 145u8, 42u8, 71u8, 155u8, 107u8, 130u8, 196u8,
							205u8, 56u8, 103u8, 138u8, 168u8, 214u8, 61u8, 8u8, 114u8, 244u8,
							193u8, 119u8, 231u8, 249u8, 213u8, 221u8, 66u8, 23u8, 53u8, 138u8,
							167u8,
						],
					)
				}
				pub fn approvals_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::Approval<
						::core::primitive::u128,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Approvals",
						Vec::new(),
						[
							25u8, 10u8, 241u8, 35u8, 145u8, 42u8, 71u8, 155u8, 107u8, 130u8, 196u8,
							205u8, 56u8, 103u8, 138u8, 168u8, 214u8, 61u8, 8u8, 114u8, 244u8,
							193u8, 119u8, 231u8, 249u8, 213u8, 221u8, 66u8, 23u8, 53u8, 138u8,
							167u8,
						],
					)
				}
				pub fn metadata(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::AssetMetadata<
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Metadata",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							174u8, 52u8, 190u8, 56u8, 219u8, 36u8, 226u8, 185u8, 117u8, 18u8,
							147u8, 36u8, 226u8, 6u8, 39u8, 2u8, 179u8, 75u8, 200u8, 111u8, 108u8,
							254u8, 78u8, 24u8, 182u8, 172u8, 163u8, 253u8, 211u8, 44u8, 241u8,
							53u8,
						],
					)
				}
				pub fn metadata_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_assets::types::AssetMetadata<
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Assets",
						"Metadata",
						Vec::new(),
						[
							174u8, 52u8, 190u8, 56u8, 219u8, 36u8, 226u8, 185u8, 117u8, 18u8,
							147u8, 36u8, 226u8, 6u8, 39u8, 2u8, 179u8, 75u8, 200u8, 111u8, 108u8,
							254u8, 78u8, 24u8, 182u8, 172u8, 163u8, 253u8, 211u8, 44u8, 241u8,
							53u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn remove_items_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Assets",
						"RemoveItemsLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn asset_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Assets",
						"AssetDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn asset_account_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Assets",
						"AssetAccountDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn metadata_deposit_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Assets",
						"MetadataDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn metadata_deposit_per_byte(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Assets",
						"MetadataDepositPerByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn approval_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Assets",
						"ApprovalDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn string_limit(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Assets",
						"StringLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod asset_registry {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::orml_asset_registry::module::Error;
		pub type Call = runtime_types::orml_asset_registry::module::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RegisterAsset {
					pub metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
						::core::primitive::u128,
						(),
					>,
					pub asset_id: ::core::option::Option<::core::primitive::u128>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RegisterAsset {
					const PALLET: &'static str = "AssetRegistry";
					const CALL: &'static str = "register_asset";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateAsset {
					pub asset_id: ::core::primitive::u128,
					pub decimals: ::core::option::Option<::core::primitive::u32>,
					pub name: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					pub symbol: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					pub existential_deposit: ::core::option::Option<::core::primitive::u128>,
					pub location: ::core::option::Option<
						::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
					>,
					pub additional: ::core::option::Option<()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateAsset {
					const PALLET: &'static str = "AssetRegistry";
					const CALL: &'static str = "update_asset";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn register_asset(
					&self,
					metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
						::core::primitive::u128,
						(),
					>,
					asset_id: ::core::option::Option<::core::primitive::u128>,
				) -> ::subxt::tx::Payload<types::RegisterAsset> {
					::subxt::tx::Payload::new_static(
						"AssetRegistry",
						"register_asset",
						types::RegisterAsset { metadata, asset_id },
						[
							108u8, 207u8, 88u8, 172u8, 25u8, 108u8, 91u8, 28u8, 236u8, 157u8, 63u8,
							195u8, 62u8, 193u8, 72u8, 188u8, 40u8, 35u8, 225u8, 237u8, 89u8, 125u8,
							148u8, 244u8, 225u8, 153u8, 220u8, 106u8, 54u8, 132u8, 139u8, 96u8,
						],
					)
				}
				pub fn update_asset(
					&self,
					asset_id: ::core::primitive::u128,
					decimals: ::core::option::Option<::core::primitive::u32>,
					name: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					symbol: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					existential_deposit: ::core::option::Option<::core::primitive::u128>,
					location: ::core::option::Option<
						::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
					>,
					additional: ::core::option::Option<()>,
				) -> ::subxt::tx::Payload<types::UpdateAsset> {
					::subxt::tx::Payload::new_static(
						"AssetRegistry",
						"update_asset",
						types::UpdateAsset {
							asset_id,
							decimals,
							name,
							symbol,
							existential_deposit,
							location,
							additional,
						},
						[
							20u8, 174u8, 11u8, 191u8, 217u8, 40u8, 85u8, 26u8, 223u8, 142u8, 94u8,
							93u8, 94u8, 226u8, 132u8, 163u8, 191u8, 110u8, 70u8, 243u8, 49u8,
							138u8, 117u8, 130u8, 10u8, 255u8, 154u8, 37u8, 58u8, 162u8, 59u8,
							212u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::orml_asset_registry::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RegisteredAsset {
				pub asset_id: ::core::primitive::u128,
				pub metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
					::core::primitive::u128,
					(),
				>,
			}
			impl ::subxt::events::StaticEvent for RegisteredAsset {
				const PALLET: &'static str = "AssetRegistry";
				const EVENT: &'static str = "RegisteredAsset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdatedAsset {
				pub asset_id: ::core::primitive::u128,
				pub metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
					::core::primitive::u128,
					(),
				>,
			}
			impl ::subxt::events::StaticEvent for UpdatedAsset {
				const PALLET: &'static str = "AssetRegistry";
				const EVENT: &'static str = "UpdatedAsset";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn metadata(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::orml_traits::asset_registry::AssetMetadata<
						::core::primitive::u128,
						(),
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetRegistry",
						"Metadata",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							235u8, 106u8, 181u8, 114u8, 101u8, 48u8, 116u8, 4u8, 187u8, 167u8,
							158u8, 196u8, 172u8, 65u8, 69u8, 67u8, 189u8, 112u8, 47u8, 165u8, 35u8,
							232u8, 135u8, 150u8, 211u8, 213u8, 55u8, 22u8, 250u8, 64u8, 135u8,
							88u8,
						],
					)
				}
				pub fn metadata_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::orml_traits::asset_registry::AssetMetadata<
						::core::primitive::u128,
						(),
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetRegistry",
						"Metadata",
						Vec::new(),
						[
							235u8, 106u8, 181u8, 114u8, 101u8, 48u8, 116u8, 4u8, 187u8, 167u8,
							158u8, 196u8, 172u8, 65u8, 69u8, 67u8, 189u8, 112u8, 47u8, 165u8, 35u8,
							232u8, 135u8, 150u8, 211u8, 213u8, 55u8, 22u8, 250u8, 64u8, 135u8,
							88u8,
						],
					)
				}
				pub fn location_to_asset_id(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::v3::multilocation::MultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetRegistry",
						"LocationToAssetId",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							237u8, 85u8, 165u8, 249u8, 132u8, 108u8, 81u8, 86u8, 81u8, 237u8,
							214u8, 147u8, 190u8, 211u8, 144u8, 102u8, 65u8, 20u8, 108u8, 144u8,
							149u8, 55u8, 75u8, 222u8, 196u8, 44u8, 60u8, 107u8, 249u8, 126u8, 34u8,
							181u8,
						],
					)
				}
				pub fn location_to_asset_id_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetRegistry",
						"LocationToAssetId",
						Vec::new(),
						[
							237u8, 85u8, 165u8, 249u8, 132u8, 108u8, 81u8, 86u8, 81u8, 237u8,
							214u8, 147u8, 190u8, 211u8, 144u8, 102u8, 65u8, 20u8, 108u8, 144u8,
							149u8, 55u8, 75u8, 222u8, 196u8, 44u8, 60u8, 107u8, 249u8, 126u8, 34u8,
							181u8,
						],
					)
				}
				pub fn last_asset_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AssetRegistry",
						"LastAssetId",
						vec![],
						[
							205u8, 108u8, 239u8, 204u8, 119u8, 128u8, 163u8, 33u8, 248u8, 115u8,
							163u8, 39u8, 206u8, 153u8, 13u8, 173u8, 68u8, 38u8, 189u8, 82u8, 119u8,
							216u8, 170u8, 251u8, 137u8, 222u8, 36u8, 36u8, 5u8, 184u8, 243u8, 88u8,
						],
					)
				}
			}
		}
	}
	pub mod ibc {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_ibc::pallet::Error;
		pub type Call = runtime_types::pallet_ibc::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Deliver {
					pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Deliver {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "deliver";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Transfer {
					pub params:
						runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
					pub asset_id: ::core::primitive::u128,
					pub amount: ::core::primitive::u128,
					pub memo: ::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpgradeClient {
					pub params: runtime_types::pallet_ibc::UpgradeParams,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeClient {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "upgrade_client";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FreezeClient {
					pub client_id: ::std::vec::Vec<::core::primitive::u8>,
					pub height: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for FreezeClient {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "freeze_client";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IncreaseCounters;
				impl ::subxt::blocks::StaticExtrinsic for IncreaseCounters {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "increase_counters";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AddChannelsToFeelessChannelList {
					pub source_channel: ::core::primitive::u64,
					pub destination_channel: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddChannelsToFeelessChannelList {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "add_channels_to_feeless_channel_list";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveChannelsFromFeelessChannelList {
					pub source_channel: ::core::primitive::u64,
					pub destination_channel: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveChannelsFromFeelessChannelList {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "remove_channels_from_feeless_channel_list";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetChildStorage {
					pub key: ::std::vec::Vec<::core::primitive::u8>,
					pub value: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetChildStorage {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "set_child_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SubstituteClientState {
					pub client_id: ::std::string::String,
					pub height: runtime_types::ibc::core::ics02_client::height::Height,
					pub client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					pub consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SubstituteClientState {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "substitute_client_state";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn deliver(
					&self,
					messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				) -> ::subxt::tx::Payload<types::Deliver> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"deliver",
						types::Deliver { messages },
						[
							172u8, 108u8, 202u8, 218u8, 161u8, 9u8, 26u8, 231u8, 26u8, 10u8, 248u8,
							118u8, 238u8, 72u8, 137u8, 49u8, 125u8, 168u8, 175u8, 195u8, 152u8,
							13u8, 15u8, 109u8, 162u8, 50u8, 39u8, 76u8, 12u8, 121u8, 4u8, 88u8,
						],
					)
				}
				pub fn transfer(
					&self,
					params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
					asset_id: ::core::primitive::u128,
					amount: ::core::primitive::u128,
					memo: ::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"transfer",
						types::Transfer { params, asset_id, amount, memo },
						[
							83u8, 73u8, 238u8, 203u8, 83u8, 164u8, 50u8, 136u8, 100u8, 244u8, 86u8,
							16u8, 84u8, 72u8, 132u8, 210u8, 66u8, 223u8, 224u8, 251u8, 106u8, 20u8,
							85u8, 152u8, 141u8, 225u8, 250u8, 225u8, 60u8, 53u8, 187u8, 177u8,
						],
					)
				}
				pub fn upgrade_client(
					&self,
					params: runtime_types::pallet_ibc::UpgradeParams,
				) -> ::subxt::tx::Payload<types::UpgradeClient> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"upgrade_client",
						types::UpgradeClient { params },
						[
							136u8, 241u8, 209u8, 215u8, 167u8, 18u8, 46u8, 4u8, 180u8, 211u8,
							206u8, 87u8, 35u8, 220u8, 28u8, 117u8, 219u8, 157u8, 108u8, 216u8,
							164u8, 252u8, 10u8, 64u8, 11u8, 68u8, 235u8, 152u8, 31u8, 229u8, 186u8,
							60u8,
						],
					)
				}
				pub fn freeze_client(
					&self,
					client_id: ::std::vec::Vec<::core::primitive::u8>,
					height: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::FreezeClient> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"freeze_client",
						types::FreezeClient { client_id, height },
						[
							113u8, 2u8, 129u8, 175u8, 26u8, 200u8, 192u8, 198u8, 33u8, 79u8, 157u8,
							230u8, 222u8, 140u8, 139u8, 83u8, 46u8, 104u8, 155u8, 252u8, 241u8,
							115u8, 83u8, 85u8, 255u8, 160u8, 27u8, 110u8, 216u8, 204u8, 36u8, 76u8,
						],
					)
				}
				pub fn increase_counters(&self) -> ::subxt::tx::Payload<types::IncreaseCounters> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"increase_counters",
						types::IncreaseCounters {},
						[
							23u8, 152u8, 106u8, 41u8, 249u8, 118u8, 33u8, 254u8, 129u8, 72u8, 33u8,
							238u8, 90u8, 101u8, 181u8, 145u8, 237u8, 244u8, 146u8, 226u8, 229u8,
							32u8, 140u8, 133u8, 218u8, 21u8, 207u8, 4u8, 76u8, 251u8, 237u8, 93u8,
						],
					)
				}
				pub fn add_channels_to_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::AddChannelsToFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"add_channels_to_feeless_channel_list",
						types::AddChannelsToFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							169u8, 204u8, 104u8, 69u8, 180u8, 7u8, 182u8, 226u8, 184u8, 252u8,
							99u8, 141u8, 110u8, 251u8, 56u8, 123u8, 165u8, 35u8, 16u8, 218u8,
							133u8, 148u8, 75u8, 70u8, 36u8, 16u8, 7u8, 180u8, 81u8, 218u8, 167u8,
							213u8,
						],
					)
				}
				pub fn remove_channels_from_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::RemoveChannelsFromFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"remove_channels_from_feeless_channel_list",
						types::RemoveChannelsFromFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							237u8, 33u8, 247u8, 167u8, 226u8, 81u8, 137u8, 60u8, 121u8, 147u8,
							170u8, 115u8, 151u8, 134u8, 67u8, 94u8, 241u8, 53u8, 60u8, 55u8, 74u8,
							178u8, 161u8, 78u8, 50u8, 12u8, 239u8, 179u8, 116u8, 81u8, 8u8, 32u8,
						],
					)
				}
				pub fn set_child_storage(
					&self,
					key: ::std::vec::Vec<::core::primitive::u8>,
					value: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetChildStorage> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"set_child_storage",
						types::SetChildStorage { key, value },
						[
							139u8, 28u8, 205u8, 76u8, 87u8, 10u8, 43u8, 171u8, 183u8, 69u8, 47u8,
							139u8, 34u8, 218u8, 68u8, 65u8, 89u8, 195u8, 163u8, 182u8, 22u8, 133u8,
							217u8, 11u8, 233u8, 56u8, 121u8, 171u8, 148u8, 76u8, 253u8, 214u8,
						],
					)
				}
				pub fn substitute_client_state(
					&self,
					client_id: ::std::string::String,
					height: runtime_types::ibc::core::ics02_client::height::Height,
					client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SubstituteClientState> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"substitute_client_state",
						types::SubstituteClientState {
							client_id,
							height,
							client_state_bytes,
							consensus_state_bytes,
						},
						[
							135u8, 25u8, 90u8, 50u8, 36u8, 216u8, 99u8, 223u8, 110u8, 48u8, 210u8,
							68u8, 212u8, 56u8, 88u8, 164u8, 215u8, 129u8, 67u8, 168u8, 122u8,
							234u8, 135u8, 90u8, 24u8, 2u8, 191u8, 138u8, 24u8, 195u8, 128u8, 82u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_ibc::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Events {
				pub events: ::std::vec::Vec<
					::core::result::Result<
						runtime_types::pallet_ibc::events::IbcEvent,
						runtime_types::pallet_ibc::errors::IbcError,
					>,
				>,
			}
			impl ::subxt::events::StaticEvent for Events {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "Events";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferInitiated {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferInitiated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferInitiated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChannelOpened";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ParamsUpdated {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for ParamsUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ParamsUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferCompleted {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferCompleted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenReceived {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
				pub amount: ::core::primitive::u128,
				pub is_receiver_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenReceived {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferFailed {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferTimeout {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferTimeout {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferTimeout";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OnRecvPacketError {
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for OnRecvPacketError {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "OnRecvPacketError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClientUpgradeSet;
			impl ::subxt::events::StaticEvent for ClientUpgradeSet {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientUpgradeSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClientFrozen {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
				pub revision_number: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ClientFrozen {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientFrozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetAdminUpdated {
				pub admin_account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for AssetAdminUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "AssetAdminUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FeeLessChannelIdsAdded {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsAdded {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "FeeLessChannelIdsAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FeeLessChannelIdsRemoved {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsRemoved {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "FeeLessChannelIdsRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargingFeeOnTransferInitiated {
				pub sequence: ::core::primitive::u64,
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
				pub amount: ::core::primitive::u128,
				pub is_flat_fee: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeOnTransferInitiated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeOnTransferInitiated";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargingFeeConfirmed {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeConfirmed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeConfirmed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargingFeeTimeout {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeTimeout {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeTimeout";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargingFeeFailedAcknowledgement {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeFailedAcknowledgement {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeFailedAcknowledgement";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChildStateUpdated;
			impl ::subxt::events::StaticEvent for ChildStateUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChildStateUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClientStateSubstituted {
				pub client_id: ::std::string::String,
				pub height: runtime_types::ibc::core::ics02_client::height::Height,
			}
			impl ::subxt::events::StaticEvent for ClientStateSubstituted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientStateSubstituted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoStarted {
				pub account_id: ::subxt::utils::AccountId32,
				pub memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoStarted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoIbcTokenTransferSuccess {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub asset_id: ::core::primitive::u128,
				pub amount: ::core::primitive::u128,
				pub channel: ::core::primitive::u64,
				pub next_memo: ::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferSuccess {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoIbcTokenTransferFailedWithReason {
				pub from: ::subxt::utils::AccountId32,
				pub memo: ::std::string::String,
				pub reason: ::core::primitive::u8,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferFailedWithReason {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferFailedWithReason";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoIbcTokenTransferFailed {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub asset_id: ::core::primitive::u128,
				pub amount: ::core::primitive::u128,
				pub channel: ::core::primitive::u64,
				pub next_memo: ::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoXcmSuccess {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub asset_id: ::core::primitive::u128,
				pub para_id: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoXcmSuccess {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoXcmSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoXcmFailed {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub asset_id: ::core::primitive::u128,
				pub para_id: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoXcmFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoXcmFailed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn client_update_height(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateHeight",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							155u8, 151u8, 188u8, 28u8, 116u8, 139u8, 236u8, 229u8, 213u8, 48u8,
							98u8, 68u8, 244u8, 26u8, 21u8, 33u8, 110u8, 34u8, 22u8, 41u8, 19u8,
							76u8, 199u8, 252u8, 81u8, 168u8, 134u8, 232u8, 170u8, 21u8, 156u8,
							186u8,
						],
					)
				}
				pub fn client_update_height_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateHeight",
						Vec::new(),
						[
							155u8, 151u8, 188u8, 28u8, 116u8, 139u8, 236u8, 229u8, 213u8, 48u8,
							98u8, 68u8, 244u8, 26u8, 21u8, 33u8, 110u8, 34u8, 22u8, 41u8, 19u8,
							76u8, 199u8, 252u8, 81u8, 168u8, 134u8, 232u8, 170u8, 21u8, 156u8,
							186u8,
						],
					)
				}
				pub fn service_charge_out(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ServiceChargeOut",
						vec![],
						[
							131u8, 102u8, 75u8, 64u8, 161u8, 69u8, 217u8, 111u8, 153u8, 9u8, 32u8,
							20u8, 79u8, 224u8, 110u8, 11u8, 215u8, 10u8, 235u8, 83u8, 154u8, 125u8,
							69u8, 205u8, 62u8, 167u8, 110u8, 18u8, 36u8, 95u8, 143u8, 58u8,
						],
					)
				}
				pub fn client_update_time(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateTime",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							14u8, 48u8, 14u8, 183u8, 216u8, 185u8, 220u8, 201u8, 37u8, 50u8, 149u8,
							62u8, 16u8, 163u8, 184u8, 43u8, 99u8, 157u8, 110u8, 70u8, 217u8, 4u8,
							93u8, 237u8, 187u8, 84u8, 29u8, 4u8, 126u8, 121u8, 20u8, 150u8,
						],
					)
				}
				pub fn client_update_time_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateTime",
						Vec::new(),
						[
							14u8, 48u8, 14u8, 183u8, 216u8, 185u8, 220u8, 201u8, 37u8, 50u8, 149u8,
							62u8, 16u8, 163u8, 184u8, 43u8, 99u8, 157u8, 110u8, 70u8, 217u8, 4u8,
							93u8, 237u8, 187u8, 84u8, 29u8, 4u8, 126u8, 121u8, 20u8, 150u8,
						],
					)
				}
				pub fn channel_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelCounter",
						vec![],
						[
							61u8, 228u8, 58u8, 117u8, 65u8, 31u8, 204u8, 92u8, 12u8, 247u8, 4u8,
							176u8, 121u8, 146u8, 112u8, 118u8, 201u8, 138u8, 154u8, 6u8, 4u8,
							201u8, 218u8, 64u8, 250u8, 201u8, 50u8, 148u8, 87u8, 172u8, 215u8,
							152u8,
						],
					)
				}
				pub fn packet_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PacketCounter",
						vec![],
						[
							6u8, 61u8, 72u8, 154u8, 126u8, 33u8, 9u8, 44u8, 64u8, 112u8, 73u8,
							229u8, 28u8, 106u8, 109u8, 206u8, 11u8, 152u8, 124u8, 157u8, 252u8,
							28u8, 153u8, 248u8, 16u8, 60u8, 33u8, 198u8, 78u8, 254u8, 10u8, 244u8,
						],
					)
				}
				pub fn channels_connection(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelsConnection",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							61u8, 56u8, 30u8, 52u8, 104u8, 79u8, 38u8, 145u8, 177u8, 198u8, 11u8,
							183u8, 39u8, 83u8, 126u8, 54u8, 113u8, 42u8, 69u8, 84u8, 85u8, 94u8,
							46u8, 185u8, 159u8, 129u8, 72u8, 1u8, 2u8, 54u8, 171u8, 254u8,
						],
					)
				}
				pub fn channels_connection_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelsConnection",
						Vec::new(),
						[
							61u8, 56u8, 30u8, 52u8, 104u8, 79u8, 38u8, 145u8, 177u8, 198u8, 11u8,
							183u8, 39u8, 83u8, 126u8, 54u8, 113u8, 42u8, 69u8, 84u8, 85u8, 94u8,
							46u8, 185u8, 159u8, 129u8, 72u8, 1u8, 2u8, 54u8, 171u8, 254u8,
						],
					)
				}
				pub fn fee_less_channel_ids(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"FeeLessChannelIds",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							14u8, 192u8, 26u8, 54u8, 234u8, 220u8, 170u8, 40u8, 189u8, 79u8, 218u8,
							180u8, 69u8, 98u8, 80u8, 62u8, 178u8, 228u8, 223u8, 45u8, 148u8, 105u8,
							146u8, 246u8, 161u8, 14u8, 85u8, 112u8, 181u8, 15u8, 47u8, 225u8,
						],
					)
				}
				pub fn fee_less_channel_ids_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"FeeLessChannelIds",
						Vec::new(),
						[
							14u8, 192u8, 26u8, 54u8, 234u8, 220u8, 170u8, 40u8, 189u8, 79u8, 218u8,
							180u8, 69u8, 98u8, 80u8, 62u8, 178u8, 228u8, 223u8, 45u8, 148u8, 105u8,
							146u8, 246u8, 161u8, 14u8, 85u8, 112u8, 181u8, 15u8, 47u8, 225u8,
						],
					)
				}
				pub fn sequence_fee(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SequenceFee",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							66u8, 41u8, 5u8, 19u8, 126u8, 113u8, 10u8, 96u8, 77u8, 206u8, 105u8,
							119u8, 8u8, 92u8, 211u8, 69u8, 216u8, 169u8, 91u8, 108u8, 227u8, 237u8,
							196u8, 25u8, 18u8, 68u8, 253u8, 253u8, 21u8, 205u8, 3u8, 184u8,
						],
					)
				}
				pub fn sequence_fee_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SequenceFee",
						Vec::new(),
						[
							66u8, 41u8, 5u8, 19u8, 126u8, 113u8, 10u8, 96u8, 77u8, 206u8, 105u8,
							119u8, 8u8, 92u8, 211u8, 69u8, 216u8, 169u8, 91u8, 108u8, 227u8, 237u8,
							196u8, 25u8, 18u8, 68u8, 253u8, 253u8, 21u8, 205u8, 3u8, 184u8,
						],
					)
				}
				pub fn client_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientCounter",
						vec![],
						[
							25u8, 194u8, 154u8, 231u8, 229u8, 240u8, 25u8, 94u8, 46u8, 3u8, 137u8,
							130u8, 211u8, 146u8, 87u8, 105u8, 17u8, 95u8, 56u8, 55u8, 184u8, 245u8,
							170u8, 189u8, 108u8, 17u8, 167u8, 134u8, 161u8, 89u8, 168u8, 243u8,
						],
					)
				}
				pub fn connection_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConnectionCounter",
						vec![],
						[
							244u8, 243u8, 169u8, 141u8, 145u8, 248u8, 15u8, 50u8, 150u8, 185u8,
							137u8, 105u8, 14u8, 154u8, 199u8, 20u8, 160u8, 78u8, 64u8, 36u8, 14u8,
							36u8, 46u8, 219u8, 114u8, 164u8, 64u8, 178u8, 1u8, 152u8, 80u8, 33u8,
						],
					)
				}
				pub fn acknowledgement_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"AcknowledgementCounter",
						vec![],
						[
							202u8, 105u8, 177u8, 209u8, 89u8, 60u8, 15u8, 21u8, 193u8, 73u8, 105u8,
							98u8, 195u8, 143u8, 40u8, 194u8, 50u8, 117u8, 157u8, 7u8, 45u8, 115u8,
							179u8, 143u8, 180u8, 121u8, 208u8, 171u8, 88u8, 133u8, 232u8, 84u8,
						],
					)
				}
				pub fn packet_receipt_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PacketReceiptCounter",
						vec![],
						[
							67u8, 12u8, 175u8, 101u8, 87u8, 155u8, 170u8, 123u8, 89u8, 82u8, 227u8,
							122u8, 221u8, 201u8, 185u8, 186u8, 236u8, 193u8, 153u8, 59u8, 195u8,
							145u8, 19u8, 56u8, 254u8, 51u8, 130u8, 110u8, 167u8, 148u8, 210u8, 1u8,
						],
					)
				}
				pub fn connection_client(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConnectionClient",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							83u8, 232u8, 147u8, 248u8, 232u8, 163u8, 16u8, 70u8, 5u8, 126u8, 202u8,
							74u8, 247u8, 4u8, 128u8, 0u8, 97u8, 40u8, 45u8, 223u8, 77u8, 103u8,
							3u8, 8u8, 202u8, 160u8, 249u8, 11u8, 135u8, 42u8, 170u8, 28u8,
						],
					)
				}
				pub fn connection_client_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConnectionClient",
						Vec::new(),
						[
							83u8, 232u8, 147u8, 248u8, 232u8, 163u8, 16u8, 70u8, 5u8, 126u8, 202u8,
							74u8, 247u8, 4u8, 128u8, 0u8, 97u8, 40u8, 45u8, 223u8, 77u8, 103u8,
							3u8, 8u8, 202u8, 160u8, 249u8, 11u8, 135u8, 42u8, 170u8, 28u8,
						],
					)
				}
				pub fn ibc_asset_ids(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcAssetIds",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							231u8, 141u8, 73u8, 64u8, 3u8, 122u8, 38u8, 213u8, 52u8, 208u8, 233u8,
							163u8, 190u8, 200u8, 93u8, 122u8, 132u8, 69u8, 250u8, 135u8, 54u8,
							32u8, 55u8, 3u8, 201u8, 217u8, 171u8, 113u8, 200u8, 73u8, 154u8, 6u8,
						],
					)
				}
				pub fn ibc_asset_ids_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcAssetIds",
						Vec::new(),
						[
							231u8, 141u8, 73u8, 64u8, 3u8, 122u8, 38u8, 213u8, 52u8, 208u8, 233u8,
							163u8, 190u8, 200u8, 93u8, 122u8, 132u8, 69u8, 250u8, 135u8, 54u8,
							32u8, 55u8, 3u8, 201u8, 217u8, 171u8, 113u8, 200u8, 73u8, 154u8, 6u8,
						],
					)
				}
				pub fn counter_for_ibc_asset_ids(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"CounterForIbcAssetIds",
						vec![],
						[
							90u8, 132u8, 19u8, 129u8, 205u8, 73u8, 172u8, 84u8, 103u8, 136u8,
							108u8, 4u8, 28u8, 154u8, 26u8, 233u8, 237u8, 49u8, 179u8, 163u8, 27u8,
							207u8, 33u8, 151u8, 43u8, 149u8, 169u8, 224u8, 169u8, 185u8, 174u8,
							163u8,
						],
					)
				}
				pub fn ibc_denoms(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcDenoms",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							236u8, 1u8, 132u8, 86u8, 85u8, 246u8, 172u8, 13u8, 176u8, 43u8, 0u8,
							44u8, 191u8, 203u8, 130u8, 219u8, 12u8, 240u8, 56u8, 35u8, 34u8, 58u8,
							52u8, 57u8, 110u8, 166u8, 178u8, 14u8, 98u8, 118u8, 186u8, 36u8,
						],
					)
				}
				pub fn ibc_denoms_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcDenoms",
						Vec::new(),
						[
							236u8, 1u8, 132u8, 86u8, 85u8, 246u8, 172u8, 13u8, 176u8, 43u8, 0u8,
							44u8, 191u8, 203u8, 130u8, 219u8, 12u8, 240u8, 56u8, 35u8, 34u8, 58u8,
							52u8, 57u8, 110u8, 166u8, 178u8, 14u8, 98u8, 118u8, 186u8, 36u8,
						],
					)
				}
				pub fn counter_for_ibc_denoms(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"CounterForIbcDenoms",
						vec![],
						[
							173u8, 80u8, 18u8, 126u8, 90u8, 140u8, 11u8, 142u8, 44u8, 11u8, 237u8,
							176u8, 99u8, 234u8, 76u8, 153u8, 72u8, 128u8, 135u8, 74u8, 200u8,
							125u8, 235u8, 251u8, 82u8, 40u8, 108u8, 251u8, 19u8, 100u8, 190u8,
							129u8,
						],
					)
				}
				pub fn channel_ids(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelIds",
						vec![],
						[
							38u8, 162u8, 117u8, 100u8, 229u8, 142u8, 242u8, 166u8, 44u8, 113u8,
							65u8, 252u8, 7u8, 122u8, 116u8, 201u8, 12u8, 46u8, 231u8, 186u8, 254u8,
							172u8, 232u8, 237u8, 81u8, 13u8, 20u8, 156u8, 229u8, 38u8, 221u8, 89u8,
						],
					)
				}
				pub fn escrow_addresses(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"EscrowAddresses",
						vec![],
						[
							67u8, 10u8, 57u8, 221u8, 25u8, 4u8, 126u8, 69u8, 202u8, 103u8, 147u8,
							128u8, 228u8, 191u8, 222u8, 129u8, 252u8, 40u8, 103u8, 233u8, 145u8,
							178u8, 123u8, 64u8, 220u8, 165u8, 16u8, 239u8, 241u8, 109u8, 42u8,
							52u8,
						],
					)
				}
				pub fn consensus_heights(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::ibc::core::ics02_client::height::Height,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConsensusHeights",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							137u8, 67u8, 38u8, 191u8, 173u8, 103u8, 10u8, 53u8, 179u8, 154u8,
							249u8, 162u8, 24u8, 177u8, 26u8, 37u8, 135u8, 23u8, 146u8, 67u8, 193u8,
							100u8, 166u8, 243u8, 57u8, 220u8, 111u8, 6u8, 143u8, 190u8, 115u8,
							195u8,
						],
					)
				}
				pub fn consensus_heights_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::ibc::core::ics02_client::height::Height,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConsensusHeights",
						Vec::new(),
						[
							137u8, 67u8, 38u8, 191u8, 173u8, 103u8, 10u8, 53u8, 179u8, 154u8,
							249u8, 162u8, 24u8, 177u8, 26u8, 37u8, 135u8, 23u8, 146u8, 67u8, 193u8,
							100u8, 166u8, 243u8, 57u8, 220u8, 111u8, 6u8, 143u8, 190u8, 115u8,
							195u8,
						],
					)
				}
				pub fn send_packets(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SendPackets",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							146u8, 36u8, 174u8, 244u8, 3u8, 99u8, 30u8, 80u8, 4u8, 155u8, 40u8,
							93u8, 154u8, 184u8, 154u8, 248u8, 86u8, 2u8, 66u8, 28u8, 216u8, 112u8,
							227u8, 2u8, 84u8, 181u8, 209u8, 107u8, 214u8, 0u8, 113u8, 214u8,
						],
					)
				}
				pub fn send_packets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SendPackets",
						Vec::new(),
						[
							146u8, 36u8, 174u8, 244u8, 3u8, 99u8, 30u8, 80u8, 4u8, 155u8, 40u8,
							93u8, 154u8, 184u8, 154u8, 248u8, 86u8, 2u8, 66u8, 28u8, 216u8, 112u8,
							227u8, 2u8, 84u8, 181u8, 209u8, 107u8, 214u8, 0u8, 113u8, 214u8,
						],
					)
				}
				pub fn recv_packets(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"RecvPackets",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							88u8, 112u8, 94u8, 195u8, 131u8, 124u8, 176u8, 218u8, 61u8, 137u8,
							96u8, 192u8, 216u8, 30u8, 59u8, 193u8, 159u8, 223u8, 137u8, 19u8, 4u8,
							215u8, 187u8, 222u8, 222u8, 166u8, 25u8, 117u8, 177u8, 188u8, 174u8,
							80u8,
						],
					)
				}
				pub fn recv_packets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"RecvPackets",
						Vec::new(),
						[
							88u8, 112u8, 94u8, 195u8, 131u8, 124u8, 176u8, 218u8, 61u8, 137u8,
							96u8, 192u8, 216u8, 30u8, 59u8, 193u8, 159u8, 223u8, 137u8, 19u8, 4u8,
							215u8, 187u8, 222u8, 222u8, 166u8, 25u8, 117u8, 177u8, 188u8, 174u8,
							80u8,
						],
					)
				}
				pub fn acks(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"Acks",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							131u8, 210u8, 94u8, 82u8, 190u8, 32u8, 60u8, 172u8, 246u8, 142u8, 20u8,
							55u8, 0u8, 165u8, 54u8, 177u8, 117u8, 207u8, 17u8, 18u8, 67u8, 45u8,
							21u8, 115u8, 169u8, 55u8, 216u8, 237u8, 197u8, 69u8, 17u8, 150u8,
						],
					)
				}
				pub fn acks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"Acks",
						Vec::new(),
						[
							131u8, 210u8, 94u8, 82u8, 190u8, 32u8, 60u8, 172u8, 246u8, 142u8, 20u8,
							55u8, 0u8, 165u8, 54u8, 177u8, 117u8, 207u8, 17u8, 18u8, 67u8, 45u8,
							21u8, 115u8, 169u8, 55u8, 216u8, 237u8, 197u8, 69u8, 17u8, 150u8,
						],
					)
				}
				pub fn pending_send_packet_seqs(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingSendPacketSeqs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							214u8, 111u8, 108u8, 82u8, 74u8, 7u8, 161u8, 129u8, 147u8, 16u8, 202u8,
							69u8, 50u8, 71u8, 24u8, 208u8, 182u8, 116u8, 228u8, 216u8, 196u8,
							237u8, 134u8, 195u8, 126u8, 163u8, 49u8, 188u8, 76u8, 250u8, 158u8,
							135u8,
						],
					)
				}
				pub fn pending_send_packet_seqs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingSendPacketSeqs",
						Vec::new(),
						[
							214u8, 111u8, 108u8, 82u8, 74u8, 7u8, 161u8, 129u8, 147u8, 16u8, 202u8,
							69u8, 50u8, 71u8, 24u8, 208u8, 182u8, 116u8, 228u8, 216u8, 196u8,
							237u8, 134u8, 195u8, 126u8, 163u8, 49u8, 188u8, 76u8, 250u8, 158u8,
							135u8,
						],
					)
				}
				pub fn pending_recv_packet_seqs(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingRecvPacketSeqs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							240u8, 245u8, 147u8, 74u8, 239u8, 240u8, 113u8, 229u8, 246u8, 12u8,
							130u8, 0u8, 178u8, 251u8, 61u8, 94u8, 150u8, 51u8, 139u8, 160u8, 155u8,
							104u8, 3u8, 22u8, 220u8, 63u8, 119u8, 113u8, 203u8, 165u8, 57u8, 125u8,
						],
					)
				}
				pub fn pending_recv_packet_seqs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingRecvPacketSeqs",
						Vec::new(),
						[
							240u8, 245u8, 147u8, 74u8, 239u8, 240u8, 113u8, 229u8, 246u8, 12u8,
							130u8, 0u8, 178u8, 251u8, 61u8, 94u8, 150u8, 51u8, 139u8, 160u8, 155u8,
							104u8, 3u8, 22u8, 220u8, 63u8, 119u8, 113u8, 203u8, 165u8, 57u8, 125u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn native_asset_id(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"NativeAssetId",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn pallet_prefix(
					&self,
				) -> ::subxt::constants::Address<::std::vec::Vec<::core::primitive::u8>> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"PalletPrefix",
						[
							64u8, 190u8, 244u8, 122u8, 87u8, 182u8, 217u8, 16u8, 55u8, 223u8,
							128u8, 6u8, 112u8, 30u8, 236u8, 222u8, 153u8, 53u8, 247u8, 102u8,
							196u8, 31u8, 6u8, 186u8, 251u8, 209u8, 114u8, 125u8, 213u8, 222u8,
							240u8, 8u8,
						],
					)
				}
				pub fn light_client_protocol(
					&self,
				) -> ::subxt::constants::Address<runtime_types::pallet_ibc::LightClientProtocol>
				{
					::subxt::constants::Address::new_static(
						"Ibc",
						"LightClientProtocol",
						[
							240u8, 114u8, 208u8, 235u8, 143u8, 121u8, 26u8, 220u8, 3u8, 235u8,
							204u8, 181u8, 246u8, 38u8, 3u8, 168u8, 135u8, 78u8, 63u8, 128u8, 219u8,
							159u8, 37u8, 244u8, 202u8, 191u8, 202u8, 74u8, 203u8, 26u8, 44u8,
							132u8,
						],
					)
				}
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"ExpectedBlockTime",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn minimum_connection_delay(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"MinimumConnectionDelay",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn spam_protection_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"SpamProtectionDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn clean_up_packets_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"CleanUpPacketsPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn service_charge_out(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"Ibc",
						"ServiceChargeOut",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_set {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedBTreeSet<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod cumulus_pallet_dmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					OverLimit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					InvalidFormat { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 1)]
					UnsupportedVersion { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 2)]
					ExecutedDownward {
						message_id: [::core::primitive::u8; 32usize],
						outcome: runtime_types::xcm::v3::traits::Outcome,
					},
					#[codec(index = 3)]
					WeightExhausted {
						message_id: [::core::primitive::u8; 32usize],
						remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					OverweightEnqueued {
						message_id: [::core::primitive::u8; 32usize],
						overweight_index: ::core::primitive::u64,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 5)]
					OverweightServiced {
						overweight_index: ::core::primitive::u64,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					MaxMessagesExhausted { message_id: [::core::primitive::u8; 32usize] },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ConfigData {
				pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PageIndexData {
				pub begin_used: ::core::primitive::u32,
				pub end_used: ::core::primitive::u32,
				pub overweight_count: ::core::primitive::u64,
			}
		}
		pub mod cumulus_pallet_parachain_system {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					# [codec (index = 0)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: utils :: H256 , check_version : :: core :: primitive :: bool , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					OverlappingUpgrades,
					#[codec(index = 1)]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					TooBig,
					#[codec(index = 3)]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					NotScheduled,
					#[codec(index = 6)]
					NothingAuthorized,
					#[codec(index = 7)]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ValidationFunctionStored,
					#[codec(index = 1)]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec(index = 2)]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					UpgradeAuthorized { code_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 5)]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: ::subxt::utils::H256,
					},
					#[codec(index = 6)]
					UpwardMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MessagingStateSnapshot { pub dmq_mqc_head : :: subxt :: utils :: H256 , pub relay_dispatch_queue_size : runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: RelayDispachQueueSize , pub ingress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain :: primitives :: Id , runtime_types :: polkadot_primitives :: v4 :: AbridgedHrmpChannel ,) > , pub egress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain :: primitives :: Id , runtime_types :: polkadot_primitives :: v4 :: AbridgedHrmpChannel ,) > , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RelayDispachQueueSize {
					pub remaining_count: ::core::primitive::u32,
					pub remaining_size: ::core::primitive::u32,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CodeUpgradeAuthorization {
				pub code_hash: ::subxt::utils::H256,
				pub check_version: ::core::primitive::bool,
			}
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::xcm::v3::traits::Outcome,
					),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					suspend_xcm_execution,
					#[codec(index = 2)]
					resume_xcm_execution,
					#[codec(index = 3)]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					update_resume_threshold { new: ::core::primitive::u32 },
					#[codec(index = 6)]
					update_threshold_weight { new: runtime_types::sp_weights::weight_v2::Weight },
					#[codec(index = 7)]
					update_weight_restrict_decay {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 8)]
					update_xcmp_max_individual_weight {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					FailedToSend,
					#[codec(index = 1)]
					BadXcmOrigin,
					#[codec(index = 2)]
					BadXcm,
					#[codec(index = 3)]
					BadOverweightIndex,
					#[codec(index = 4)]
					WeightOverLimit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Success {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					Fail {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						error: runtime_types::xcm::v3::traits::Error,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					BadVersion {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 3)]
					BadFormat {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					XcmpMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					OverweightEnqueued {
						sender: runtime_types::polkadot_parachain::primitives::Id,
						sent_at: ::core::primitive::u32,
						index: ::core::primitive::u64,
						required: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					OverweightServiced {
						index: ::core::primitive::u64,
						used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundChannelDetails {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
				pub message_metadata: ::std::vec::Vec<(
					::core::primitive::u32,
					runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum InboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
				pub threshold_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub weight_restrict_decay: runtime_types::sp_weights::weight_v2::Weight,
				pub xcmp_max_individual_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MessageQueueChain(pub ::subxt::utils::H256);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ParachainInherentData {
				pub validation_data:
					runtime_types::polkadot_primitives::v4::PersistedValidationData<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<
						::core::primitive::u32,
					>,
				>,
				pub horizontal_messages: ::subxt::utils::KeyedVec<
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<
							::core::primitive::u32,
						>,
					>,
				>,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 6)]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidSpecName,
					#[codec(index = 1)]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					NonDefaultComposite,
					#[codec(index = 4)]
					NonZeroRefCount,
					#[codec(index = 5)]
					CallFiltered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					CodeUpdated,
					#[codec(index = 3)]
					NewAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					KilledAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					Remarked { sender: ::subxt::utils::AccountId32, hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod ibc {
			use super::runtime_types;
			pub mod core {
				use super::runtime_types;
				pub mod ics02_client {
					use super::runtime_types;
					pub mod height {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub struct Height {
							pub revision_number: ::core::primitive::u64,
							pub revision_height: ::core::primitive::u64,
						}
					}
				}
			}
			pub mod signer {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signer(pub ::std::string::String);
			}
		}
		pub mod ibc_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Timeout {
				#[codec(index = 0)]
				Offset {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
				#[codec(index = 1)]
				Absolute {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
			}
		}
		pub mod orml_asset_registry {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					register_asset {
						metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
						asset_id: ::core::option::Option<::core::primitive::u128>,
					},
					#[codec(index = 1)]
					update_asset {
						asset_id: ::core::primitive::u128,
						decimals: ::core::option::Option<::core::primitive::u32>,
						name: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						symbol: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						existential_deposit: ::core::option::Option<::core::primitive::u128>,
						location: ::core::option::Option<
							::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
						>,
						additional: ::core::option::Option<()>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					AssetNotFound,
					#[codec(index = 1)]
					BadVersion,
					#[codec(index = 2)]
					InvalidAssetId,
					#[codec(index = 3)]
					ConflictingLocation,
					#[codec(index = 4)]
					ConflictingAssetId,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					RegisteredAsset {
						asset_id: ::core::primitive::u128,
						metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
					},
					#[codec(index = 1)]
					UpdatedAsset {
						asset_id: ::core::primitive::u128,
						metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
					},
				}
			}
		}
		pub mod orml_traits {
			use super::runtime_types;
			pub mod asset_registry {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AssetMetadata<_0, _1> {
					pub decimals: ::core::primitive::u32,
					pub name: ::std::vec::Vec<::core::primitive::u8>,
					pub symbol: ::std::vec::Vec<::core::primitive::u8>,
					pub existential_deposit: _0,
					pub location:
						::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
					pub additional: _1,
				}
			}
		}
		pub mod pallet_assets {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					create {
						id: ::core::primitive::u128,
						admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					force_create {
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						is_sufficient: ::core::primitive::bool,
						#[codec(compact)]
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					start_destroy { id: ::core::primitive::u128 },
					#[codec(index = 3)]
					destroy_accounts { id: ::core::primitive::u128 },
					#[codec(index = 4)]
					destroy_approvals { id: ::core::primitive::u128 },
					#[codec(index = 5)]
					finish_destroy { id: ::core::primitive::u128 },
					#[codec(index = 6)]
					mint {
						id: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					burn {
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					transfer {
						id: ::core::primitive::u128,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					transfer_keep_alive {
						id: ::core::primitive::u128,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					force_transfer {
						id: ::core::primitive::u128,
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					freeze {
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 12)]
					thaw {
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 13)]
					freeze_asset { id: ::core::primitive::u128 },
					#[codec(index = 14)]
					thaw_asset { id: ::core::primitive::u128 },
					#[codec(index = 15)]
					transfer_ownership {
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 16)]
					set_team {
						id: ::core::primitive::u128,
						issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 17)]
					set_metadata {
						id: ::core::primitive::u128,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
					},
					#[codec(index = 18)]
					clear_metadata { id: ::core::primitive::u128 },
					#[codec(index = 19)]
					force_set_metadata {
						id: ::core::primitive::u128,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 20)]
					force_clear_metadata { id: ::core::primitive::u128 },
					#[codec(index = 21)]
					force_asset_status {
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						min_balance: ::core::primitive::u128,
						is_sufficient: ::core::primitive::bool,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 22)]
					approve_transfer {
						id: ::core::primitive::u128,
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 23)]
					cancel_approval {
						id: ::core::primitive::u128,
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 24)]
					force_cancel_approval {
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 25)]
					transfer_approved {
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 26)]
					touch { id: ::core::primitive::u128 },
					#[codec(index = 27)]
					refund { id: ::core::primitive::u128, allow_burn: ::core::primitive::bool },
					#[codec(index = 28)]
					set_min_balance {
						id: ::core::primitive::u128,
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 29)]
					touch_other {
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 30)]
					refund_other {
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 31)]
					block {
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					BalanceLow,
					#[codec(index = 1)]
					NoAccount,
					#[codec(index = 2)]
					NoPermission,
					#[codec(index = 3)]
					Unknown,
					#[codec(index = 4)]
					Frozen,
					#[codec(index = 5)]
					InUse,
					#[codec(index = 6)]
					BadWitness,
					#[codec(index = 7)]
					MinBalanceZero,
					#[codec(index = 8)]
					UnavailableConsumer,
					#[codec(index = 9)]
					BadMetadata,
					#[codec(index = 10)]
					Unapproved,
					#[codec(index = 11)]
					WouldDie,
					#[codec(index = 12)]
					AlreadyExists,
					#[codec(index = 13)]
					NoDeposit,
					#[codec(index = 14)]
					WouldBurn,
					#[codec(index = 15)]
					LiveAsset,
					#[codec(index = 16)]
					AssetNotLive,
					#[codec(index = 17)]
					IncorrectStatus,
					#[codec(index = 18)]
					NotFrozen,
					#[codec(index = 19)]
					CallbackFailed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Created {
						asset_id: ::core::primitive::u128,
						creator: ::subxt::utils::AccountId32,
						owner: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					Issued {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transferred {
						asset_id: ::core::primitive::u128,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					Burned {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					TeamChanged {
						asset_id: ::core::primitive::u128,
						issuer: ::subxt::utils::AccountId32,
						admin: ::subxt::utils::AccountId32,
						freezer: ::subxt::utils::AccountId32,
					},
					#[codec(index = 5)]
					OwnerChanged {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
					},
					#[codec(index = 6)]
					Frozen { asset_id: ::core::primitive::u128, who: ::subxt::utils::AccountId32 },
					#[codec(index = 7)]
					Thawed { asset_id: ::core::primitive::u128, who: ::subxt::utils::AccountId32 },
					#[codec(index = 8)]
					AssetFrozen { asset_id: ::core::primitive::u128 },
					#[codec(index = 9)]
					AssetThawed { asset_id: ::core::primitive::u128 },
					#[codec(index = 10)]
					AccountsDestroyed {
						asset_id: ::core::primitive::u128,
						accounts_destroyed: ::core::primitive::u32,
						accounts_remaining: ::core::primitive::u32,
					},
					#[codec(index = 11)]
					ApprovalsDestroyed {
						asset_id: ::core::primitive::u128,
						approvals_destroyed: ::core::primitive::u32,
						approvals_remaining: ::core::primitive::u32,
					},
					#[codec(index = 12)]
					DestructionStarted { asset_id: ::core::primitive::u128 },
					#[codec(index = 13)]
					Destroyed { asset_id: ::core::primitive::u128 },
					#[codec(index = 14)]
					ForceCreated {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
					},
					#[codec(index = 15)]
					MetadataSet {
						asset_id: ::core::primitive::u128,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					MetadataCleared { asset_id: ::core::primitive::u128 },
					#[codec(index = 17)]
					ApprovedTransfer {
						asset_id: ::core::primitive::u128,
						source: ::subxt::utils::AccountId32,
						delegate: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					ApprovalCancelled {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						delegate: ::subxt::utils::AccountId32,
					},
					#[codec(index = 19)]
					TransferredApproved {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						delegate: ::subxt::utils::AccountId32,
						destination: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					AssetStatusChanged { asset_id: ::core::primitive::u128 },
					#[codec(index = 21)]
					AssetMinBalanceChanged {
						asset_id: ::core::primitive::u128,
						new_min_balance: ::core::primitive::u128,
					},
					#[codec(index = 22)]
					Touched {
						asset_id: ::core::primitive::u128,
						who: ::subxt::utils::AccountId32,
						depositor: ::subxt::utils::AccountId32,
					},
					#[codec(index = 23)]
					Blocked { asset_id: ::core::primitive::u128, who: ::subxt::utils::AccountId32 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AccountStatus {
					#[codec(index = 0)]
					Liquid,
					#[codec(index = 1)]
					Frozen,
					#[codec(index = 2)]
					Blocked,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Approval<_0, _1> {
					pub amount: _0,
					pub deposit: _0,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AssetAccount<_0, _1, _2, _3> {
					pub balance: _0,
					pub status: runtime_types::pallet_assets::types::AccountStatus,
					pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0, _3>,
					pub extra: _2,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AssetDetails<_0, _1, _2> {
					pub owner: _1,
					pub issuer: _1,
					pub admin: _1,
					pub freezer: _1,
					pub supply: _0,
					pub deposit: _0,
					pub min_balance: _0,
					pub is_sufficient: ::core::primitive::bool,
					pub accounts: ::core::primitive::u32,
					pub sufficients: ::core::primitive::u32,
					pub approvals: ::core::primitive::u32,
					pub status: runtime_types::pallet_assets::types::AssetStatus,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AssetMetadata<_0, _1> {
					pub deposit: _0,
					pub name: _1,
					pub symbol: _1,
					pub decimals: ::core::primitive::u8,
					pub is_frozen: ::core::primitive::bool,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AssetStatus {
					#[codec(index = 0)]
					Live,
					#[codec(index = 1)]
					Frozen,
					#[codec(index = 2)]
					Destroying,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ExistenceReason<_0, _1> {
					#[codec(index = 0)]
					Consumer,
					#[codec(index = 1)]
					Sufficient,
					#[codec(index = 2)]
					DepositHeld(_0),
					#[codec(index = 3)]
					DepositRefunded,
					#[codec(index = 4)]
					DepositFrom(_1, _0),
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					transfer_allow_death {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					set_balance_deprecated {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						old_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					force_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					upgrade_accounts { who: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 7)]
					transfer {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					force_set_balance {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					VestingBalance,
					#[codec(index = 1)]
					LiquidityRestrictions,
					#[codec(index = 2)]
					InsufficientBalance,
					#[codec(index = 3)]
					ExistentialDeposit,
					#[codec(index = 4)]
					Expendability,
					#[codec(index = 5)]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
					#[codec(index = 8)]
					TooManyHolds,
					#[codec(index = 9)]
					TooManyFreezes,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					BalanceSet { who: ::subxt::utils::AccountId32, free: ::core::primitive::u128 },
					#[codec(index = 4)]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					Minted { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					Burned { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					Suspended { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 13)]
					Restored { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 14)]
					Upgraded { who: ::subxt::utils::AccountId32 },
					#[codec(index = 15)]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					Locked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					Unlocked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 19)]
					Frozen { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					Thawed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_invulnerables { new: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					register_as_candidate,
					#[codec(index = 4)]
					leave_intent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooManyCandidates,
					#[codec(index = 1)]
					TooFewCandidates,
					#[codec(index = 2)]
					Unknown,
					#[codec(index = 3)]
					Permission,
					#[codec(index = 4)]
					AlreadyCandidate,
					#[codec(index = 5)]
					NotCandidate,
					#[codec(index = 6)]
					TooManyInvulnerables,
					#[codec(index = 7)]
					AlreadyInvulnerable,
					#[codec(index = 8)]
					NoAssociatedValidatorId,
					#[codec(index = 9)]
					ValidatorNotRegistered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewInvulnerables { invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					NewDesiredCandidates { desired_candidates: ::core::primitive::u32 },
					#[codec(index = 2)]
					NewCandidacyBond { bond_amount: ::core::primitive::u128 },
					#[codec(index = 3)]
					CandidateAdded {
						account_id: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					CandidateRemoved { account_id: ::subxt::utils::AccountId32 },
				}
			}
		}
		pub mod pallet_ibc {
			use super::runtime_types;
			pub mod errors {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum IbcError {
					#[codec(index = 0)]
					Ics02Client { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					Ics03Connection { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 2)]
					Ics04Channel { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					Ics20FungibleTokenTransfer { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					UnknownMessageTypeUrl { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 5)]
					MalformedMessageBytes { message: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
			pub mod events {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum IbcEvent {
					#[codec(index = 0)]
					NewBlock {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					CreateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					UpdateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					UpgradeClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					ClientMisbehaviour {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					OpenInitConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					OpenConfirmConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					OpenTryConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 8)]
					OpenAckConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 9)]
					OpenInitChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 10)]
					OpenConfirmChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 11)]
					OpenTryChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 12)]
					OpenAckChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 13)]
					CloseInitChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 14)]
					CloseConfirmChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 15)]
					ReceivePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 16)]
					SendPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 17)]
					AcknowledgePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 18)]
					WriteAcknowledgement {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					TimeoutPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 20)]
					TimeoutOnClosePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 21)]
					Empty,
					#[codec(index = 22)]
					ChainError,
					#[codec(index = 23)]
					AppModule {
						kind: ::std::vec::Vec<::core::primitive::u8>,
						module_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 24)]
					PushWasmCode { wasm_checksum: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					deliver { messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any> },
					#[codec(index = 1)]
					transfer {
						params:
							runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
						asset_id: ::core::primitive::u128,
						amount: ::core::primitive::u128,
						memo: ::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
					},
					#[codec(index = 3)]
					upgrade_client { params: runtime_types::pallet_ibc::UpgradeParams },
					#[codec(index = 4)]
					freeze_client {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					increase_counters,
					#[codec(index = 6)]
					add_channels_to_feeless_channel_list {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 7)]
					remove_channels_from_feeless_channel_list {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 8)]
					set_child_storage {
						key: ::std::vec::Vec<::core::primitive::u8>,
						value: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 9)]
					substitute_client_state {
						client_id: ::std::string::String,
						height: runtime_types::ibc::core::ics02_client::height::Height,
						client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
						consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ProcessingError,
					#[codec(index = 1)]
					DecodingError,
					#[codec(index = 2)]
					EncodingError,
					#[codec(index = 3)]
					ProofGenerationError,
					#[codec(index = 4)]
					ConsensusStateNotFound,
					#[codec(index = 5)]
					ChannelNotFound,
					#[codec(index = 6)]
					ClientStateNotFound,
					#[codec(index = 7)]
					ConnectionNotFound,
					#[codec(index = 8)]
					PacketCommitmentNotFound,
					#[codec(index = 9)]
					PacketReceiptNotFound,
					#[codec(index = 10)]
					PacketAcknowledgmentNotFound,
					#[codec(index = 11)]
					SendPacketError,
					#[codec(index = 12)]
					InvalidChannelId,
					#[codec(index = 13)]
					InvalidPortId,
					#[codec(index = 14)]
					Other,
					#[codec(index = 15)]
					InvalidRoute,
					#[codec(index = 16)]
					InvalidMessageType,
					#[codec(index = 17)]
					TransferInternals,
					#[codec(index = 18)]
					TransferSerde,
					#[codec(index = 19)]
					TransferOther,
					#[codec(index = 20)]
					TransferProtocol,
					#[codec(index = 21)]
					TransferSend,
					#[codec(index = 22)]
					Utf8Error,
					#[codec(index = 23)]
					InvalidAssetId,
					#[codec(index = 24)]
					PrefixedDenomParse,
					#[codec(index = 25)]
					InvalidAmount,
					#[codec(index = 26)]
					InvalidTimestamp,
					#[codec(index = 27)]
					FailedToGetRevisionNumber,
					#[codec(index = 28)]
					InvalidParams,
					#[codec(index = 29)]
					ChannelInitError,
					#[codec(index = 30)]
					TimestampAndHeightNotFound,
					#[codec(index = 31)]
					ChannelEscrowAddress,
					#[codec(index = 32)]
					WriteAckError,
					#[codec(index = 33)]
					ClientUpdateNotFound,
					#[codec(index = 34)]
					ClientFreezeFailed,
					#[codec(index = 35)]
					AccessDenied,
					#[codec(index = 36)]
					RateLimiter,
					#[codec(index = 37)]
					FailedSendFeeToAccount,
					#[codec(index = 38)]
					OriginAddress,
					#[codec(index = 39)]
					InvalidMemo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Events {
						events: ::std::vec::Vec<
							::core::result::Result<
								runtime_types::pallet_ibc::events::IbcEvent,
								runtime_types::pallet_ibc::errors::IbcError,
							>,
						>,
					},
					#[codec(index = 1)]
					TokenTransferInitiated {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					ParamsUpdated {
						send_enabled: ::core::primitive::bool,
						receive_enabled: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					TokenTransferCompleted {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					TokenReceived {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_receiver_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					TokenTransferFailed {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					TokenTransferTimeout {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 8)]
					OnRecvPacketError { msg: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 9)]
					ClientUpgradeSet,
					#[codec(index = 10)]
					ClientFrozen {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 11)]
					AssetAdminUpdated { admin_account: ::subxt::utils::AccountId32 },
					#[codec(index = 12)]
					FeeLessChannelIdsAdded {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					FeeLessChannelIdsRemoved {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 14)]
					ChargingFeeOnTransferInitiated {
						sequence: ::core::primitive::u64,
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_flat_fee: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 15)]
					ChargingFeeConfirmed { sequence: ::core::primitive::u64 },
					#[codec(index = 16)]
					ChargingFeeTimeout { sequence: ::core::primitive::u64 },
					#[codec(index = 17)]
					ChargingFeeFailedAcknowledgement { sequence: ::core::primitive::u64 },
					#[codec(index = 18)]
					ChildStateUpdated,
					#[codec(index = 19)]
					ClientStateSubstituted {
						client_id: ::std::string::String,
						height: runtime_types::ibc::core::ics02_client::height::Height,
					},
					#[codec(index = 20)]
					ExecuteMemoStarted {
						account_id: ::subxt::utils::AccountId32,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 21)]
					ExecuteMemoIbcTokenTransferSuccess {
						from: ::subxt::utils::AccountId32,
						to: ::std::vec::Vec<::core::primitive::u8>,
						asset_id: ::core::primitive::u128,
						amount: ::core::primitive::u128,
						channel: ::core::primitive::u64,
						next_memo:
							::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
					},
					#[codec(index = 22)]
					ExecuteMemoIbcTokenTransferFailedWithReason {
						from: ::subxt::utils::AccountId32,
						memo: ::std::string::String,
						reason: ::core::primitive::u8,
					},
					#[codec(index = 23)]
					ExecuteMemoIbcTokenTransferFailed {
						from: ::subxt::utils::AccountId32,
						to: ::std::vec::Vec<::core::primitive::u8>,
						asset_id: ::core::primitive::u128,
						amount: ::core::primitive::u128,
						channel: ::core::primitive::u64,
						next_memo:
							::core::option::Option<runtime_types::parachain_runtime::RawMemo>,
					},
					#[codec(index = 24)]
					ExecuteMemoXcmSuccess {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: ::core::primitive::u128,
						para_id: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 25)]
					ExecuteMemoXcmFailed {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: ::core::primitive::u128,
						para_id: ::core::option::Option<::core::primitive::u32>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Any {
				pub type_url: ::std::string::String,
				pub value: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum LightClientProtocol {
				#[codec(index = 0)]
				Beefy,
				#[codec(index = 1)]
				Grandpa,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiAddress<_0> {
				#[codec(index = 0)]
				Id(_0),
				#[codec(index = 1)]
				Raw(::std::vec::Vec<::core::primitive::u8>),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferParams<_0> {
				pub to: runtime_types::pallet_ibc::MultiAddress<_0>,
				pub source_channel: ::core::primitive::u64,
				pub timeout: runtime_types::ibc_primitives::Timeout,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpgradeParams {
				pub client_state: ::std::vec::Vec<::core::primitive::u8>,
				pub consensus_state: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod pallet_ibc_ping {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					send_ping { params: runtime_types::pallet_ibc_ping::SendPingParams },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidParams,
					#[codec(index = 1)]
					ChannelInitError,
					#[codec(index = 2)]
					PacketSendError,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					PacketSent,
					#[codec(index = 1)]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SendPingParams {
				pub data: ::std::vec::Vec<::core::primitive::u8>,
				pub timeout_height_offset: ::core::primitive::u64,
				pub timeout_timestamp_offset: ::core::primitive::u64,
				pub channel_id: ::core::primitive::u64,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_keys {
						keys: runtime_types::parachain_runtime::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidProof,
					#[codec(index = 1)]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					DuplicatedKey,
					#[codec(index = 3)]
					NoKeys,
					#[codec(index = 4)]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					sudo { call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall> },
					#[codec(index = 1)]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					set_key { new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> },
					#[codec(index = 3)]
					sudo_as {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					KeyChanged { old_sudoer: ::core::option::Option<::subxt::utils::AccountId32> },
					#[codec(index = 2)]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 9)]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 10)]
					force_suspension { suspended: ::core::primitive::bool },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					Unreachable,
					#[codec(index = 1)]
					SendFailure,
					#[codec(index = 2)]
					Filtered,
					#[codec(index = 3)]
					UnweighableMessage,
					#[codec(index = 4)]
					DestinationNotInvertible,
					#[codec(index = 5)]
					Empty,
					#[codec(index = 6)]
					CannotReanchor,
					#[codec(index = 7)]
					TooManyAssets,
					#[codec(index = 8)]
					InvalidOrigin,
					#[codec(index = 9)]
					BadVersion,
					#[codec(index = 10)]
					BadLocation,
					#[codec(index = 11)]
					NoSubscription,
					#[codec(index = 12)]
					AlreadySubscribed,
					#[codec(index = 13)]
					InvalidAsset,
					#[codec(index = 14)]
					LowBalance,
					#[codec(index = 15)]
					TooManyLocks,
					#[codec(index = 16)]
					AccountNotSovereign,
					#[codec(index = 17)]
					FeesNotMet,
					#[codec(index = 18)]
					LockNotFound,
					#[codec(index = 19)]
					InUse,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Attempted(runtime_types::xcm::v3::traits::Outcome),
					#[codec(index = 1)]
					Sent(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::Xcm,
					),
					#[codec(index = 2)]
					UnexpectedResponse(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v3::Response),
					#[codec(index = 4)]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 6)]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					InvalidResponder(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					InvalidResponderVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					AssetsTrapped(
						::subxt::utils::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					VersionChangeNotified(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 13)]
					SupportedVersionChanged(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					NotifyTargetSendFail(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::traits::Error,
					),
					#[codec(index = 15)]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 16)]
					InvalidQuerierVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 17)]
					InvalidQuerier(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 18)]
					VersionNotifyStarted(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 19)]
					VersionNotifyRequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 20)]
					VersionNotifyUnrequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 21)]
					FeesPaid(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 22)]
					AssetsClaimed(
						::subxt::utils::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedMultiLocation,
						maybe_match_querier:
							::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
						maybe_notify:
							::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedMultiLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoteLockedFungibleRecord<_0> {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedMultiLocation,
					pub locker: runtime_types::xcm::VersionedMultiLocation,
					pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						_0,
						::core::primitive::u128,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum VersionMigrationStage {
					#[codec(index = 0)]
					MigrateSupportedVersion,
					#[codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					),
					#[codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
		}
		pub mod parachain_info {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {}
			}
		}
		pub mod parachain_runtime {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RawMemo(pub ::std::string::String);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 2)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 3)]
				ParachainInfo(runtime_types::parachain_info::pallet::Call),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 33)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 35)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 36)]
				IbcPing(runtime_types::pallet_ibc_ping::pallet::Call),
				#[codec(index = 37)]
				Assets(runtime_types::pallet_assets::pallet::Call),
				#[codec(index = 38)]
				AssetRegistry(runtime_types::orml_asset_registry::module::Call),
				#[codec(index = 255)]
				Ibc(runtime_types::pallet_ibc::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 2)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 11)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 33)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 35)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 36)]
				IbcPing(runtime_types::pallet_ibc_ping::pallet::Event),
				#[codec(index = 37)]
				Assets(runtime_types::pallet_assets::pallet::Event),
				#[codec(index = 38)]
				AssetRegistry(runtime_types::orml_asset_registry::module::Event),
				#[codec(index = 255)]
				Ibc(runtime_types::pallet_ibc::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SessionKeys {
				pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum XcmpMessageFormat {
					#[codec(index = 0)]
					ConcatenatedVersionedXcm,
					#[codec(index = 1)]
					ConcatenatedEncodedBlob,
					#[codec(index = 2)]
					Signals,
				}
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v4 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_cooldown: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Perbill(pub ::core::primitive::u32);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod unchecked_extrinsic {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
					);
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DoubleEncoded2 {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v2::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v2::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v2::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v2::BodyId,
							part: runtime_types::xcm::v2::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v2::multiasset::AssetId,
						pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v2::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v2::multiasset::AssetId,
							fun: runtime_types::xcm::v2::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v2::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v2::multilocation::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum BodyId {
					#[codec(index = 0)]
					Unit,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Index(#[codec(compact)] ::core::primitive::u32),
					#[codec(index = 3)]
					Executive,
					#[codec(index = 4)]
					Technical,
					#[codec(index = 5)]
					Legislative,
					#[codec(index = 6)]
					Judicial,
					#[codec(index = 7)]
					Defense,
					#[codec(index = 8)]
					Administration,
					#[codec(index = 9)]
					Treasury,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum BodyPart {
					#[codec(index = 0)]
					Voice,
					#[codec(index = 1)]
					Members {
						#[codec(compact)]
						count: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Fraction {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					AtLeastProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					MoreThanProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum NetworkId {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Polkadot,
					#[codec(index = 3)]
					Kusama,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction2>);
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
						#[codec(index = 7)]
						Defense,
						#[codec(index = 8)]
						Administration,
						#[codec(index = 9)]
						Treasury,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v3::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v3::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v3::junctions::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						UnhandledXcmVersion,
						#[codec(index = 36)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 37)]
						Barrier,
						#[codec(index = 38)]
						WeightNotComputable,
						#[codec(index = 39)]
						ExceedsStackLimit,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 1)]
						Incomplete(
							runtime_types::sp_weights::weight_v2::Weight,
							runtime_types::xcm::v3::traits::Error,
						),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v3::traits::Error),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum MaybeErrorCode {
					#[codec(index = 0)]
					Success,
					#[codec(index = 1)]
					Error(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::xcm::v3::multilocation::MultiLocation,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::xcm::v3::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(runtime_types::sp_weights::weight_v2::Weight),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction2>);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedAssetId {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::AssetId),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedMultiAssets {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAssets),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAssets),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedMultiLocation {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multilocation::MultiLocation),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multilocation::MultiLocation),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedResponse {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Response),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedXcm {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedXcm2 {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm2),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm2),
			}
		}
	}
}
