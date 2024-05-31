pub trait RawTxRPC: RpcApi {
    fn create_raw_transaction(&self, outputs: CreateRawTxOut) -> Result<String>;
    fn decode_raw_transaction(&self, hexstring: String, iswitness: bool) -> Result<RawTransaction>;
    fn get_raw_transaction(&self, txid: bitcoin::Txid) -> Result<String>;
    fn send_raw_transaction(&self, signed_tx: String,max_fee_rate:Option<u64>) -> Result<String>;
    fn sign_raw_transaction_with_key(&self, raw_tx: String) -> Result<SignRawTxWithKeyResult>;
    fn test_mempool_accept(&self, signed_tx: String,max_fee_rate:Option<u64>) -> Result<TestMempoolAcceptResult>;
}
