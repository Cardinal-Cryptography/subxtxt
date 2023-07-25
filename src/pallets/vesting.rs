use crate::{connection::TxInfo, AccountId, BlockHash, TxStatus};

/// Read only pallet vesting API.
#[async_trait::async_trait]
pub trait VestingApi {
    /// Information about the vesting schedule.
    type VestingInfo;

    /// Returns [`VestingInfo`] of the given account.
    /// * `who` - an account id
    /// * `at` - optional hash of a block to query state from
    async fn get_vesting(&self, who: AccountId, at: Option<BlockHash>) -> Vec<Self::VestingInfo>;
}

/// Pallet vesting api.
#[async_trait::async_trait]
pub trait VestingUserApi {
    /// Information about the vesting schedule.
    type VestingInfo;

    /// API for [`vest`](https://paritytech.github.io/substrate/master/pallet_vesting/pallet/enum.Call.html#variant.vest) call.
    async fn vest(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`vest_other`](https://paritytech.github.io/substrate/master/pallet_vesting/pallet/enum.Call.html#variant.vest_other) call.
    async fn vest_other(&self, status: TxStatus, other: AccountId) -> anyhow::Result<TxInfo>;

    /// API for [`vested_transfer`](https://paritytech.github.io/substrate/master/pallet_vesting/pallet/enum.Call.html#variant.vested_transfer) call.
    async fn vested_transfer(
        &self,
        receiver: AccountId,
        schedule: Self::VestingInfo,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`merge_schedules`](https://paritytech.github.io/substrate/master/pallet_vesting/pallet/enum.Call.html#variant.merge_schedules) call.
    async fn merge_schedules(
        &self,
        idx1: u32,
        idx2: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}
