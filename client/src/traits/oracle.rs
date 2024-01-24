pub trait OracleRPC {
    fn appoint_oracle(&self, address: String) -> Result<String>;
    fn get_fixed_interval_price(&self, id: String) -> Result<FixedIntervalPrice>;
    fn get_future_swap_block(&self) -> Result<u64>;
    fn get_oracle_data(&self, oracle_id: String) -> Result<OracleData>;
    fn get_price(&self, price_feed: OraclePriceFeed) -> Result<BigNumber>;
    fn list_fixed_interval_prices(&self) -> Result<Vec<ListFixedIntervalPrice>>;
    fn list_latest_raw_prices(
        &self,
        price_feed: Option<OraclePriceFeed>,
    ) -> Result<Vec<OracleRawPrice>>;
    fn list_oracles(&self) -> Result<()>;
    fn list_prices(
        &self,
        start_index: Option<u64>,
        including_start: Option<bool>,
        limit: Option<u64>,
    ) -> Result<Vec<ListPricesData>>;
    fn remove_oracle(&self, oracle_id: String, utxos: Option<UTXO>) -> Result<String>;
    fn set_oracle_data(&self, oracle_id: String, timestamp: u64) -> Result<String>;
    fn update_oracle(&self, oracle_id: String, address: String) -> Result<String>;
}
