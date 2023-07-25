use crate::{connection::TxInfo, AccountId, BlockNumber, TxStatus, Weight};

/// An alias for a call hash.
pub type CallHash = [u8; 32];
/// An alias for a threshold.
pub type MultisigThreshold = u16;

/// Struct describing coordinates of a multisig aggregation.
#[derive(Clone, Debug, Eq, PartialEq, codec::Decode, codec::Encode)]
pub struct Timepoint {
    /// Index of a block.
    pub height: BlockNumber,
    /// Index of a call in a block.
    pub index: BlockNumber,
}

/// Pallet multisig api.
#[async_trait::async_trait]
pub trait MultisigUserApi {
    /// Runtime call API.
    type Call;

    /// API for [`as_multi_threshold_1`](https://paritytech.github.io/substrate/master/pallet_multisig/pallet/struct.Pallet.html#method.as_multi_threshold_1) call.
    async fn as_multi_threshold_1(
        &self,
        other_signatories: Vec<AccountId>,
        call: Self::Call,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`as_multi`](https://paritytech.github.io/substrate/master/pallet_multisig/pallet/struct.Pallet.html#method.as_multi) call.
    async fn as_multi(
        &self,
        threshold: MultisigThreshold,
        other_signatories: Vec<AccountId>,
        timepoint: Option<Timepoint>,
        max_weight: Weight,
        call: Self::Call,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`approve_as_multi`](https://paritytech.github.io/substrate/master/pallet_multisig/pallet/struct.Pallet.html#method.approve_as_multi) call.
    async fn approve_as_multi(
        &self,
        threshold: MultisigThreshold,
        other_signatories: Vec<AccountId>,
        timepoint: Option<Timepoint>,
        max_weight: Weight,
        call_hash: CallHash,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// API for [`cancel_as_multi`](https://paritytech.github.io/substrate/master/pallet_multisig/pallet/struct.Pallet.html#method.cancel_as_multi) call.
    async fn cancel_as_multi(
        &self,
        threshold: MultisigThreshold,
        other_signatories: Vec<AccountId>,
        timepoint: Timepoint,
        call_hash: CallHash,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}
