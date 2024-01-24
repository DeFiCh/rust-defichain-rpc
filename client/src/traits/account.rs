pub trait AccountRPC {
    fn account_to_account(&self, from: String, payload: BalanceTransferPayload) -> Result<String>;
    fn account_to_utxos(&self, from: String, payload: BalanceTransferPayload) -> Result<String>;
    fn future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String>;
    fn get_account(&self, owner: String, pagination: AccountPagination) -> Result<AccountAmount>;
    fn get_account_history(
        &self,
        owner: String,
        block_height: u64,
        txn: u64,
    ) -> Result<AccountHistory>;
    fn get_burn_info(&self) -> Result<BurnInfo>;
    fn get_pending_dusd_swaps(&self, address: String) -> Result<DusdSwapsInfo>;
    fn get_pending_future_swaps(&self, address: String) -> Result<GetFutureInfo>;
    fn get_token_balances(
        &self,
        pagination: Option<AccountPagination>,
        indexed_amounts: Option<bool>,
        options: Option<GetTokenBalancesOptions>,
    ) -> Result<()>;
    fn history_count(&self) -> Result<u64>;
    fn list_account_history(&self) -> Result<Vec<AccountHistory>>;
    fn list_accounts(
        &self,
        pagination: Option<AccountPagination>,
        verbose: Option<bool>,
        options: Option<ListAccountOptions>,
    ) -> Result<Array>;
    fn list_burn_history(&self) -> Result<Vec<BurnHistory>>;
    fn list_community_balances(&self) -> Result<CommunityBalanceData>;
    fn list_pending_dusd_swaps(&self) -> Result<Vec<DusdSwapsInfo>>;
    fn list_pending_future_swaps(&self) -> Result<Vec<ListFutureInfo>>;
    fn send_tokens_to_address(&self, from: AddressBalances, to: AddressBalances) -> Result<String>;
    fn transfer_domain(&self, payload: Array) -> Result<String>;
    fn utxos_to_account(
        &self,
        payload: BalanceTransferPayload,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn withdraw_future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String>;
}
