#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::tokens::nonfungibles;
use pallet_octopus_support::{traits::ConvertIntoNep171, types::Nep171TokenMetadata};
use scale_info::prelude::string::{String, ToString};
use serde::Deserialize;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[derive(Deserialize, RuntimeDebug)]
struct Erc721Metadata {
	#[serde(default)]
	uri: String,
}

pub struct Erc721MetadataConvertor<T>(sp_std::marker::PhantomData<T>);
impl<T> ConvertIntoNep171 for Erc721MetadataConvertor<T>
where
	T: pallet_octopus_bridge::Config,
{
	type CollectionId = <T as pallet_octopus_bridge::Config>::CollectionId;
	type ItemId = <T as pallet_octopus_bridge::Config>::ItemId;

	fn convert_into_nep171_metadata(
		collection: Self::CollectionId,
		item: Self::ItemId,
	) -> Option<Nep171TokenMetadata> {
		let mut data: Vec<u8> = Vec::new();
		if let Some(attribute) = <T::Nonfungibles as nonfungibles::Inspect<T::AccountId>>::attribute(
			&collection,
			&item,
			&vec![],
		) {
			data.extend(attribute);
		}

		if data.is_empty() {
			return None
		}

		let uri: Option<String> = String::from_utf8(data).ok();
		log::trace!(target: "Erc721 convertor", "Erc721 metadata is : {:?}", uri.clone().unwrap());

		// parse rmrk base metadata to nep171 format
		let metadata = Nep171TokenMetadata {
			title: Some("A erc721 nft".to_string()),
			media: uri,
			..Default::default()
		};
		log::trace!(target: "Erc721 convertor", "After, the Nep171 media data is {:?} ", metadata.clone());

		Some(metadata)
	}
}
