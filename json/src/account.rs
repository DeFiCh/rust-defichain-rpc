use std::collections::HashMap;

use crate::common::UTXO;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OwnerType {
    Mine,
    All,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Id,
    Symbol,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelectionModeType {
    Pie,
    Crumbs,
    Forward,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferDomainType {
    None = 0,
    Utxo = 1,
    Dvm = 2,
    Evm = 3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountsPagination {
    start: Option<String>,
    including_start: Option<bool>,
    limit: Option<u64>,
}

impl ListAccountsPagination {
    pub fn new(start: Option<String>, including_start: Option<bool>, limit: Option<u64>) -> Self {
        Self {
            start,
            including_start,
            limit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountPagination {
    start: Option<u32>,
    including_start: Option<bool>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AccountsResult<T, U> {
    key: String,
    owner: T,
    amount: U,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountsResultOwner {
    asm: String,
    hex: String,
    req_sigs: Option<u64>,
    r#type: String,
    addresses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AccountAmount(pub Vec<String>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountOptions {
    indexed_amounts: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenBalancesOptions {
    symbol_lookup: Option<bool>,
    include_eth: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceTransferPayload(HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceTransferAccountOptions {
    utxos: Option<Vec<UTXO>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistory {
    pub owner: String,
    block_height: u64,
    block_hash: Option<String>,
    block_time: Option<u64>,
    r#type: String,
    txn: Option<u64>,
    txid: Option<String>,
    pub amounts: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistoryOptions {
    max_block_height: Option<u64>,
    depth: Option<u64>,
    #[serde(rename = "no_rewards")]
    no_rewards: Option<bool>,
    token: Option<String>,
    txtype: Option<char>,
    txtypes: Option<Vec<char>>,
    limit: Option<u64>,
    start: Option<u64>,
    #[serde(rename = "including_start")]
    including_start: Option<bool>,
    txn: Option<u64>,
    format: Option<Format>,
}

impl AccountHistoryOptions {
    pub fn new(
        max_block_height: Option<u64>,
        depth: Option<u64>,
        no_rewards: Option<bool>,
        token: Option<String>,
        txtype: Option<char>,
        txtypes: Option<Vec<char>>,
        limit: Option<u64>,
        start: Option<u64>,
        including_start: Option<bool>,
        txn: Option<u64>,
        format: Option<crate::account::Format>,
    ) -> Self {
        Self {
            max_block_height,
            depth,
            no_rewards,
            token,
            txtype,
            txtypes,
            limit,
            start,
            including_start,
            txn,
            format,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistoryCountOptions {
    token: Option<String>,
    txtype: Option<char>,
    txtypes: Option<Vec<char>>,
    no_rewards: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressBalances(HashMap<String, Vec<String>>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTokensOptions {
    selection_mode: SelectionModeType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityBalanceData {
    anchor_reward: i64,
    incentive_funding: Option<i64>,
    burnt: i64,
    swap: Option<i64>,
    futures: Option<i64>,
    options: Option<i64>,
    unallocated: Option<i64>,
    unknown: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BurnHistoryOptions {
    max_block_height: Option<u64>,
    depth: Option<u64>,
    token: Option<String>,
    txtype: Option<char>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BurnHistory {
    owner: String,
    block_height: u64,
    block_hash: String,
    block_time: u64,
    r#type: String,
    txn: u64,
    txid: String,
    amounts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BurnInfo {
    address: String,
    pub amount: f64,
    pub tokens: Vec<String>,
    pub feeburn: f64,
    pub emissionburn: f64,
    pub auctionburn: f64,
    pub paybackburn: Vec<String>,
    dexfeetokens: Vec<String>,
    dfipaybackfee: f64,
    dfipaybacktokens: Vec<String>,
    paybackfees: Vec<String>,
    paybacktokens: Vec<String>,
    #[serde(rename = "dfip2203")]
    dfip_2203: Vec<String>,
    #[serde(rename = "dfip2206f")]
    dfip_2206_f: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureSwap {
    address: String,
    amount: String,
    destination: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFutureInfo {
    owner: String,
    values: Vec<FutureData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureData {
    source: String,
    destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFutureInfo {
    owner: String,
    source: String,
    destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DusdSwapsInfo {
    owner: String,
    amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferDomain {
    src: TransferDomainData,
    dst: TransferDomainData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDomainData {
    address: String,
    amount: String,
    domain: TransferDomainType,
}
