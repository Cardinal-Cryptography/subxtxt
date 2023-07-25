use subxt::storage::StorageKey;

use crate::{connection::TxInfo, AccountId, Balance, BlockHash, TxStatus};

/// Any object that implemnts pallet staking read-only api.
#[async_trait::async_trait]
pub trait StakingApi {
    /// Staking era index type.
    type EraIndex;
    /// Staking ledger type.
    type StakingLedger;
    /// Staking exposure type.
    type Exposure;
    /// Staking reward points type.
    type EraRewardPoints;

    /// Returns [`active_era`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.active_era).
    /// * `at` - optional hash of a block to query state from
    async fn get_active_era(&self, at: Option<BlockHash>) -> Self::EraIndex;

    /// Returns [`current_era`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.current_era).
    /// * `at` - optional hash of a block to query state from
    async fn get_current_era(&self, at: Option<BlockHash>) -> Self::EraIndex;

    /// Returns [`bonded`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.bonded) for a given stash account.
    /// * `stash` - a stash account id
    /// * `at` - optional hash of a block to query state from
    async fn get_bonded(&self, stash: AccountId, at: Option<BlockHash>) -> Option<AccountId>;

    /// Returns [`ledger`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.ledger) for a given controller account.
    /// * `controller` - a controller account id
    /// * `at` - optional hash of a block to query state from
    async fn get_ledger(&self, controller: AccountId, at: Option<BlockHash>)
        -> Self::StakingLedger;

    /// Returns [`eras_validator_reward`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.eras_validator_reward) for a given era.
    /// * `era` - an era index
    /// * `at` - optional hash of a block to query state from
    async fn get_payout_for_era(&self, era: Self::EraIndex, at: Option<BlockHash>) -> Balance;

    /// Returns [`eras_stakers`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.eras_stakers) for a given era and account id.
    /// * `era` - an era index
    /// * `account_id` - an account id
    /// * `at` - optional hash of a block to query state from
    async fn get_exposure(
        &self,
        era: Self::EraIndex,
        account_id: &AccountId,
        at: Option<BlockHash>,
    ) -> Self::Exposure;

    /// Returns [`eras_reward_points`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.eras_reward_points) for a given era.
    /// * `era` - an era index
    /// * `at` - optional hash of a block to query state from
    async fn get_era_reward_points(
        &self,
        era: Self::EraIndex,
        at: Option<BlockHash>,
    ) -> Option<Self::EraRewardPoints>;

    /// Returns [`minimum_validator_count`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.minimum_validator_count).
    /// * `at` - optional hash of a block to query state from
    async fn get_minimum_validator_count(&self, at: Option<BlockHash>) -> u32;

    /// Returns [`SessionsPerEra`](https://paritytech.github.io/substrate/master/pallet_staking/trait.Config.html#associatedtype.SessionsPerEra) const.
    async fn get_session_per_era(&self) -> anyhow::Result<u32>;
}

/// Pallet staking api
#[async_trait::async_trait]
pub trait StakingUserApi {
    /// Staking era index type.
    type EraIndex;

