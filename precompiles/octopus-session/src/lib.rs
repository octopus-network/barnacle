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

//! Precompile to interact with pallet author mapping through an evm precompile.

#![cfg_attr(not(feature = "std"), no_std)]
// #![feature(assert_matches)]

use fp_evm::PrecompileHandle;
use frame_support::traits::ConstU32;
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	// traits::Get,
	inherent::Vec,
};
use pallet_evm::AddressMapping;
use pallet_session::Call as OctopusSessionCall;
use precompile_utils::prelude::*;
use sp_core::{crypto::UncheckedFrom, Decode, H256};
use sp_std::marker::PhantomData;

type GetEncodedProposalSizeLimit = ConstU32<2048>; //Andy: Need check

/// A precompile to wrap the functionality from pallet session.
pub struct OctopusSessionPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
#[precompile::test_concrete_types(mock::Runtime)]
impl<Runtime> OctopusSessionPrecompile<Runtime>
where
	Runtime: pallet_session::Config + pallet_evm::Config + frame_system::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<OctopusSessionCall<Runtime>>,
	Runtime::Hash: From<H256>,
{
	// The dispatchable wrappers are next. They dispatch a Substrate inner Call.
	#[precompile::public("setKeys(bytes,bytes)")]
	#[precompile::public("set_keys(bytes,bytes)")]
	fn set_keys(
		handle: &mut impl PrecompileHandle,
		keys: BoundedBytes<GetEncodedProposalSizeLimit>,
		proof: BoundedBytes<GetEncodedProposalSizeLimit>,
	) -> EvmResult {
		let keys: Vec<u8> = keys.into();
		let proof: Vec<u8> = proof.into();

		log::trace!(
			target: "session-precompile",
			"set_keys with keys {:?}, and proof {:?}",
			keys,
			proof,
		);

		let keys = <Runtime as pallet_session::Config>::Keys::decode(&mut keys.as_slice())
			.map_err(|_| revert("decode keys error"))?;
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = OctopusSessionCall::<Runtime>::set_keys { keys, proof };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}
