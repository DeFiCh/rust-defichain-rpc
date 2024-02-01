use crate::bitcoin::BlockHash;
use crate::{into_json, Client, Error, Result, RpcApi};
use defichain_rpc_json::bitcoin::Txid;
use defichain_rpc_json::blockchain::*;

pub trait BlockchainRPC: RpcApi {
    fn get_best_block_hash(&self) -> Result<BlockHash>;
    fn get_block(&self, hash: BlockHash, verbosity: u8) -> Result<GetBlockResult>;
    fn get_block_count(&self) -> Result<u64>;
    fn get_block_hash(&self, height: u32) -> Result<BlockHash>;
    fn get_block_header(&self, hash: BlockHash) -> Result<BlockHeader>;
    fn get_block_stats(&self, height: u32, stats: Option<Vec<String>>) -> Result<BlockStats>;
    fn get_blockchain_info(&self) -> Result<BlockchainInfo>;
    fn get_chain_tips(&self) -> Result<Vec<ChainTip>>;
    fn get_chain_tx_stats(
        &self,
        n_blocks: Option<u64>,
        block_hash: Option<BlockHash>,
    ) -> Result<ChainTxStats>;
    fn get_difficulty(&self) -> Result<u64>;
    fn get_mempool_ancestors(&self, tx_id: Txid) -> Result<()>;
    fn get_mempool_descendants(&self, tx_id: Txid) -> Result<()>;
    fn get_mempool_entry(&self, tx_id: Txid) -> Result<MempoolTx>;
    fn get_mempool_info(&self) -> Result<MempoolInfo>;
    fn get_raw_mempool(&self) -> Result<()>;
    fn get_tx_out(
        &self,
        tx_id: Txid,
        index: u64,
        include_mempool: Option<bool>,
    ) -> Result<UTXODetails>;
    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo>;
    fn wait_for_block(&self, blockhash: BlockHash) -> Result<WaitBlockResult>;
    fn wait_for_block_height(&self, height: u64) -> Result<WaitBlockResult>;
    fn wait_for_new_block(&self) -> Result<WaitBlockResult>;
}

impl BlockchainRPC for Client {
    fn get_best_block_hash(&self) -> Result<BlockHash> {
        self.call("getbestblockhash", &[])
    }

    fn get_block(&self, hash: BlockHash, verbosity: u8) -> Result<GetBlockResult> {
        if verbosity > 2 {
            return Err(Error::Custom("Only verbosity up to 2 is supported".into()));
        }
        self.call("getblock", &[into_json(hash)?, into_json(verbosity)?])
    }

    fn get_block_count(&self) -> Result<u64> {
        self.call("getblockcount", &[])
    }

    fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
        self.call("getblockhash", &[into_json(height)?])
    }

    fn get_block_header(&self, hash: BlockHash) -> Result<BlockHeader> {
        self.call("getblockheader", &[into_json(hash)?])
    }

    fn get_block_stats(&self, height: u32, stats: Option<Vec<String>>) -> Result<BlockStats> {
        self.call("getblockstats", &[into_json(height)?, into_json(stats)?])
    }

    fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.call("getblockchaininfo", &[])
    }

    fn get_chain_tips(&self) -> Result<Vec<ChainTip>> {
        self.call("getchaintips", &[])
    }

    fn get_chain_tx_stats(
        &self,
        n_blocks: Option<u64>,
        block_hash: Option<BlockHash>,
    ) -> Result<ChainTxStats> {
        self.call("getchaintxstats", &[into_json(n_blocks)?, into_json(block_hash)?])
    }

    fn get_difficulty(&self) -> Result<u64> {
        self.call("getdifficulty", &[])
    }

    fn get_mempool_ancestors(&self, tx_id: Txid) -> Result<()> {
        self.call("getmempoolancestors", &[into_json(tx_id)?])
    }

    fn get_mempool_descendants(&self, tx_id: Txid) -> Result<()> {
        self.call("getmempooldescendants", &[into_json(tx_id)?])
    }

    fn get_mempool_entry(&self, tx_id: Txid) -> Result<MempoolTx> {
        self.call("getmempoolentry", &[into_json(tx_id)?])
    }

    fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.call("getmempoolinfo", &[])
    }

    fn get_raw_mempool(&self) -> Result<()> {
        self.call("getrawmempool", &[])
    }

    fn get_tx_out(
        &self,
        tx_id: Txid,
        index: u64,
        include_mempool: Option<bool>,
    ) -> Result<UTXODetails> {
        self.call(
            "gettxout",
            &[into_json(tx_id)?, into_json(index)?, into_json(include_mempool.unwrap_or(true))?],
        )
    }

    fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo> {
        self.call("gettxoutsetinfo", &[])
    }

    fn wait_for_block(&self, blockhash: BlockHash) -> Result<WaitBlockResult> {
        self.call("waitforblock", &[into_json(blockhash)?])
    }

    fn wait_for_block_height(&self, height: u64) -> Result<WaitBlockResult> {
        self.call("waitforblockheight", &[into_json(height)?])
    }

    fn wait_for_new_block(&self) -> Result<WaitBlockResult> {
        self.call("waitfornewblock", &[])
    }
}
