pub trait PoolPairRPC {
    fn add_pool_liquidity(
        &self,
        from: AddPoolLiquiditySource,
        share_address: String,
    ) -> Result<String>;
    fn composite_swap(&self, metadata: PoolSwapMetadata, utxos: Option<UTXO>) -> Result<String>;
    fn create_pool_pair(
        &self,
        metadata: CreatePoolPairMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn get_pool_pair(&self, symbol: String) -> Result<PoolPairsResult>;
    fn list_pool_pairs(&self) -> Result<PoolPairsResult>;
    fn list_pool_shares(&self) -> Result<PoolSharesResult>;
    fn pool_swap(&self, metadata: PoolSwapMetadata, utxos: Option<UTXO>) -> Result<String>;
    fn remove_pool_liquidity(&self, address: String, pool_account: String) -> Result<String>;
    fn test_pool_swap(&self, metadata: PoolSwapMetadata) -> Result<String>;
    fn update_pool_pair(
        &self,
        metadata: UpdatePoolPairMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String>;
}
