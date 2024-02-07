use crate::bitcoin::BlockHash;
use crate::{into_json, Client, Error, Result, RpcApi};
use async_trait::async_trait;
use defichain_rpc_json::bitcoin::Txid;
use defichain_rpc_json::blockchain::*;

#[async_trait]
pub trait BlockchainRPC: RpcApi {
    async fn get_best_block_hash(&self) -> Result<BlockHash>;
    async fn get_block(&self, hash: BlockHash, verbosity: u8) -> Result<GetBlockResult>;
    async fn get_block_count(&self) -> Result<u64>;
    async fn get_block_hash(&self, height: u32) -> Result<BlockHash>;
    async fn get_block_header(&self, hash: BlockHash) -> Result<BlockHeader>;
    async fn get_block_stats(&self, height: u32, stats: Option<Vec<String>>) -> Result<BlockStats>;
    async fn get_blockchain_info(&self) -> Result<BlockchainInfo>;
    async fn get_chain_tips(&self) -> Result<Vec<ChainTip>>;
    async fn get_chain_tx_stats(
        &self,
        n_blocks: Option<u64>,
        block_hash: Option<BlockHash>,
    ) -> Result<ChainTxStats>;
    async fn get_difficulty(&self) -> Result<u64>;
    async fn get_mempool_ancestors(&self, tx_id: Txid) -> Result<()>;
    async fn get_mempool_descendants(&self, tx_id: Txid) -> Result<()>;
    async fn get_mempool_entry(&self, tx_id: Txid) -> Result<MempoolTx>;
    async fn get_mempool_info(&self) -> Result<MempoolInfo>;
    async fn get_raw_mempool(&self) -> Result<()>;
    async fn get_tx_out(
        &self,
        tx_id: Txid,
        index: u64,
        include_mempool: Option<bool>,
    ) -> Result<UTXODetails>;
    async fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo>;
    async fn wait_for_block(&self, blockhash: BlockHash) -> Result<WaitBlockResult>;
    async fn wait_for_block_height(&self, height: u64) -> Result<WaitBlockResult>;
    async fn wait_for_new_block(&self) -> Result<WaitBlockResult>;
}

#[async_trait]
impl BlockchainRPC for Client {
    async fn get_best_block_hash(&self) -> Result<BlockHash> {
        self.call("getbestblockhash", &[]).await
    }

    async fn get_block(&self, hash: BlockHash, verbosity: u8) -> Result<GetBlockResult> {
        if verbosity > 2 {
            return Err(Error::Custom("Only verbosity up to 2 is supported".into()));
        }
        self.call("getblock", &[into_json(hash)?, into_json(verbosity)?]).await
    }

    async fn get_block_count(&self) -> Result<u64> {
        self.call("getblockcount", &[]).await
    }

    async fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
        self.call("getblockhash", &[into_json(height)?]).await
    }

    async fn get_block_header(&self, hash: BlockHash) -> Result<BlockHeader> {
        self.call("getblockheader", &[into_json(hash)?]).await
    }

    async fn get_block_stats(&self, height: u32, stats: Option<Vec<String>>) -> Result<BlockStats> {
        self.call("getblockstats", &[into_json(height)?, into_json(stats)?]).await
    }

    async fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.call("getblockchaininfo", &[]).await
    }

    async fn get_chain_tips(&self) -> Result<Vec<ChainTip>> {
        self.call("getchaintips", &[]).await
    }

    async fn get_chain_tx_stats(
        &self,
        n_blocks: Option<u64>,
        block_hash: Option<BlockHash>,
    ) -> Result<ChainTxStats> {
        self.call("getchaintxstats", &[into_json(n_blocks)?, into_json(block_hash)?]).await
    }

    async fn get_difficulty(&self) -> Result<u64> {
        self.call("getdifficulty", &[]).await
    }

    async fn get_mempool_ancestors(&self, tx_id: Txid) -> Result<()> {
        self.call("getmempoolancestors", &[into_json(tx_id)?]).await
    }

    async fn get_mempool_descendants(&self, tx_id: Txid) -> Result<()> {
        self.call("getmempooldescendants", &[into_json(tx_id)?]).await
    }

    async fn get_mempool_entry(&self, tx_id: Txid) -> Result<MempoolTx> {
        self.call("getmempoolentry", &[into_json(tx_id)?]).await
    }

    async fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.call("getmempoolinfo", &[]).await
    }

    async fn get_raw_mempool(&self) -> Result<()> {
        self.call("getrawmempool", &[]).await
    }

    async fn get_tx_out(
        &self,
        tx_id: Txid,
        index: u64,
        include_mempool: Option<bool>,
    ) -> Result<UTXODetails> {
        self.call(
            "gettxout",
            &[into_json(tx_id)?, into_json(index)?, into_json(include_mempool.unwrap_or(true))?],
        )
        .await
    }

    async fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo> {
        self.call("gettxoutsetinfo", &[]).await
    }

    async fn wait_for_block(&self, blockhash: BlockHash) -> Result<WaitBlockResult> {
        self.call("waitforblock", &[into_json(blockhash)?]).await
    }

    async fn wait_for_block_height(&self, height: u64) -> Result<WaitBlockResult> {
        self.call("waitforblockheight", &[into_json(height)?]).await
    }

    async fn wait_for_new_block(&self) -> Result<WaitBlockResult> {
        self.call("waitfornewblock", &[]).await
    }
}
