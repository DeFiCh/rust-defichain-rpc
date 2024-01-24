pub trait EvmRPC {
    fn evmtx(&self) -> Result<String>;
}
