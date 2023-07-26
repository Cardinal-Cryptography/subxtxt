use crate::{connection::TxInfo, AccountId, BlockHash, TxStatus};

/// Pallet session read-only api.
#[async_trait::async_trait]
pub trait SessionApi {
    /// Session keys type.
    type SessionKeys;
    /// Session index type.
    type SessionIndex;

    /// API for [`next_keys`](https://paritytech.github.io/substrate/master/pallet_session/pallet/type.NextKeys.html) call.
    async fn get_next_session_keys(
        &self,
        account: AccountId,
        at: Option<BlockHash>,
    ) -> Option<Self::SessionKeys>;

    /// API for [`current_index`](https://paritytech.github.io/substrate/master/pallet_session/pallet/struct.Pallet.html#method.current_index) call.
    async fn get_session(&self, at: Option<BlockHash>) -> Self::SessionIndex;

    /// API for [`validators`](https://paritytech.github.io/substrate/master/pallet_session/pallet/struct.Pallet.html#method.validators) call.
    async fn get_validators(&self, at: Option<BlockHash>) -> Vec<AccountId>;
}

/// Pallet session API.
#[async_trait::async_trait]
pub trait SessionUserApi {
    /// Session keys type.
    type SessionKeys;

    /// API for [`set_keys`](https://paritytech.github.io/substrate/master/pallet_session/pallet/struct.Pallet.html#method.set_keys) call.
    async fn set_keys(
        &self,
        new_keys: Self::SessionKeys,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}
