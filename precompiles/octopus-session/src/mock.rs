use super::*;

use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU64},
	weights::{constants::WEIGHT_PER_SECOND, Weight},
	PalletId,
};
pub use frame_support::{traits::FindAuthor, ConsensusEngineId};
use frame_system as system;
use sp_core::{H160, H256, U256};

use sp_runtime::{
	impl_opaque_keys,
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup,
		Keccak256, OpaqueKeys, Verify,
	},
	Perbill,
};

use frame_system::{EnsureRoot, EnsureSigned};
use pallet_evm::{EnsureAddressNever, EnsureAddressRoot, FeeCalculator, Precompile, PrecompileSet};
use std::str::FromStr;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use account::EthereumSignature;
type Signature = EthereumSignature;
pub type MyAccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Balance = u128;

pub const PRECOMPILE_ADDRESS: u64 = 1;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,

		OctopusAppchain: pallet_octopus_appchain,
		OctopusBridge: pallet_octopus_bridge,
		OctopusLpos: pallet_octopus_lpos,
		OctopusUpwardMessages: pallet_octopus_upward_messages,
		OctopusAssets: pallet_assets,
		OctopusUniques: pallet_uniques,
		Session: pallet_session,

		Ethereum: pallet_ethereum,
		EVM: pallet_evm,
		// EVMPermission: pallet_evm_permission,
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

// use pallet_octopus_appchain::AuthorityId as OctopusId;
impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub octopus: OctopusAppchain,
	}
}

pub struct MockSessionManager;

impl pallet_session::SessionManager<AccountId> for MockSessionManager {
	fn end_session(_: sp_staking::SessionIndex) {}
	fn start_session(index: sp_staking::SessionIndex) {
		OctopusLpos::start_session(index);
	}
	fn new_session(_: sp_staking::SessionIndex) -> Option<Vec<AccountId>> {
		None
	}
}

parameter_types! {
	pub const Period: u32 = 1;
	pub const Offset: u32 = 0;
}
impl pallet_session::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = MockSessionManager;
	type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = MockSessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Test>;
}

impl pallet_session::historical::Config for Test {
	type FullIdentification = u128;
	type FullIdentificationOf = pallet_octopus_lpos::ExposureOf<Test>;
}

pub(crate) type Extrinsic = TestXt<RuntimeCall, ()>;
pub(crate) type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(RuntimeCall, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

const SUPPLY_FACTOR: Balance = 100;
const _WEI: Balance = 1;
const _KILOWEI: Balance = 1_000;
const _MEGAWEI: Balance = 1_000_000;
const GIGAWEI: Balance = 1_000_000_000;
const MICROEBAR: Balance = 1_000_000_000_000;
const MILLIEBAR: Balance = 1_000_000_000_000_000;
const EBAR: Balance = 1_000_000_000_000_000_000;
const _KILOEBAR: Balance = 1_000_000_000_000_000_000_000;
const _TRANSACTION_BYTE_FEE: Balance = 10 * MICROEBAR * SUPPLY_FACTOR;
const STORAGE_BYTE_FEE: Balance = 100 * MICROEBAR * SUPPLY_FACTOR;
const _WEIGHT_FEE: Balance = 100 * _KILOWEI * SUPPLY_FACTOR;

const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 100 * MILLIEBAR * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
}

parameter_types! {
	pub const AssetDeposit: Balance = 100 * EBAR * SUPPLY_FACTOR;
	pub const ApprovalDeposit: Balance = 0;
	pub const AssetsStringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = deposit(1,68);
	pub const MetadataDepositPerByte: Balance = deposit(0, 1);
}
pub type AssetId = u32;
pub type AssetBalance = u128;

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = AssetBalance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<{ deposit(1, 18) }>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = AssetsStringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Test>;
}

pub struct OctopusAppCrypto;

impl frame_system::offchain::AppCrypto<<Signature as Verify>::Signer, Signature>
	for OctopusAppCrypto
{
	type RuntimeAppPublic = pallet_octopus_appchain::ecdsa::AuthorityId;
	type GenericSignature = sp_core::ecdsa::Signature;
	type GenericPublic = sp_core::ecdsa::Public;
}

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	pub const BondingDuration: pallet_octopus_lpos::EraIndex = 24 * 28;
	pub const BlocksPerEra: u32 = 20;
}

