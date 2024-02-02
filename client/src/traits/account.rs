use crate::{into_json, Client, Result, RpcApi};
use defichain_rpc_json::account::*;

pub trait AccountRPC: RpcApi {
    fn account_to_account(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String>;
    fn account_to_utxos(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String>;
    fn future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String>;
    fn get_account(
        &self,
        owner: String,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
    ) -> Result<AccountAmount>;
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
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
        options: Option<GetTokenBalancesOptions>,
    ) -> Result<()>;
    fn history_count(
        &self,
        owner: Option<String>,
        options: AccountHistoryCountOptions,
    ) -> Result<u64>;
    fn list_account_history(
        &self,
        owner: Option<String>,
        options: AccountHistoryOptions,
    ) -> Result<Vec<AccountHistory>>;
    // TODO handle AccountResult enum type
    // fn list_accounts(
    //     &self,
    //     pagination: Option<ListAccountPagination>,
    //     verbose: Option<bool>,
    //     options: Option<ListAccountOptions>,
    // ) -> Result<Vec<AccountResult<String, String>>>;
    fn list_burn_history(&self, options: BurnHistoryOptions) -> Result<Vec<BurnHistory>>;
    fn list_community_balances(&self) -> Result<CommunityBalanceData>;
    fn list_pending_dusd_swaps(&self) -> Result<Vec<DusdSwapsInfo>>;
    fn list_pending_future_swaps(&self) -> Result<Vec<ListFutureInfo>>;
    fn send_tokens_to_address(
        &self,
        from: AddressBalances,
        to: AddressBalances,
        options: SendTokensOptions,
    ) -> Result<String>;
    fn transfer_domain(&self, payload: Vec<TransferDomain>) -> Result<String>;
    fn utxos_to_account(
        &self,
        payload: BalanceTransferPayload,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn withdraw_future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String>;
}
impl AccountRPC for Client {
    fn account_to_account(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String> {
        self.call("accounttoaccount", &[into_json(from)?, into_json(payload)?, into_json(options)?])
    }
    fn account_to_utxos(
        &self,
        from: String,
        payload: BalanceTransferPayload,
        options: BalanceTransferAccountOptions,
    ) -> Result<String> {
        self.call("accounttoutxos", &[into_json(from)?, into_json(payload)?, into_json(options)?])
    }
    fn future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String> {
        self.call("futureswap", &[into_json(future)?, into_json(utxos)?])
    }
    fn get_account(
        &self,
        owner: String,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
    ) -> Result<AccountAmount> {
        self.call(
            "getaccount",
            &[into_json(owner)?, into_json(pagination)?, into_json(indexed_amounts)?],
        )
    }
    fn get_account_history(
        &self,
        owner: String,
        block_height: u64,
        txn: u64,
    ) -> Result<AccountHistory> {
        self.call(
            "getaccounthistory",
            &[into_json(owner)?, into_json(block_height)?, into_json(txn)?],
        )
    }
    fn get_burn_info(&self) -> Result<BurnInfo> {
        self.call("getburninfo", &[])
    }
    fn get_pending_dusd_swaps(&self, address: String) -> Result<DusdSwapsInfo> {
        self.call("getpendingdusdswaps", &[into_json(address)?])
    }
    fn get_pending_future_swaps(&self, address: String) -> Result<GetFutureInfo> {
        self.call("getpendingfutureswaps", &[into_json(address)?])
    }
    fn get_token_balances(
        &self,
        pagination: Option<GetAccountPagination>,
        indexed_amounts: Option<bool>,
        options: Option<GetTokenBalancesOptions>,
    ) -> Result<()> {
        self.call(
            "gettokenbalances",
            &[into_json(pagination)?, into_json(indexed_amounts)?, into_json(options)?],
        )
    }
    fn history_count(
        &self,
        owner: Option<String>,
        options: AccountHistoryCountOptions,
    ) -> Result<u64> {
        self.call("historycount", &[into_json(owner)?, into_json(options)?])
    }
    fn list_account_history(
        &self,
        owner: Option<String>,
        options: AccountHistoryOptions,
    ) -> Result<Vec<AccountHistory>> {
        self.call("listaccounthistory", &[into_json(owner)?, into_json(options)?])
    }
    // fn list_accounts(
    //     &self,
    //     pagination: Option<ListAccountPagination>,
    //     verbose: Option<bool>,
    //     options: Option<ListAccountOptions>,
    // ) -> Result<Vec<AccountResult<String, String>>> {
    //     self.call(
    //         "listaccounts",
    //         &[into_json(pagination)?, into_json(verbose)?, into_json(options)?],
    //     )
    // }
    fn list_burn_history(&self, options: BurnHistoryOptions) -> Result<Vec<BurnHistory>> {
        self.call("listburnhistory", &[into_json(options)?])
    }
    fn list_community_balances(&self) -> Result<CommunityBalanceData> {
        self.call("listcommunitybalances", &[])
    }
    fn list_pending_dusd_swaps(&self) -> Result<Vec<DusdSwapsInfo>> {
        self.call("listpendingdusdswaps", &[])
    }
    fn list_pending_future_swaps(&self) -> Result<Vec<ListFutureInfo>> {
        self.call("listpendingfutureswaps", &[])
    }
    fn send_tokens_to_address(
        &self,
        from: AddressBalances,
        to: AddressBalances,
        options: SendTokensOptions,
    ) -> Result<String> {
        self.call("sendtokenstoaddress", &[into_json(from)?, into_json(to)?, into_json(options)?])
    }
    fn transfer_domain(&self, payload: Vec<TransferDomain>) -> Result<String> {
        self.call("transferdomain", &[into_json(payload)?])
    }
    fn utxos_to_account(
        &self,
        payload: BalanceTransferPayload,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("utxostoaccount", &[into_json(payload)?, into_json(utxos)?])
    }
    fn withdraw_future_swap(&self, future: FutureSwap, utxos: Option<UTXO>) -> Result<String> {
        self.call("withdrawfutureswap", &[into_json(future)?, into_json(utxos)?])
    }
}
