use super::*;
use pallet_octopus_appchain::traits_default_impl::ExampleConvertor;
use sp_runtime::{
	generic, impl_opaque_keys,
	testing::TestXt,
	traits::{
		AccountIdLookup, BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount,
		OpaqueKeys, Verify,
	},
};

use pallet_evm::{EnsureAddressNever, EnsureAddressRoot, PrecompileSet};
use sp_core::H160;

pub use frame_support::{
	construct_runtime,
	pallet_prelude::GenesisBuild,
	parameter_types,
	traits::{
		ConstU128, ConstU32, Hooks, KeyOwnerProofSystem, OnFinalize, OnInitialize, Randomness,
		StorageInfo,
	},
	weights::{IdentityFee, Weight},
	PalletId, StorageValue,
};

use account::EthereumSignature;
use frame_system::EnsureRoot;

pub(crate) type BlockNumber = u32;
pub type Signature = EthereumSignature;
pub type Balance = u128;
pub type Moment = u64;
pub type Index = u64;
pub type Hash = sp_core::H256;

pub const MILLICENTS: Balance = 10_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;
pub const MILLISECS_PER_BLOCK: Moment = 3000;
pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 1 * MINUTES;
pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const SS58Prefix: u16 = 42;
}
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1 * DOLLARS;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}
impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

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
	type Event = Event;
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

pub(crate) type Extrinsic = TestXt<Call, ()>;
pub(crate) type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

parameter_types! {
	pub const AssetDeposit: Balance = 100 * DOLLARS;
	pub const ApprovalDeposit: Balance = 1 * DOLLARS;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 * DOLLARS;
	pub const MetadataDepositPerByte: Balance = 1 * DOLLARS;
}
pub type AssetId = u32;
pub type AssetBalance = u128;

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = AssetBalance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<DOLLARS>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
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
	pub const BlocksPerEra: u32 = EPOCH_DURATION_IN_BLOCKS * 6 / (SECS_PER_BLOCK as u32);
}

impl pallet_octopus_lpos::Config for Test {
	type Currency = Balances;
	type UnixTime = Timestamp;
	type Event = Event;
	type Reward = (); // rewards are minted from the void
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type BlocksPerEra = BlocksPerEra;
	type SessionInterface = Self;
	type AppchainInterface = OctopusAppchain;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type PalletId = OctopusAppchainPalletId;
	type ValidatorsProvider = OctopusAppchain;
	type WeightInfo = pallet_octopus_lpos::weights::SubstrateWeight<Test>;
}

impl pallet_octopus_upward_messages::Config for Test {
	type Event = Event;
	type Call = Call;
	type UpwardMessagesLimit = UpwardMessagesLimit;
	type WeightInfo = pallet_octopus_upward_messages::weights::SubstrateWeight<Test>;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		OctopusAppchain: pallet_octopus_appchain,
		OctopusLpos: pallet_octopus_lpos,
		OctopusUpwardMessages: pallet_octopus_upward_messages,
		Session: pallet_session,
		Assets: pallet_assets,
		Uniques: pallet_uniques,
		Evm: pallet_evm,
	}
);

type ClassId = u128;
type InstanceId = u128;
parameter_types! {
	pub const ClassDeposit: Balance = 100 * DOLLARS;
	pub const InstanceDeposit: Balance = 1 * DOLLARS;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
}
impl pallet_uniques::Config for Test {
	type Event = Event;
	type ClassId = ClassId;
	type InstanceId = InstanceId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type ClassDeposit = ClassDeposit;
	type InstanceDeposit = InstanceDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
}

parameter_types! {
	   pub const OctopusAppchainPalletId: PalletId = PalletId(*b"py/octps");
	   pub const GracePeriod: u32 = 10;
	   pub const UnsignedPriority: u64 = 1 << 21;
	   pub const RequestEventLimit: u32 = 10;
	   pub const UpwardMessagesLimit: u32 = 10;
}

impl pallet_octopus_appchain::Config for Test {
	type AssetId = AssetId;
	type AssetBalance = AssetBalance;
	type AssetIdByTokenId = OctopusAppchain;
	type AuthorityId = pallet_octopus_appchain::ecdsa::AuthorityId;
	type AppCrypto = OctopusAppCrypto;
	type Event = Event;
	type Call = Call;
	type PalletId = OctopusAppchainPalletId;
	type LposInterface = OctopusLpos;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type ClassId = ClassId;
	type InstanceId = InstanceId;
	type Uniques = Uniques;
	type Convertor = ExampleConvertor<Test>;
	type Currency = Balances;
	type Assets = Assets;
	type GracePeriod = GracePeriod;
	type UnsignedPriority = UnsignedPriority;
	type RequestEventLimit = RequestEventLimit;
	type WeightInfo = ();
}

parameter_types! {
	pub const PrecompilesValue: Precompiles<Test> = Precompiles(PhantomData);
}

pub struct IntoAddressMapping;

impl<T: From<H160>> pallet_evm::AddressMapping<T> for IntoAddressMapping {
	fn into_account_id(address: H160) -> T {
		address.into()
	}
}

impl pallet_evm::Config for Test {
	type FeeCalculator = ();
	type GasWeightMapping = ();
	type CallOrigin = EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = EnsureAddressNever<AccountId>;
	type AddressMapping = IntoAddressMapping;
	type Currency = Balances;
	type Event = Event;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = Precompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ();
	type OnChargeTransaction = ();
	type BlockGasLimit = ();
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type FindAuthor = ();
	type WeightInfo = ();
}

pub const PRECOMPILE_ADDRESS: u64 = 1;

#[derive(Default)]
pub struct Precompiles<R>(PhantomData<R>);

impl<R> PrecompileSet for Precompiles<R>
where
	OctopusSessionWrapper<R>: Precompile,
{
	fn execute(
		&self,
		address: H160,
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
		is_static: bool,
	) -> Option<EvmResult<PrecompileOutput>> {
		match address {
			a if a == hash(PRECOMPILE_ADDRESS) =>
				Some(OctopusSessionWrapper::<R>::execute(input, target_gas, context, is_static)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		address == hash(PRECOMPILE_ADDRESS)
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

pub(crate) struct ExtBuilder {
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

pub fn precompile_address() -> H160 {
	H160::from_low_u64_be(1)
}
