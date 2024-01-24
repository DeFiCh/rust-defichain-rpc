pub trait EvmRPC: RpcApi {
    fn evmtx(&self) -> Result<String>;
}
