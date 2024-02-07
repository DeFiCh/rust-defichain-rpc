#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    txid: bitcoin::Txid,
    vout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interest {
    token: String,
    realized_interest_per_block: i64,
    total_interest: i64,
    interest_per_block: i64,
}
