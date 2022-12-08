use super::*;
use crate::{self as pallet_evm_permission};

use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU16, ConstU64},
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};
pub use frame_support::{traits::FindAuthor, ConsensusEngineId};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	Perbill,
};

use frame_system::EnsureRoot;
use pallet_evm::{EnsureAddressNever, EnsureAddressRoot, FeeCalculator};
use std::str::FromStr;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use account::EthereumSignature;
type Signature = EthereumSignature;
pub type MyAccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Balance = u128;

pub mod currency {
	use super::*;

	pub const SUPPLY_FACTOR: Balance = 100;
	pub const GIGAWEI: Balance = 1_000_000_000;
}

use currency::*;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Ethereum: pallet_ethereum,
		EVM: pallet_evm::{Pallet, Storage, Event<T>, Config},
		EVMPermission: pallet_evm_permission,
		Sudo: pallet_sudo,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = MyAccountId;
	type Lookup = IdentityLookup<MyAccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_sudo::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

parameter_types! {
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 4];
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<0>;
	type AccountStore = frame_system::Pallet<Test>;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

/// And ipmlementation of Frontier's AddressMapping trait for Moonbeam Accounts.
/// This is basically identical to Frontier's own IdentityAddressMapping, but it works for any type
/// that is Into<H160> like AccountId20 for example.
pub struct IntoAddressMapping;

impl<T: From<H160>> pallet_evm::AddressMapping<T> for IntoAddressMapping {
	fn into_account_id(address: H160) -> T {
		address.into()
	}
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND.ref_time() / GAS_PER_SECOND;

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (U256, Weight) {
		((1 * GIGAWEI * SUPPLY_FACTOR).into(), Weight::zero())
	}
}

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_mul(2);

parameter_types! {
	pub const ChainId: u64 = 1281;
	pub BlockGasLimit: U256
		= U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
	pub WeightPerGas: Weight = Weight::from_ref_time(WEIGHT_PER_GAS);
}

impl pallet_evm::Config for Test {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;

	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressRoot<MyAccountId>;
	type WithdrawOrigin = EnsureAddressNever<MyAccountId>;
	type AddressMapping = IntoAddressMapping;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type ChainId = ChainId;
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = pallet_evm::EVMCurrencyAdapter<Balances, ()>;
	type FindAuthor = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 6000 / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pallet_ethereum::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
}

impl pallet_evm_permission::Config for Test {
	type CreateOrigin = EnsureRoot<MyAccountId>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		// Create the block author account with some balance.
		balances: vec![(
			MyAccountId::from_str("0x1234500000000000000000000000000000000000").unwrap(),
			12345,
		)],
	}
	.assimilate_storage(&mut t)
	.expect("Pallet balances storage can be assimilated");

	t.into()
}
