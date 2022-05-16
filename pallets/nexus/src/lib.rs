#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type Score = u32;

// TODO add Balance
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct RenderingInstance<BlockNumber> {
	endpoint: Vec<u8>,
	// in-use, idle, slashed...
	status: u32,
	// when
	register_at: BlockNumber,
	// self-claim score, will update to challenging result
	score: Score,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// two ways to use external pallet:
	// 1. associate type
	// 2. inherite
	// we must use pallet-balances(native token POV) and pallet-assets(stable coin)
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn rendering_instances)]
	pub type RenderingInstances<T> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		RenderingInstance<T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RenderingInstanceRegistered(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		DuplicatedInstance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register(origin: OriginFor<T>, score: Score) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn play(
			origin: OriginFor<T>,
			instance: T::AccountId, /* TODO add stablecoin payment */
		) -> DispatchResult {
			let player = ensure_signed(origin)?;
			Ok(())
		}
	}
}
