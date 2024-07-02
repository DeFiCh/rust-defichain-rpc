use std::{fmt, collections::HashMap};
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

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

#[derive(Debug)]
pub enum VaultState {
    Unknown,
    Active,
    InLiquidation,
    Frozen,
    MayLiquidate,
}

impl Serialize for VaultState {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            VaultState::Unknown => serializer.serialize_str("unknown"),
            VaultState::Active => serializer.serialize_str("active"),
            VaultState::InLiquidation => serializer.serialize_str("inLiquidation"),
            VaultState::Frozen => serializer.serialize_str("frozen"),
            VaultState::MayLiquidate => serializer.serialize_str("mayLiquidate"),
        }
    }
}

impl<'a> Deserialize<'a> for VaultState {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<VaultState, D::Error> {
        struct VaultStateVisitor;

        impl<'de> Visitor<'de> for VaultStateVisitor {
            type Value = VaultState;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`Unknown`, `Active`, `InLiquidation`, `Frozon`, `MayLiquidate`")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E>
            {
                match v {
                    "active" => Ok(VaultState::Active),
                    "inLiquidation" => Ok(VaultState::InLiquidation),
                    "frozen" => Ok(VaultState::Frozen),
                    "mayLiquidation" => Ok(VaultState::MayLiquidate),
                    _ => Ok(VaultState::Unknown),
                }
            }
        }

        deserializer.deserialize_identifier(VaultStateVisitor)
    }
}

impl VaultState {
    fn in_liquidation() -> Self { VaultState::InLiquidation }
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
    pub vault_id: String,
    pub loan_scheme_id: String,
    pub owner_address: String,
    pub state: VaultState,
    pub collateral_amounts: Vec<String>,
    pub loan_amounts: Vec<String>,
    pub interest_amounts: Vec<String>,
    pub collateral_value: f64,
    pub loan_value: f64,
    pub interest_value: f64,
    pub collateral_ratio: i64,
    pub informative_ratio: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_collateral_ratio: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_per_block_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests_per_block: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultLiquidation {
    pub vault_id: String,
    pub loan_scheme_id: String,
    pub owner_address: String,
    #[serde(default = "VaultState::in_liquidation")]
    pub state: VaultState,
    pub liquidation_height: u64,
    pub liquidation_penalty: f64,
    pub batch_count: usize,
    pub batches: Vec<VaultLiquidationBatch>,
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
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPaginationStart {
    pub vault_id: String,
    pub height: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultLiquidationBatch {
    pub index: u32,
    pub collaterals: Vec<String>,
    pub loan: String,
    pub highest_bid: Option<HighestBid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighestBid {
    pub amount: String,
    pub owner: String,
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
