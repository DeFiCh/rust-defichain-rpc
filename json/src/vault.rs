use std::collections::HashMap;

use crate::loan::LoanSchemeResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVault {
    owner_address: String,
    loan_scheme_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVault {
    owner_address: Option<String>,
    loan_scheme_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VaultState {
    Unknown,
    Active,
    InLiquidation,
    Frozen,
    MayLiquidate,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vault {
    vault_id: String,
    loan_scheme_id: String,
    owner_address: String,
    state: VaultState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultActive {
    vault_id: String,
    loan_scheme_id: String,
    owner_address: String,
    state: VaultState,
    collateral_amounts: Vec<String>,
    loan_amounts: Vec<String>,
    interest_amounts: Vec<String>,
    collateral_value: f64,
    loan_value: f64,
    interest_value: f64,
    collateral_ratio: i64,
    informative_ratio: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_collateral_ratio: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interest_per_block_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interests_per_block: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultLiquidation {
    vault_id: String,
    loan_scheme_id: String,
    owner_address: String,
    state: VaultState,
    liquidation_height: u64,
    liquidation_penalty: f64,
    batch_count: u64,
    batches: Vec<VaultLiquidationBatch>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositVault {
    vault_id: String,
    from: String,
    amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawVault {
    vault_id: String,
    to: String,
    amount: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VaultPagination {
    start: Option<String>,
    including_start: Option<bool>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListVaultOptions {
    pub owner_address: Option<String>,
    pub loan_scheme_id: Option<String>,
    pub state: Option<VaultState>,
    pub verbose: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseVault {
    vault_id: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAuctionBid {
    vault_id: String,
    index: u64,
    from: String,
    amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPagination {
    pub start: Option<AuctionPaginationStart>,
    pub including_start: Option<bool>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPaginationStart {
    pub vault_id: Option<String>,
    pub height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultLiquidationBatch {
    index: usize,
    collaterals: Vec<String>,
    loan: String,
    highest_bid: Option<HighestBid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighestBid {
    amount: String,
    owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAuctionHistoryPagination {
    max_block_height: Option<u64>,
    vault_id: Option<String>,
    index: Option<u64>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAuctionHistoryDetail {
    winner: String,
    block_height: u64,
    block_hash: String,
    block_time: u64,
    vault_id: String,
    batch_index: u64,
    auction_bid: String,
    auction_won: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultEstimation {
    collateral_value: i64,
    loan_value: i64,
    informative_ratio: i64,
    collateral_ratio: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPercentageSplit(HashMap<String, u64>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VaultResult {
    VaultActive(VaultActive), // Verbose active
    VaultLiquidation(VaultLiquidation), // Verbose in liquidation
                              // Vault(Vault),                       // Any state non-verbose
}
