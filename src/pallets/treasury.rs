use crate::{connection::TxInfo, AccountId, Balance, BlockHash, TxStatus};

/// Pallet treasury read-only api.
#[async_trait::async_trait]
pub trait TreasuryApi {
    /// Returns an unique account id for all treasury transfers.
    async fn treasury_account(&self) -> AccountId;

    /// Returns storage `proposals_count`.
    /// * `at` - an optional block hash to query state from
    async fn proposals_count(&self, at: Option<BlockHash>) -> Option<u32>;

    /// Returns storage `approvals`.
    /// * `at` - an optional block hash to query state from
    async fn approvals(&self, at: Option<BlockHash>) -> Vec<u32>;
}

/// Pallet treasury api.
#[async_trait::async_trait]
pub trait TreasuryUserApi {
    /// API for [`propose_spend`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.propose_spend) call.
    async fn propose_spend(
        &self,
        amount: Balance,
        beneficiary: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`approve_proposal`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.approve_proposal) call.
    async fn approve(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`reject_proposal`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.reject_proposal) call.
    async fn reject(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo>;
}

/// Pallet treasury functionality that is not directly related to any pallet call.
#[async_trait::async_trait]
pub trait TreasureApiExt {
    /// When `staking.payout_stakers` is done, what amount of AZERO is transferred to the treasury.
    async fn possible_treasury_payout(&self) -> anyhow::Result<Balance>;
}
