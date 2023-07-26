use parity_scale_codec::Compact;

use crate::{connection::TxInfo, AccountId, Balance, BlockHash, CodeHash, TxStatus, Weight};

/// Arguments to [`ContractRpc::call_and_get`].
#[derive(parity_scale_codec::Encode)]
pub struct ContractCallArgs {
    /// Who is singing a tx.
    pub origin: AccountId,
    /// Address of the contract to call.
    pub dest: AccountId,
    /// The balance to transfer from the `origin` to `dest`.
    pub value: Balance,
    /// The gas limit enforced when executing the constructor.
    pub gas_limit: Option<Weight>,
    /// The maximum amount of balance that can be charged from the caller to pay for the storage consumed.
    pub storage_deposit_limit: Option<Balance>,
    /// The input data to pass to the contract.
    pub input_data: Vec<u8>,
}

/// Pallet contracts read-only api.
#[async_trait::async_trait]
pub trait ContractsApi {
    /// Information about a contract owner.
    type OwnerInfo;

    /// Returns `contracts.owner_info_of` storage for a given code hash.
    /// * `code_hash` - a code hash
    /// * `at` - optional hash of a block to query state from
    async fn get_owner_info(
        &self,
        code_hash: CodeHash,
        at: Option<BlockHash>,
    ) -> Option<Self::OwnerInfo>;
}

/// Pallet contracts api.
#[async_trait::async_trait]
pub trait ContractsUserApi {
    /// The type of determinism to use when instantiating a contract.
    type Determinism;

    /// API for [`upload_code`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.upload_code) call.
    async fn upload_code(
        &self,
        code: Vec<u8>,
        storage_limit: Option<Compact<Balance>>,
        determinism: Self::Determinism,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`instantiate`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.instantiate) call.
    #[allow(clippy::too_many_arguments)]
    async fn instantiate(
        &self,
        code_hash: CodeHash,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`instantiate_with_code`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.instantiate_with_code) call.
    #[allow(clippy::too_many_arguments)]
    async fn instantiate_with_code(
        &self,
        code: Vec<u8>,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`call`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.call) call.
    async fn call(
        &self,
        destination: AccountId,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`remove_code`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.remove_code) call.
    async fn remove_code(&self, code_hash: BlockHash, status: TxStatus) -> anyhow::Result<TxInfo>;
}

/// RPC for runtime ContractsApi
#[async_trait::async_trait]
pub trait ContractRpc {
    /// The type returned from [`call_and_get`](https://paritytech.github.io/substrate/master/pallet_contracts/trait.ContractsApi.html#method.call).
    type ContractExecResult: Send;

    /// API for [`call`](https://paritytech.github.io/substrate/master/pallet_contracts/trait.ContractsApi.html#method.call) call.
    async fn call_and_get(
        &self,
        args: ContractCallArgs,
    ) -> anyhow::Result<Self::ContractExecResult>;
}
