pub trait BlockchainRPC {
    fn get_best_block_hash(&self) -> Result<String>;
    fn get_block(&self, hash: String) -> Result<String>;
    fn get_block_count(&self) -> Result<u64>;
    fn get_block_hash(&self, height: u64) -> Result<String>;
    fn get_block_header(&self, hash: String) -> Result<BlockHeader>;
    fn get_block_stats(&self, stats: Option<Array>) -> Result<BlockStats>;
    fn get_blockchain_info(&self) -> Result<BlockchainInfo>;
    fn get_chain_tips(&self) -> Result<Vec<ChainTip>>;
    fn get_chain_tx_stats(
        &self,
        n_blocks: Option<u64>,
        block_hash: Option<String>,
    ) -> Result<ChainTxStats>;
    fn get_difficulty(&self) -> Result<u64>;
    fn get_mempool_ancestors(&self, tx_id: String) -> Result<()>;
    fn get_mempool_descendants(&self, tx_id: String) -> Result<()>;
    fn get_mempool_entry(&self, tx_id: String) -> Result<MempoolTx>;
    fn get_mempool_info(&self) -> Result<MempoolInfo>;
    fn get_raw_mempool(&self) -> Result<()>;
    fn get_tx_out(&self, tx_id: String, index: u64) -> Result<UTXODetails>;
    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo>;
    fn wait_for_block(&self, blockhash: String) -> Result<WaitBlockResult>;
    fn wait_for_block_height(&self, height: u64) -> Result<WaitBlockResult>;
    fn wait_for_new_block(&self) -> Result<WaitBlockResult>;
}
