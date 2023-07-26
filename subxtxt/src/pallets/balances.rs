use crate::{connection::TxInfo, AccountId, Balance, BlockHash, TxStatus};

/// Pallet balances read-only API.
#[async_trait::async_trait]
pub trait BalancesApi {
    /// The type of balance lock.
    type BalanceLock: Send;

    /// API for [`locks`](https://paritytech.github.io/substrate/master/pallet_balances/pallet/struct.Pallet.html#method.locks) call.
    /// * `account` - an account to query locked balance for
    /// * `at` - optional hash of a block to query state from
    async fn locks_for_account(
        &self,
        account: AccountId,
        at: Option<BlockHash>,
    ) -> Vec<Self::BalanceLock>;

    /// API for [`locks`](https://paritytech.github.io/substrate/master/pallet_balances/pallet/struct.Pallet.html#method.locks) call.
    /// * `accounts` - a list of accounts to query locked balance for
    /// * `at` - optional hash of a block to query state from
    ///
    /// By default, this calls `locks_for_account` for each account in `accounts`.
    async fn locks(
        &self,
        accounts: &[AccountId],
        at: Option<BlockHash>,
    ) -> Vec<Vec<Self::BalanceLock>> {
        let mut locks = vec![];
        for account in accounts {
            locks.push(self.locks_for_account(account.clone(), at).await);
        }
        locks
    }

    /// Returns [`total_issuance`](https://paritytech.github.io/substrate/master/pallet_balances/pallet/type.TotalIssuance.html).
    async fn total_issuance(&self, at: Option<BlockHash>) -> Balance;

    /// Returns [`existential_deposit`](https://paritytech.github.io/substrate/master/pallet_balances/index.html#terminology).
    async fn existential_deposit(&self) -> anyhow::Result<Balance>;
}

/// Pallet balances API
#[async_trait::async_trait]
pub trait BalancesUserApi {
    /// API for [`transfer`](https://paritytech.github.io/substrate/master/pallet_balances/pallet/struct.Pallet.html#method.transfer) call.
    async fn transfer(
        &self,
        dest: AccountId,
        amount: Balance,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`transfer`](https://paritytech.github.io/substrate/master/pallet_balances/pallet/struct.Pallet.html#method.transfer) call.
    /// Include tip in the tx.
    async fn transfer_with_tip(
        &self,
        dest: AccountId,
        amount: Balance,
        tip: Balance,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

/// Pallet balances logic not directly related to any pallet call.
#[async_trait::async_trait]
pub trait BalancesUserBatchExtApi {
    /// Performs batch of `balances.transfer` calls.
    /// * `dest` - a list of accounts to send tokens to
    /// * `amount` - an amount to transfer
    /// * `status` - a [`TxStatus`] for a tx to wait for
    ///
    /// # Examples
    /// ```ignore
    ///  for chunk in stash_accounts.chunks(1024) {
    ///         connection
    ///             .batch_transfer(chunk, 1_000_000_000_000u128, TxStatus::InBlock)
    ///             .await
    ///             .unwrap();
    ///     }
    /// ```
    async fn batch_transfer(
        &self,
        dest: &[AccountId],
        amount: Balance,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}
