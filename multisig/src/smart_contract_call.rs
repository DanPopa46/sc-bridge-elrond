use elrond_wasm::api::BigUintApi;
use elrond_wasm::types::{Address, BoxedBytes, TokenIdentifier};
use transaction::*;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum EgldEsdtSwapCall<BigUint: BigUintApi> {
    IssueWrappedEgld {
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: BigUint,
        issue_cost: BigUint,
    },
    SetLocalMintRole,
    MintWrappedEgld {
        amount: BigUint,
    },
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum EsdtSafeCall<BigUint: BigUintApi> {
    SetTransactionFee {
        transaction_fee: BigUint,
    },
    AddTokenToWhitelist {
        token_id: TokenIdentifier,
    },
    RemoveTokenFromWhitelist {
        token_id: TokenIdentifier,
    },
    GetNextPendingTransaction,
    SetTransactionStatus {
        sender: Address,
        nonce: Nonce,
        transaction_status: TransactionStatus,
    },
    Claim,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum MultiTransferEsdtCall<BigUint: BigUintApi> {
    IssueEsdtToken {
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: BigUint,
        issue_cost: BigUint,
    },
    SetLocalMintRole {
        token_id: TokenIdentifier,
    },
    MintEsdtToken {
        token_id: TokenIdentifier,
        amount: BigUint,
    },
    TransferEsdtToken {
        to: Address,
        token_id: TokenIdentifier,
        amount: BigUint,
    },
}