    /// API for [`bond`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.bond) call.
    async fn bond(
        &self,
        initial_stake: Balance,
        controller_id: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`validate`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.validate) call.
    async fn validate(
        &self,
        validator_commission_percentage: u8,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`payout_stakers`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.payout_stakers) call.
    async fn payout_stakers(
        &self,
        stash_account: AccountId,
        era: Self::EraIndex,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`nominate`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.nominate) call.
    async fn nominate(
        &self,
        nominee_account_id: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`chill`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.chill) call.
    async fn chill(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`bond_extra`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.bond_extra) call.
    async fn bond_extra_stake(
        &self,
        extra_stake: Balance,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

/// Pallet staking logic, not directly related to any particular pallet call.
#[async_trait::async_trait]
pub trait StakingApiExt {
    /// Send batch of [`bond`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.bond) calls.
    /// * `accounts` - a slice of account ids pairs (stash, controller)
    /// * `stake` - what amount should be bonded,
    /// * `status` - a [`TxStatus`] of a tx to wait for
    ///
    /// # Examples
    /// ```ignore
    /// async fn nominate_validator(
    ///     connection: &RootConnection,
    ///     nominator_controller_accounts: Vec<AccountId>,
    ///     nominator_stash_accounts: Vec<AccountId>,
    ///     nominee_account: AccountId,
    /// ) {
    ///     let stash_controller_accounts = nominator_stash_accounts
    ///         .iter()
    ///         .cloned()
    ///         .zip(nominator_controller_accounts.iter().cloned())
    ///         .collect::<Vec<_>>();
    ///
    ///     let mut rng = thread_rng();
    ///     for chunk in stash_controller_accounts
    ///         .chunks(256)
    ///         .map(|c| c.to_vec())
    ///     {
    ///         let stake = 100 * 1_000_000_000_000u128;
    ///         connection
    ///             .batch_bond(&chunk, stake, TxStatus::Submitted)
    ///             .await
    ///             .unwrap();
    ///     }
    ///     let nominator_nominee_accounts = nominator_controller_accounts
    ///        .iter()
    ///        .cloned()
    ///        .zip(iter::repeat(&nominee_account).cloned())
    ///        .collect::<Vec<_>>();
    ///     for chunks in nominator_nominee_accounts.chunks(128) {
    ///        connection
    ///            .batch_nominate(chunks, TxStatus::InBlock)
    ///            .await
    ///            .unwrap();
    ///    }
    /// }
    /// ```
    async fn batch_bond(
        &self,
        accounts: &[(AccountId, AccountId)],
        stake: Balance,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// Send batch of [`nominate`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.nominate) calls.
    /// * `nominator_nominee_pairs` - a slice of account ids pairs (nominator, nominee)
    /// * `status` - a [`TxStatus`] of a tx to wait for
    ///
    /// # Examples
    /// see [`Self::batch_bond`] example above
    async fn batch_nominate(
        &self,
        nominator_nominee_pairs: &[(AccountId, AccountId)],
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

/// Pallet staking api that requires sudo.
#[async_trait::async_trait]
pub trait StakingSudoApi {
    /// API for [`force_new_era`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.force_new_era) call.
    async fn force_new_era(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`set_staking_config`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.set_staking_configs) call.
    async fn set_staking_config(
        &self,
        minimal_nominator_bond: Option<Balance>,
        minimal_validator_bond: Option<Balance>,
        max_nominators_count: Option<u32>,
        max_validators_count: Option<u32>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

/// Logic for retrieving raw storage keys or values from a pallet staking.
#[async_trait::async_trait]
pub trait StakingRawApi {
    /// Staking era index type.
    type EraIndex;

    /// Returns all encoded [`eras_stakers`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.eras_stakers).
    /// storage keys for a given era
    /// * `era` - an era index
    /// * `at` - optional hash of a block to query state from
    ///
    /// # Examples
    /// ```ignore
    /// let stakers = connection
    ///         .get_stakers_storage_keys(current_era, None)
    ///         .await
    ///         .into_iter()
    ///         .map(|key| key.0);
    /// ```
    async fn get_stakers_storage_keys(
        &self,
        era: Self::EraIndex,
        at: Option<BlockHash>,
    ) -> anyhow::Result<Vec<StorageKey>>;

    /// Returns encoded [`eras_stakers`](https://paritytech.github.io/substrate/master/pallet_staking/struct.Pallet.html#method.eras_stakers).
    /// storage keys for a given era and given account ids
    /// * `era` - an era index
    /// * `accounts` - list of account ids
    /// * `at` - optional hash of a block to query state from
    async fn get_stakers_storage_keys_from_accounts(
        &self,
        era: Self::EraIndex,
        accounts: &[AccountId],
        at: Option<BlockHash>,
    ) -> Vec<StorageKey>;
}