impl pallet_octopus_lpos::Config for Test {
	type Currency = Balances;
	type UnixTime = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SessionInterface = Self;
	type AppchainInterface = OctopusAppchain;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type PalletId = OctopusAppchainPalletId;
	type WeightInfo = pallet_octopus_lpos::weights::SubstrateWeight<Test>;
}

parameter_types! {
	pub const MaxMessagePayloadSize: u32 = 256;
	pub const MaxMessagesPerCommit: u32 = 20;
}
impl pallet_octopus_upward_messages::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Hashing = Keccak256;
	type MaxMessagePayloadSize = MaxMessagePayloadSize;
	type MaxMessagesPerCommit = MaxMessagesPerCommit;
	type WeightInfo = pallet_octopus_upward_messages::weights::SubstrateWeight<Test>;
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

type CollectionId = u128;
type ItemId = u128;
parameter_types! {
	pub const CollectionDeposit: Balance = 100 * EBAR * SUPPLY_FACTOR;
	pub const ItemDeposit: Balance = 1 * EBAR * SUPPLY_FACTOR;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
	pub const UniqueStringLimit: u32 = 2048;
}
impl pallet_uniques::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = CollectionId;
	type ItemId = ItemId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = UniqueStringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}

parameter_types! {
	   pub const OctopusAppchainPalletId: PalletId = PalletId(*b"py/octps");
	   pub const GracePeriod: u32 = 10;
	   pub const UnsignedPriority: u64 = 1 << 21;
	   pub const RequestEventLimit: u32 = 10;
	   pub const UpwardMessagesLimit: u32 = 10;
	   pub const NativeTokenDecimals: u128 = 1_000_000_000_000_000_000;
	   pub const MaxValidators: u32 = 100;
	   pub const Th: u64 = 300;
}

impl pallet_octopus_bridge::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = OctopusAppchainPalletId;
	type Currency = Balances;
	type AppchainInterface = OctopusAppchain;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type AssetIdByTokenId = OctopusBridge;
	type AssetId = AssetId;
	type AssetBalance = AssetBalance;
	type Fungibles = OctopusAssets;
	type CollectionId = u128;
	type ItemId = u128;
	type Nonfungibles = pallet_octopus_bridge::impls::UnImplementUniques<Test>;
	type Convertor = pallet_octopus_bridge::impls::ExampleConvertor<Test>;
	type NativeTokenDecimals = NativeTokenDecimals;
	type Threshold = Th;
	type WeightInfo = ();
}

impl pallet_octopus_appchain::Config for Test {
	type AuthorityId = pallet_octopus_appchain::sr25519::AuthorityId;
	type AppCrypto = OctopusAppCrypto;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type BridgeInterface = OctopusBridge;
	type LposInterface = OctopusLpos;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type GracePeriod = GracePeriod;
	type UnsignedPriority = UnsignedPriority;
	type RequestEventLimit = RequestEventLimit;
	type MaxValidators = MaxValidators;
	type WeightInfo = ();
}

pub fn _new_test_ext() -> sp_io::TestExternalities {
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

pub fn precompile_address() -> H160 {
	H160::from_low_u64_be(1)
}

#[derive(Default)]
pub struct Precompiles<R>(PhantomData<R>);

impl<R> PrecompileSet for Precompiles<R>
where
	OctopusSessionPrecompile<R>: Precompile,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<EvmResult<PrecompileOutput>> {
		match handle.code_address() {
			a if a == hash(PRECOMPILE_ADDRESS) =>
				Some(OctopusSessionPrecompile::<R>::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		address == hash(PRECOMPILE_ADDRESS)
	}
}

pub type PCall = OctopusSessionPrecompileCall<Test>;

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

pub(crate) struct ExtBuilder {
	// endowed accounts with balances
	balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> ExtBuilder {
		ExtBuilder { balances: vec![] }
	}
}

impl ExtBuilder {
	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.expect("Frame system builds valid default genesis config");

		pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.expect("Pallet balances storage can be assimilated");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub(crate) fn _events() -> Vec<RuntimeEvent> {
	System::events().into_iter().map(|r| r.event).collect::<Vec<_>>()
}
