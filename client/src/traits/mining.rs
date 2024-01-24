pub trait MiningRPC: RpcApi {
    fn estimate_smart_fee(&self, confirmation_target: u64) -> Result<SmartFeeEstimation>;
    fn get_mining_info(&self) -> Result<MiningInfo>;
    fn get_network_hash_per_second(&self) -> Result<u64>;
}
