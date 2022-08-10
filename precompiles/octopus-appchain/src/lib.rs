#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{Context, ExitSucceed, PrecompileOutput};
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	traits::Currency,
};
use pallet_evm::{AddressMapping, Precompile};
use pallet_octopus_appchain::Call as OctopusAppchainCall;
use precompile_utils::{
	Bytes, EvmData, EvmDataReader, EvmResult, FunctionModifier, Gasometer, RuntimeHelper,
};
use sp_core::H256;
use sp_std::{fmt::Debug, marker::PhantomData};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<Runtime> = <<Runtime as pallet_octopus_appchain::Config>::Currency as Currency<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	Lock = "lock(uint256,bytes)",
	BurnAsset = "burn_asset(uint32,uint128,bytes)",
	LockNft = "lock_nft(uint128,uint128,bytes)",
}

/// A precompile to wrap the functionality from pallet octopus-appchain.
pub struct OctopusAppchainWrapper<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for OctopusAppchainWrapper<Runtime>
where
	Runtime: pallet_octopus_appchain::Config + pallet_evm::Config + frame_system::Config,
	<Runtime as frame_system::Config>::Call:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<Runtime as frame_system::Config>::Call as Dispatchable>::Origin:
		From<Option<Runtime::AccountId>>,
	<Runtime as frame_system::Config>::Call: From<OctopusAppchainCall<Runtime>>,
	Runtime::Hash: From<H256>,
	BalanceOf<Runtime>: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::AssetId: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::AssetBalance: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::ClassId: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::InstanceId: EvmData,
{
	fn execute(
		input: &[u8], //Reminder this is big-endian
		target_gas: Option<u64>,
		context: &Context,
		is_static: bool,
	) -> EvmResult<PrecompileOutput> {
		log::trace!(target: "octopus-appchain-precompile", "In octopus-appchain wrapper");

		let mut gasometer = Gasometer::new(target_gas);
		let gasometer = &mut gasometer;

		let (mut input, selector) = EvmDataReader::new_with_selector(gasometer, input)?;
		let input = &mut input;

		gasometer.check_function_modifier(context, is_static, FunctionModifier::NonPayable)?;

		match selector {
			// Dispatchables
			Action::Lock => Self::lock(input, gasometer, context),
			Action::BurnAsset => Self::burn_asset(input, gasometer, context),
			Action::LockNft => Self::lock_nft(input, gasometer, context),
		}
	}
}

impl<Runtime> OctopusAppchainWrapper<Runtime>
where
	Runtime: pallet_octopus_appchain::Config + pallet_evm::Config + frame_system::Config,
	<Runtime as frame_system::Config>::Call:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<Runtime as frame_system::Config>::Call as Dispatchable>::Origin:
		From<Option<Runtime::AccountId>>,
	<Runtime as frame_system::Config>::Call: From<OctopusAppchainCall<Runtime>>,
	Runtime::Hash: From<H256>,
	BalanceOf<Runtime>: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::AssetId: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::AssetBalance: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::ClassId: EvmData,
	<Runtime as pallet_octopus_appchain::Config>::InstanceId: EvmData,
{
	// The dispatchable wrappers are next. They dispatch a Substrate inner Call.
	fn lock(
		input: &mut EvmDataReader,
		gasometer: &mut Gasometer,
		context: &Context,
	) -> EvmResult<PrecompileOutput> {
		// Bound check
		input.expect_arguments(gasometer, 2)?;

		let amount = input.read::<BalanceOf<Runtime>>(gasometer)?;
		let receiver_id = input.read::<Bytes>(gasometer)?;

		log::trace!(
			target: "octopus-appchain-precompile",
			"lock in appchain, receiver_id: {:?}, amount {:?}", receiver_id, amount,
		);

		let origin = Runtime::AddressMapping::into_account_id(context.caller);
		let call = OctopusAppchainCall::<Runtime>::lock { amount, receiver_id: receiver_id.0 };

		RuntimeHelper::<Runtime>::try_dispatch(Some(origin).into(), call, gasometer)?;

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default(),
		})
	}

	fn burn_asset(
		input: &mut EvmDataReader,
		gasometer: &mut Gasometer,
		context: &Context,
	) -> EvmResult<PrecompileOutput> {
		// Bound check
		input.expect_arguments(gasometer, 3)?;

		let asset_id: <Runtime as pallet_octopus_appchain::Config>::AssetId =
			input.read(gasometer)?;
		let amount = input.read(gasometer)?;
		let receiver_id = input.read::<Bytes>(gasometer)?;

		log::trace!(
			target: "octopus-appchain-precompile",
			"burn asset in appchain, asset_id: {:?}, receiver_id: {:?}, amount {:?}", asset_id, receiver_id, amount,
		);

		let origin = Runtime::AddressMapping::into_account_id(context.caller);
		let call = OctopusAppchainCall::<Runtime>::burn_asset {
			asset_id,
			amount,
			receiver_id: receiver_id.0,
		};

		RuntimeHelper::<Runtime>::try_dispatch(Some(origin).into(), call, gasometer)?;

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default(),
		})
	}

	fn lock_nft(
		input: &mut EvmDataReader,
		gasometer: &mut Gasometer,
		context: &Context,
	) -> EvmResult<PrecompileOutput> {
		// Bound check
		input.expect_arguments(gasometer, 3)?;

		let class: <Runtime as pallet_octopus_appchain::Config>::ClassId = input.read(gasometer)?;
		let instance: <Runtime as pallet_octopus_appchain::Config>::InstanceId =
			input.read(gasometer)?;
		let receiver_id = input.read::<Bytes>(gasometer)?;

		log::trace!(
			target: "octopus-appchain-precompile",
			"lock nft in appchain, class_id: {:?}, instance_id: {:?}, receiver_id: {:?}", class, instance, receiver_id,
		);

		let origin = Runtime::AddressMapping::into_account_id(context.caller);
		let call = OctopusAppchainCall::<Runtime>::lock_nft {
			class,
			instance,
			receiver_id: receiver_id.0,
		};

		RuntimeHelper::<Runtime>::try_dispatch(Some(origin).into(), call, gasometer)?;

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default(),
		})
	}
}
