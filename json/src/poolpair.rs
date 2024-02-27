use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePoolPairMetadata {
    token_a: String,
    token_b: String,
    commission: u64,
    status: bool,
    owner_address: String,
    custom_rewards: Option<Vec<String>>,
    pair_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePoolPairMetadata {
    pool: String,
    status: bool,
    commission: u64,
    owner_address: String,
    custom_rewards: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    txid: String,
    vout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolPairsResult(HashMap<String, PoolPairInfo>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolPairInfo {
    pub symbol: String,
    pub name: String,
    pub status: bool,
    pub id_token_a: String,
    pub id_token_b: String,
    pub dex_fee_pct_token_a: Option<i64>,
    pub dex_fee_in_pct_token_a: Option<i64>,
    pub dex_fee_out_pct_token_a: Option<i64>,
    pub dex_fee_pct_token_b: Option<i64>,
    pub dex_fee_in_pct_token_b: Option<i64>,
    pub dex_fee_out_pct_token_b: Option<i64>,
    pub reserve_a: i64,
    pub reserve_b: i64,
    pub commission: i64,
    pub total_liquidity: i64,
    #[serde(rename = "reserveA/reserveB")]
    pub reserve_a_reserve_b: f64,
    #[serde(rename = "reserveB/reserveA")]
    pub reserve_b_reserve_a: f64,
    pub trade_enabled: bool,
    pub owner_address: String,
    pub block_commission_a: i64,
    pub block_commission_b: i64,
    pub reward_pct: i64,
    pub reward_loan_pct: i64,
    pub custom_rewards: Option<Vec<String>>,
    pub creation_tx: String,
    pub creation_height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolSharesResult(HashMap<String, PoolShareInfo>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolShareInfo {
    pool_id: String,
    owner: String,
    yo: i64,
    amount: i64,
    total_liquidity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolPairPagination {
    pub start: u64,
    pub including_start: bool,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPoolLiquiditySource(HashMap<String, Vec<String>>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolLiquidityOptions {
    utxos: Option<Vec<UTXO>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolShareOptions {
    is_mine_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolSwapMetadata {
    from: String,
    token_from: String,
    amount_from: u64,
    to: String,
    token_to: String,
    max_price: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPoolSwapVerboseResult {
    path: String,
    pools: Vec<String>,
    amount: String,
}
