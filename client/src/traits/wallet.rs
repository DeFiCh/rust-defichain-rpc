pub trait WalletRPC: RpcApi {
    fn create_wallet(&self, wallet_name: String) -> Result<CreateWalletResult>;
    fn dump_priv_key(&self, address: String) -> Result<String>;
    fn get_address_info(&self, address: String) -> Result<AddressInfo>;
    fn get_balance(&self) -> Result<BigNumber>;
    fn get_balances(&self) -> Result<WalletBalances>;
    fn get_new_address(&self) -> Result<String>;
    fn get_transaction(&self, txid: bitcoin::Txid) -> Result<InWalletTransaction>;
    fn get_unconfirmed_balance(&self) -> Result<BigNumber>;
    fn get_wallet_info(&self) -> Result<WalletInfo>;
    fn import_priv_key(&self, privkey: String) -> Result<()>;
    fn list_address_groupings(&self) -> Result<()>;
    fn list_unspent(&self) -> Result<Vec<UTXO>>;
    fn list_wallets(&self) -> Result<()>;
    fn send_many(&self, amounts: Record) -> Result<String>;
    fn send_to_address(&self, address: String, amount: u64) -> Result<String>;
    fn set_wallet_flag(&self, flag: WalletFlag) -> Result<WalletFlagResult>;
    fn sign_message(&self, address: String, message: String) -> Result<String>;
    fn validate_address(&self, address: String) -> Result<ValidateAddressResult>;
}
