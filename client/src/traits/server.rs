pub trait ServerRPC {
    fn get_rpc_info(&self) -> Result<RpcInfo>;
    fn uptime(&self) -> Result<u64>;
}
