#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use scale_info::TypeInfo;
use frame_system::Config;
use frame_support::sp_runtime::SaturatedConversion;

pub type PostId = u128;

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[scale_info(skip_type_params(T))]
pub struct WhoAndWhen<T: Config + pallet_timestamp::Config> {
    pub account: T::AccountId,
    pub block: T::BlockNumber,
    pub time: T::Moment,
}
impl<T: Config + pallet_timestamp::Config> WhoAndWhen<T> {
    pub fn new(account: T::AccountId) -> Self {
        WhoAndWhen {
            account,
            block: <frame_system::Pallet<T>>::block_number(),
            time: <pallet_timestamp::Pallet<T>>::now(),
        }
    }
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct FlatWhoAndWhen<AccountId, BlockNumber> {
    pub created_by: AccountId,
    pub created_at_block: BlockNumber,
    pub created_at_time: u64,

}
impl<T: Config + pallet_timestamp::Config> From<WhoAndWhen<T>> for FlatWhoAndWhen<T::AccountId, T::BlockNumber> {
    fn from(created: WhoAndWhen<T>) -> Self {
        Self {
            created_by: created.account,
            created_at_block: created.block,
            created_at_time: created.time.saturated_into::<u64>()
        }
    }
}


