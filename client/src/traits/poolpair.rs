use async_trait::async_trait;
use defichain_rpc_json::common::UTXO;
use defichain_rpc_json::poolpair::*;

use crate::{into_json, obj_into_json, Client, Result, RpcApi};

#[async_trait]
pub trait PoolPairRPC: RpcApi {
    async fn add_pool_liquidity(
        &self,
        from: AddPoolLiquiditySource,
        share_address: String,
        options: PoolLiquidityOptions,
    ) -> Result<String>;
    async fn composite_swap(
        &self,
        metadata: PoolSwapMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn create_pool_pair(
        &self,
        metadata: CreatePoolPairMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn get_pool_pair(&self, symbol: String, verbose: Option<bool>)
        -> Result<PoolPairsResult>;
    async fn list_pool_pairs(
        &self,
        pagination: Option<PoolPairPagination>,
        verbose: Option<bool>,
    ) -> Result<PoolPairsResult>;
    async fn list_pool_shares(
        &self,
        pagination: PoolPairPagination,
        verbose: Option<bool>,
        options: PoolShareOptions,
    ) -> Result<PoolSharesResult>;
    async fn pool_swap(&self, metadata: PoolSwapMetadata, utxos: Option<UTXO>) -> Result<String>;
    async fn remove_pool_liquidity(
        &self,
        address: String,
        pool_account: String,
        options: PoolLiquidityOptions,
    ) -> Result<String>;
    async fn test_pool_swap(&self, metadata: PoolSwapMetadata) -> Result<String>;
    async fn update_pool_pair(
        &self,
        metadata: UpdatePoolPairMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String>;
}

#[async_trait]
impl PoolPairRPC for Client {
    async fn add_pool_liquidity(
        &self,
        from: AddPoolLiquiditySource,
        share_address: String,
        options: PoolLiquidityOptions,
    ) -> Result<String> {
        self.call(
            "addpoolliquidity",
            &[into_json(from)?, into_json(share_address)?, into_json(options)?],
        )
        .await
    }
    async fn composite_swap(
        &self,
        metadata: PoolSwapMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("compositeswap", &[into_json(metadata)?, into_json(utxos)?]).await
    }
    async fn create_pool_pair(
        &self,
        metadata: CreatePoolPairMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("createpoolpair", &[into_json(metadata)?, into_json(utxos)?]).await
    }
    async fn get_pool_pair(
        &self,
        symbol: String,
        verbose: Option<bool>,
    ) -> Result<PoolPairsResult> {
        self.call("getpoolpair", &[into_json(symbol)?, into_json(verbose)?]).await
    }
    async fn list_pool_pairs(
        &self,
        pagination: Option<PoolPairPagination>,
        verbose: Option<bool>,
    ) -> Result<PoolPairsResult> {
        self.call("listpoolpairs", &[obj_into_json(pagination)?, into_json(verbose)?]).await
    }
    async fn list_pool_shares(
        &self,
        pagination: PoolPairPagination,
        verbose: Option<bool>,
        options: PoolShareOptions,
    ) -> Result<PoolSharesResult> {
        self.call(
            "listpoolshares",
            &[into_json(pagination)?, into_json(verbose)?, into_json(options)?],
        )
        .await
    }
    async fn pool_swap(&self, metadata: PoolSwapMetadata, utxos: Option<UTXO>) -> Result<String> {
        self.call("poolswap", &[into_json(metadata)?, into_json(utxos)?]).await
    }
    async fn remove_pool_liquidity(
        &self,
        address: String,
        pool_account: String,
        options: PoolLiquidityOptions,
    ) -> Result<String> {
        self.call(
            "removepoolliquidity",
            &[into_json(address)?, into_json(pool_account)?, into_json(options)?],
        )
        .await
    }
    async fn test_pool_swap(&self, metadata: PoolSwapMetadata) -> Result<String> {
        self.call("testpoolswap", &[into_json(metadata)?]).await
    }
    async fn update_pool_pair(
        &self,
        metadata: UpdatePoolPairMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("updatepoolpair", &[into_json(metadata)?, into_json(utxos)?]).await
    }
}
