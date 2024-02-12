use crate::token::TokenResult;
use serde_with::serde_as;
use serde_with::OneOrMany;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoanScheme {
    min_col_ratio: u64,
    interest_rate: i64,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLoanScheme {
    min_col_ratio: u64,
    interest_rate: i64,
    id: String,
    activate_after_block: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestroyLoanScheme {
    id: String,
    activate_after_block: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanSchemeResult {
    pub id: String,
    #[serde(rename(serialize = "minColRatio"))]
    mincolratio: i64,
    #[serde(rename(serialize = "interestRate"))]
    interestrate: i64,
    default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralToken {
    token: String,
    factor: i64,
    fixed_interval_price_id: String,
    activate_after_block: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralTokenDetail {
    token: String,
    factor: i64,
    fixed_interval_price_id: String,
    token_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLoanToken {
    symbol: String,
    name: Option<String>,
    fixed_interval_price_id: String,
    mintable: Option<bool>,
    interest: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanTokenResult {
    token: TokenResult,
    fixed_interval_price_id: String,
    interest: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanConfig {
    fixed_interval_blocks: u64,
    max_price_deviation_pct: f64,
    min_oracles_per_price: u64,
    scheme: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanSummary {
    pub collateral_tokens: u64,
    pub collateral_value: f64,
    pub loan_tokens: u64,
    pub loan_value: f64,
    pub open_auctions: u64,
    pub open_vaults: u64,
    pub schemes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoanInfoResult {
    current_price_block: i64,
    next_price_block: i64,
    defaults: LoanConfig,
    pub totals: LoanSummary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLoanToken {
    symbol: Option<String>,
    name: Option<String>,
    fixed_interval_price_id: Option<String>,
    mintable: Option<bool>,
    interest: Option<i64>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeLoanMetadata {
    vault_id: String,
    #[serde_as(as = "OneOrMany<_>")]
    amounts: Vec<String>,
    to: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaybackLoanMetadata {
    vault_id: String,
    #[serde_as(as = "OneOrMany<_>")]
    amounts: Vec<String>,
    from: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPaybackAmount {
    d_token: String,
    #[serde_as(as = "OneOrMany<_>")]
    amounts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaybackLoanMetadataV2 {
    vault_id: String,
    from: String,
    loans: Vec<TokenPaybackAmount>,
}
