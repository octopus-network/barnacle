//! # EVM Permission Pallet
//!
//! ## Overview
//!
//! Once Appchain enable onchain governance, and remove sudo account,
//! Foundation or Council have permission to deploy contracts.

#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
pub use pallet_evm;

use sp_core::{H160, H256, U256};
use sp_std::vec::Vec;

use pallet_evm::{BalanceOf, GasWeightMapping};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm::Config {
		type CreateOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// wrapper withdraw
		/// Withdraw balance from EVM into currency/balances pallet.
		#[pallet::weight(0)]
		pub fn withdraw(
			origin: OriginFor<T>,
			address: H160,
			value: BalanceOf<T>,
		) -> DispatchResult {
			// It can be used to verify.
			// T::CreateOrigin::ensure_origin(origin.clone())?;
			pallet_evm::Pallet::<T>::withdraw(origin, address, value)
		}

		/// wrapper call
		/// Issue an EVM call operation. This is similar to a message call transaction in Ethereum.
		#[pallet::weight({
			let without_base_extrinsic_weight = true;
			T::GasWeightMapping::gas_to_weight(*gas_limit, without_base_extrinsic_weight)
		})]
		pub fn call(
			origin: OriginFor<T>,
			source: H160,
			target: H160,
			input: Vec<u8>,
			value: U256,
			gas_limit: u64,
			max_fee_per_gas: U256,
			max_priority_fee_per_gas: Option<U256>,
			nonce: Option<U256>,
			access_list: Vec<(H160, Vec<H256>)>,
		) -> DispatchResultWithPostInfo {
			pallet_evm::Pallet::<T>::call(
				origin,
				source,
				target,
				input,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			)
		}

		/// wrapper create
		/// Issue an EVM create operation. This is similar to a contract creation transaction in
		/// Ethereum.
		#[pallet::weight({
			let without_base_extrinsic_weight = true;
			T::GasWeightMapping::gas_to_weight(*gas_limit, without_base_extrinsic_weight)
		})]
		pub fn create(
			origin: OriginFor<T>,
			source: H160,
			init: Vec<u8>,
			value: U256,
			gas_limit: u64,
			max_fee_per_gas: U256,
			max_priority_fee_per_gas: Option<U256>,
			nonce: Option<U256>,
			access_list: Vec<(H160, Vec<H256>)>,
		) -> DispatchResultWithPostInfo {
			// Ensure that the origin has permission.
			T::CreateOrigin::ensure_origin(origin.clone())?;
			pallet_evm::Pallet::<T>::create(
				origin,
				source,
				init,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			)
		}

		/// wrapper create2
		/// Issue an EVM create2 operation.
		#[pallet::weight({
			let without_base_extrinsic_weight = true;
			T::GasWeightMapping::gas_to_weight(*gas_limit, without_base_extrinsic_weight)
		})]
		pub fn create2(
			origin: OriginFor<T>,
			source: H160,
			init: Vec<u8>,
			salt: H256,
			value: U256,
			gas_limit: u64,
			max_fee_per_gas: U256,
			max_priority_fee_per_gas: Option<U256>,
			nonce: Option<U256>,
			access_list: Vec<(H160, Vec<H256>)>,
		) -> DispatchResultWithPostInfo {
			// Ensure that the origin has permission.
			T::CreateOrigin::ensure_origin(origin.clone())?;
			pallet_evm::Pallet::<T>::create2(
				origin,
				source,
				init,
				salt,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			)
		}
	}
}
