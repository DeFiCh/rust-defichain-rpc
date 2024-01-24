use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EstimateMode {
    Unset,
    Economical,
    Conservative,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintingInfo {
    blocks: u64,
    currentblockweight: Option<u64>,
    currentblocktx: Option<u64>,
    difficulty: String,
    isoperator: bool,
    masternodeid: Option<String>,
    masternodeoperator: Option<String>,
    masternodestate: String,
    generate: Option<bool>,
    mintedblocks: Option<u64>,
    networkhashps: u64,
    pooledtx: u64,
    chain: String,
    warnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiningInfo {
    blocks: u64,
    currentblockweight: Option<u64>,
    currentblocktx: Option<u64>,
    difficulty: String,
    isoperator: bool,
    masternodes: Vec<MasternodeInfo>,
    networkhashps: u64,
    pooledtx: u64,
    chain: String,
    warnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasternodeInfo {
    id: String,
    operator: String,
    state: String,
    generate: bool,
    mintedblocks: u64,
    lastblockcreationattempt: String,
    target_multiplier: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartFeeEstimation {
    pub feerate: Option<f64>,
    errors: Option<Vec<String>>,
    blocks: u64,
}
