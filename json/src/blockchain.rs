use std::collections::HashMap;

use bitcoin::{BlockHash, Txid};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftforkInfo {
    r#type: String,
    active: bool,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainInfo {
    chain: String,
    pub blocks: u32,
    headers: u32,
    bestblockhash: String,
    difficulty: f64,
    mediantime: u64,
    verificationprogress: u64,
    initialblockdownload: bool,
    chainwork: String,
    size_on_disk: u64,
    pruned: bool,
    pub softforks: HashMap<String, SoftforkInfo>,
    warnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block<T> {
    pub hash: BlockHash,
    pub height: u32,
    pub confirmations: i64,
    pub strippedsize: u64,
    pub size: u64,
    pub weight: u64,
    pub masternode: Option<String>,
    pub minter: Option<String>,
    pub minted_blocks: u64,
    pub stake_modifier: String,
    pub version: i32,
    pub version_hex: String,
    pub merkleroot: String,
    pub time: i64,
    pub mediantime: i64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub tx: Vec<T>,
    pub n_tx: u64,
    pub previousblockhash: Option<BlockHash>,
    pub nextblockhash: Option<BlockHash>,
    #[serde(skip)]
    pub nonutxo: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    hash: String,
    confirmations: u64,
    height: u64,
    version: u64,
    version_hex: String,
    merkleroot: String,
    time: u64,
    mediantime: u64,
    bits: String,
    difficulty: u64,
    chainwork: String,
    n_tx: u64,
    previousblockhash: String,
    nextblockhash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub txid: Txid,
    pub hash: String,
    pub version: u32,
    pub size: u64,
    pub vsize: u64,
    pub weight: u64,
    pub locktime: u64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptSig {
    asm: String,
    pub hex: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Vin {
    Coinbase(VinCoinbase),
    Standard(VinStandard),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VinCoinbase {
    pub coinbase: String,
    pub sequence: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VinStandard {
    pub txid: Txid,
    pub vout: u64,
    pub script_sig: ScriptSig,
    pub txinwitness: Option<Vec<String>>,
    pub sequence: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: f64,
    pub n: u64,
    pub script_pub_key: ScriptPubKey,
    pub token_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTXODetails {
    bestblock: String,
    confirmations: u64,
    value: i64,
    script_pub_key: ScriptPubKey,
    coinbase: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxOutSetInfo {
    height: u64,
    bestblock: String,
    transactions: u64,
    txouts: u64,
    bogosize: u64,
    hash_serialized_2: String,
    disk_size: u64,
    total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPubKey {
    pub asm: String,
    #[serde(with = "crate::serde_hex")]
    pub hex: Vec<u8>,
    pub r#type: String,
    req_sigs: Option<u64>,
    pub addresses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainTip {
    height: u64,
    hash: String,
    branchlen: u64,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeData {
    base: i64,
    modified: i64,
    ancestor: i64,
    descendant: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolTxData {
    vsize: i64,
    size: i64,
    weight: i64,
    fee: i64,
    modifiedfee: i64,
    time: i64,
    height: i64,
    descendantcount: i64,
    descendantsize: i64,
    descendantfees: i64,
    ancestorcount: i64,
    ancestorsize: i64,
    ancestorfees: i64,
    wtxid: Txid,
    fees: FeeData,
    depends: Vec<String>,
    spentby: Vec<String>,
    #[serde(rename = "bip125-replaceable")]
    bip125_replaceable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MempoolTx(HashMap<String, MempoolTxData>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStats {
    avgfee: u64,
    avgfeerate: u64,
    avgtxsize: u64,
    blockhash: String,
    height: u64,
    ins: u64,
    maxfee: u64,
    maxfeerate: u64,
    maxtxsize: u64,
    medianfee: u64,
    mediantime: u64,
    mediantxsize: u64,
    minfee: u64,
    minfeerate: u64,
    mintxsize: u64,
    outs: u64,
    subsidy: u64,
    swtxs: u64,
    time: u64,
    totalfee: u64,
    txs: u64,
    swtotal_size: u64,
    swtotal_weight: u64,
    total_out: u64,
    total_size: u64,
    total_weight: u64,
    utxo_increase: u64,
    utxo_size_inc: u64,
    feerate_percentiles: [f64; 5],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolInfo {
    loaded: bool,
    size: u64,
    bytes: u64,
    usage: u64,
    maxmempool: u64,
    mempoolminfee: i64,
    minrelaytxfee: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitBlockResult {
    hash: String,
    height: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainTxStats {
    time: u64,
    txcount: u64,
    window_final_block_hash: String,
    window_final_block_height: u64,
    window_block_count: u64,
    window_tx_count: u64,
    window_interval: u64,
    txrate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockResult {
    Full(Block<Transaction>),
    TxHash(Block<Txid>),
    Hex(String),
}
