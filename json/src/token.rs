use std::collections::BTreeMap;

use crate::common::UTXO;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResult(pub BTreeMap<String, TokenInfo>);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub symbol: String,
    pub symbol_key: String,
    pub name: String,
    pub decimal: u8,
    pub limit: i64,
    pub mintable: bool,
    pub tradeable: bool,
    #[serde(rename = "isDAT")]
    pub is_dat: bool,
    #[serde(rename = "isLPS")]
    pub is_lps: bool,
    pub is_loan_token: bool,
    pub finalized: bool,
    pub minted: f64,
    pub creation_tx: String,
    pub creation_height: i64,
    pub destruction_tx: String,
    pub destruction_height: i64,
    pub collateral_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenMetadata {
    symbol: String,
    name: String,
    #[serde(rename = "isDAT")]
    is_dat: bool,
    mintable: bool,
    tradeable: bool,
    collateral_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTokenMetadata {
    symbol: Option<String>,
    name: Option<String>,
    #[serde(rename = "isDAT")]
    is_dat: Option<bool>,
    mintable: Option<bool>,
    tradeable: Option<bool>,
    finalize: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenPagination {
    pub start: u64,
    pub including_start: bool,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCustomTxResult {
    r#type: String,
    valid: bool,
    results: serde_json::Value,
    block_height: String,
    blockhash: String,
    confirmations: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeCustomTxResult {
    txid: bitcoin::Txid,
    r#type: String,
    valid: bool,
    results: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintTokensOptions {
    amounts: Vec<String>,
    utxos: Option<Vec<UTXO>>,
    to: Option<String>,
}
