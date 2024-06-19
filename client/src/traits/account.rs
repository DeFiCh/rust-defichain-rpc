use crate::{into_json, obj_into_json, Client, Result, RpcApi};
use async_trait::async_trait;
use defichain_rpc_json::{account::*, common::UTXO};

#[async_trait]
pub trait AccountRPC: RpcApi {
    async fn account_to_account(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String>;
    async fn account_to_utxos(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String>;
    async fn future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String>;
    async fn get_account(
        &self,
        owner: &str,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
    ) -> Result<AccountAmount>;
    async fn get_account_history(
        &self,
        owner: &str,
        block_height: u32,
        txn: u32,
    ) -> Result<AccountHistory>;
    async fn get_burn_info(&self) -> Result<BurnInfo>;
    async fn get_pending_dusd_swaps(&self, address: String) -> Result<DusdSwapsInfo>;
    async fn get_pending_future_swaps(&self, address: String) -> Result<GetFutureInfo>;
    async fn get_token_balances(
        &self,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
        options: Option<GetTokenBalancesOptions>,
    ) -> Result<()>;
    async fn history_count(
        &self,
        owner: Option<String>,
        options: AccountHistoryCountOptions,
    ) -> Result<u64>;
    async fn list_account_history(
        &self,
        owner: Option<String>,
        options: AccountHistoryOptions,
    ) -> Result<Vec<AccountHistory>>;
    async fn list_accounts(
        &self,
        pagination: Option<ListAccountsPagination>,
        verbose: Option<bool>,
        indexed_amounts: Option<bool>,
        is_mine_only: Option<bool>,
    ) -> Result<Vec<AccountsResult<AccountsResultOwner, String>>>;
    async fn list_burn_history(&self, options: BurnHistoryOptions) -> Result<Vec<BurnHistory>>;
    async fn list_community_balances(&self) -> Result<CommunityBalanceData>;
    async fn list_pending_dusd_swaps(&self) -> Result<Vec<DusdSwapsInfo>>;
    async fn list_pending_future_swaps(&self) -> Result<Vec<ListFutureInfo>>;
    async fn send_tokens_to_address(
        &self,
        from: AddressBalances,
        to: AddressBalances,
        options: SendTokensOptions,
    ) -> Result<String>;
    async fn transfer_domain(&self, payload: Vec<TransferDomain>) -> Result<String>;
    async fn utxos_to_account(
        &self,
        payload: BalanceTransferPayload,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn withdraw_future_swap(&self, future: FutureSwap, utxos: Option<UTXO>)
        -> Result<String>;
}

#[async_trait]
impl AccountRPC for Client {
    async fn account_to_account(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String> {
        self.call("accounttoaccount", &[into_json(from)?, into_json(payload)?, into_json(options)?])
            .await
    }
    async fn account_to_utxos(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String> {
        self.call("accounttoutxos", &[into_json(from)?, into_json(payload)?, into_json(options)?])
            .await
    }
    async fn future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String> {
        self.call("futureswap", &[into_json(future)?, into_json(utxos)?]).await
    }
    async fn get_account(
        &self,
        owner: &str,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
    ) -> Result<AccountAmount> {
        self.call(
            "getaccount",
            &[into_json(owner)?, obj_into_json(pagination)?, into_json(indexed_amounts)?],
        )
        .await
    }
    async fn get_account_history(
        &self,
        owner: &str,
        block_height: u32,
        txn: u32,
    ) -> Result<AccountHistory> {
        self.call(
            "getaccounthistory",
            &[into_json(owner)?, into_json(block_height)?, into_json(txn)?],
        )
        .await
    }
    async fn get_burn_info(&self) -> Result<BurnInfo> {
        self.call("getburninfo", &[]).await
    }
    async fn get_pending_dusd_swaps(&self, address: String) -> Result<DusdSwapsInfo> {
        self.call("getpendingdusdswaps", &[into_json(address)?]).await
    }
    async fn get_pending_future_swaps(&self, address: String) -> Result<GetFutureInfo> {
        self.call("getpendingfutureswaps", &[into_json(address)?]).await
    }
    async fn get_token_balances(
        &self,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
        options: Option<GetTokenBalancesOptions>,
    ) -> Result<()> {
        self.call(
            "gettokenbalances",
            &[into_json(pagination)?, into_json(indexed_amounts)?, into_json(options)?],
        )
        .await
    }
    async fn history_count(
        &self,
        owner: Option<String>,
        options: AccountHistoryCountOptions,
    ) -> Result<u64> {
        self.call("historycount", &[into_json(owner)?, into_json(options)?]).await
    }
    async fn list_account_history(
        &self,
        owner: Option<String>,
        options: AccountHistoryOptions,
    ) -> Result<Vec<AccountHistory>> {
        self.call("listaccounthistory", &[into_json(owner)?, into_json(options)?]).await
    }
    async fn list_accounts(
        &self,
        pagination: Option<ListAccountsPagination>,
        verbose: Option<bool>,
        indexed_amounts: Option<bool>,
        is_mine_only: Option<bool>,
    ) -> Result<Vec<AccountsResult<AccountsResultOwner, String>>> {
        self.call(
            "listaccounts",
            &[
                into_json(pagination)?,
                into_json(verbose)?,
                into_json(indexed_amounts)?,
                into_json(is_mine_only)?,
            ],
        )
        .await
    }
    async fn list_burn_history(&self, options: BurnHistoryOptions) -> Result<Vec<BurnHistory>> {
        self.call("listburnhistory", &[into_json(options)?]).await
    }
    async fn list_community_balances(&self) -> Result<CommunityBalanceData> {
        self.call("listcommunitybalances", &[]).await
    }
    async fn list_pending_dusd_swaps(&self) -> Result<Vec<DusdSwapsInfo>> {
        self.call("listpendingdusdswaps", &[]).await
    }
    async fn list_pending_future_swaps(&self) -> Result<Vec<ListFutureInfo>> {
        self.call("listpendingfutureswaps", &[]).await
    }
    async fn send_tokens_to_address(
        &self,
        from: AddressBalances,
        to: AddressBalances,
        options: SendTokensOptions,
    ) -> Result<String> {
        self.call("sendtokenstoaddress", &[into_json(from)?, into_json(to)?, into_json(options)?])
            .await
    }
    async fn transfer_domain(&self, payload: Vec<TransferDomain>) -> Result<String> {
        self.call("transferdomain", &[into_json(payload)?]).await
    }
    async fn utxos_to_account(
        &self,
        payload: BalanceTransferPayload,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("utxostoaccount", &[into_json(payload)?, into_json(utxos)?]).await
    }
    async fn withdraw_future_swap(
        &self,
        future: FutureSwap,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("withdrawfutureswap", &[into_json(future)?, into_json(utxos)?]).await
    }
}
