use std::collections::HashMap;

use bitcoin::{BlockHash, Txid};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftforkInfo {
    r#type: String,
    active: bool,
    height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainInfo {
    chain: String,
    blocks: u64,
    headers: u64,
    bestblockhash: String,
    difficulty: u64,
    mediantime: u64,
    verificationprogress: u64,
    initialblockdownload: bool,
    chainwork: String,
    size_on_disk: u64,
    pruned: bool,
    softforks: HashMap<String, SoftforkInfo>,
    warnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block<T> {
    hash: BlockHash,
    confirmations: u64,
    strippedsize: u64,
    size: u64,
    weight: u64,
    height: u64,
    masternode: String,
    minter: String,
    minted_blocks: u64,
    stake_modifier: String,
    version: u64,
    version_hex: String,
    merkleroot: String,
    time: u64,
    mediantime: u64,
    bits: String,
    difficulty: u64,
    chainwork: String,
    tx: Vec<T>,
    n_tx: u64,
    previousblockhash: String,
    nextblockhash: String,
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
    txid: Txid,
    hash: String,
    version: u64,
    size: u64,
    vsize: u64,
    weight: u64,
    locktime: u64,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    hex: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptSig {
    asm: String,
    hex: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    coinbase: Option<String>,
    txid: Txid,
    vout: u64,
    script_sig: ScriptSig,
    txinwitness: Option<Vec<String>>,
    sequence: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    value: i64,
    n: u64,
    script_pub_key: ScriptPubKey,
    token_id: u64,
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
    asm: String,
    hex: String,
    r#type: String,
    req_sigs: u64,
    addresses: Vec<String>,
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
