pub trait MiscRPC: RpcApi {
    fn derive_addresses(&self, descriptor: String) -> Result<()>;
    fn set_mock_time(&self, ts: u64) -> Result<()>;
    fn sign_message_with_priv_key(&self, privkey: String, message: String) -> Result<String>;
    fn verify_message(&self, address: String, signature: String, message: String) -> Result<bool>;
}
