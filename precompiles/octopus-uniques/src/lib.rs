// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Precompile to interact with pallet uniques through an evm precompile.

#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	inherent::Vec,
	sp_runtime::traits::{CheckedConversion, StaticLookup},
	traits::ConstU32,
};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_core::{H160, H256, U256};
use sp_std::marker::PhantomData;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

type GetMetadataSizeLimit = ConstU32<20480>;

/// A precompile to wrap the functionality from pallet uniques.
pub struct OctopusUniquesPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
#[precompile::test_concrete_types(mock::Test)]
impl<Runtime> OctopusUniquesPrecompile<Runtime>
where
	Runtime: pallet_uniques::Config<pallet_uniques::Instance1>
		+ pallet_evm::Config
		+ frame_system::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<pallet_uniques::Call<Runtime, pallet_uniques::Instance1>>,
	Runtime::Hash: From<H256>,
	<Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::CollectionId:
		TryFrom<U256> + Into<U256> + EvmData,
	<Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::ItemId:
		TryFrom<U256> + Into<U256> + EvmData,
{
	// The dispatchable wrappers are next. They dispatch a Substrate inner Call.
	#[precompile::public("createCollection(uint256,address)")]
	#[precompile::public("create_collection(uint256,address)")]
	fn create_collection(
		handle: &mut impl PrecompileHandle,
		collection: U256,
		admin: Address,
	) -> EvmResult<bool> {
		let collection:<Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::CollectionId  = collection.checked_into().unwrap();
		let admin: H160 = admin.into();
		let admin = Runtime::Lookup::unlookup(Runtime::AddressMapping::into_account_id(admin));

		log::trace!(
			target: "octopus-uniques-precompile",
			"create collection in uniques, collection: {:?}, admin: {:?} ", collection, admin,
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_uniques::Call::<Runtime, pallet_uniques::Instance1>::create {
			collection,
			admin,
		};

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;
		Ok(true)
	}

	#[precompile::public("mint(uint256,uint256,address)")]
	fn mint(
		handle: &mut impl PrecompileHandle,
		collection: U256,
		item: U256,
		owner: Address,
	) -> EvmResult<bool> {
		let collection: <Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::CollectionId  = collection.checked_into().unwrap();
		let item: <Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::ItemId =
			item.checked_into().unwrap();
		let owner: H160 = owner.into();
		let owner = Runtime::Lookup::unlookup(Runtime::AddressMapping::into_account_id(owner));

		log::trace!(
			target: "octopus-uniques-precompile",
			"mint in uniques, collection: {:?}, item: {:?}, owner: {:?} ", collection, item, owner,
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_uniques::Call::<Runtime, pallet_uniques::Instance1>::mint {
			collection,
			item,
			owner,
		};

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;
		Ok(true)
	}

	#[precompile::public("set_metadata(uint256,uint256,bytes)")]
	#[precompile::public("setMetadata(uint256,uint256,bytes)")]
	fn set_metadata(
		handle: &mut impl PrecompileHandle,
		collection: U256,
		item: U256,
		data: BoundedBytes<GetMetadataSizeLimit>,
	) -> EvmResult<bool> {
		let collection: <Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::CollectionId  = collection.checked_into().unwrap();
		let item: <Runtime as pallet_uniques::Config<pallet_uniques::Instance1>>::ItemId =
			item.checked_into().unwrap();
		let data: Vec<u8> = data.into();

		log::trace!(
			target: "octopus-uniques-precompile",
			"set metadata in uniques, collection: {:?}, item: {:?}, data: {:?} ", collection, item, data.clone(),
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_uniques::Call::<Runtime, pallet_uniques::Instance1>::set_metadata {
			collection,
			item,
			data: data.try_into().unwrap(),
			is_frozen: false,
		};

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;
		Ok(true)
	}
}
