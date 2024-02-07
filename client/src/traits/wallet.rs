#[async_trait]
pub trait WalletRPC: RpcApi {
    async fn create_wallet(&self, wallet_name: String) -> Result<CreateWalletResult>;
    async fn dump_priv_key(&self, address: String) -> Result<String>;
    async fn get_address_info(&self, address: String) -> Result<AddressInfo>;
    async fn get_balance(&self) -> Result<BigNumber>;
    async fn get_balances(&self) -> Result<WalletBalances>;
    async fn get_new_address(&self) -> Result<String>;
    async fn get_transaction(&self, txid: bitcoin::Txid) -> Result<InWalletTransaction>;
    async fn get_unconfirmed_balance(&self) -> Result<BigNumber>;
    async fn get_wallet_info(&self) -> Result<WalletInfo>;
    async fn import_priv_key(&self, privkey: String) -> Result<()>;
    async fn list_address_groupings(&self) -> Result<()>;
    async fn list_unspent(&self) -> Result<Vec<UTXO>>;
    async fn list_wallets(&self) -> Result<()>;
    async fn send_many(&self, amounts: Record) -> Result<String>;
    async fn send_to_address(&self, address: String, amount: u64) -> Result<String>;
    async fn set_wallet_flag(&self, flag: WalletFlag) -> Result<WalletFlagResult>;
    async fn sign_message(&self, address: String, message: String) -> Result<String>;
    async fn validate_address(&self, address: String) -> Result<ValidateAddressResult>;
}
