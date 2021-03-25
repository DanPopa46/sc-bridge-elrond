use transaction::*;

elrond_wasm::imports!();

#[elrond_wasm_derive::callable(EgldEsdtSwapProxy)]
pub trait EgldEsdtSwap {
    fn issueWrappedEgld(
        &self,
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: BigUint,
    ) -> ContractCall<BigUint, ()>; // payable EGLD
    fn setLocalMintRole(&self) -> ContractCall<BigUint, ()>;
    fn mintWrappedEgld(&self, amount: BigUint) -> ContractCall<BigUint, ()>;
}

#[elrond_wasm_derive::callable(EsdtSafeProxy)]
pub trait EsdtSafe {
    fn setTransactionFee(&self, transaction_fee: BigUint) -> ContractCall<BigUint, ()>;
    fn addTokenToWhitelist(&self, token_id: TokenIdentifier) -> ContractCall<BigUint, ()>;
    fn removeTokenFromWhitelist(&self, token_id: TokenIdentifier) -> ContractCall<BigUint, ()>;
    fn getNextPendingTransaction(
        &self,
    ) -> ContractCall<BigUint, MultiArg5<Nonce, Address, Address, TokenIdentifier, BigUint>>;
    fn setTransactionStatus(
        &self,
        sender: Address,
        nonce: Nonce,
        transaction_status: TransactionStatus,
    ) -> ContractCall<BigUint, ()>;
    fn claim(&self) -> ContractCall<BigUint, ()>;
}

#[elrond_wasm_derive::callable(MultiTransferEsdtProxy)]
pub trait MultiTransferEsdt {
    fn issueEsdtToken(
        &self,
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: BigUint,
    ) -> ContractCall<BigUint, ()>; // payable EGLD
    fn setLocalMintRole(&self, token_id: TokenIdentifier) -> ContractCall<BigUint, ()>;
    fn mintEsdtToken(
        &self,
        token_id: TokenIdentifier,
        amount: BigUint,
    ) -> ContractCall<BigUint, ()>;
    fn transferEsdtToken(
        &self,
        to: Address,
        token_id: TokenIdentifier,
        amount: BigUint,
    ) -> ContractCall<BigUint, ()>;
}
