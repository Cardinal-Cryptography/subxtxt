use crate::{connection::TxInfo, AccountId, Balance, BlockHash, TxStatus};

/// Pallet system read-only api.
#[async_trait::async_trait]
pub trait SystemApi {
    /// returns free balance of a given account
    /// * `account` - account id
    /// * `at` - optional hash of a block to query state from
    async fn get_free_balance(&self, account: AccountId, at: Option<BlockHash>) -> Balance;
}

/// Pallet system api.
#[async_trait::async_trait]
pub trait SystemSudoApi {
    /// API for [`set_code`](https://paritytech.github.io/substrate/master/frame_system/pallet/struct.Pallet.html#method.set_code) call.
    async fn set_code(&self, code: Vec<u8>, status: TxStatus) -> anyhow::Result<TxInfo>;
}
