pub trait NetRPC: RpcApi {
    fn get_connection_count(&self) -> Result<u64>;
    fn get_net_totals(&self) -> Result<NetTotals>;
    fn get_network_info(&self) -> Result<NetworkInfo>;
    fn get_peer_info(&self) -> Result<Vec<PeerInfo>>;
    fn set_network_active(&self, state: bool) -> Result<bool>;
}
