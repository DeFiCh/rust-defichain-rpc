pub trait TokenRPC {
    fn burn_tokens(
        &self,
        amounts: String,
        from: Option<String>,
        context: Option<String>,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn create_token(&self, metadata: CreateTokenMetadata, utxos: Option<UTXO>) -> Result<String>;
    fn decode_custom_tx(&self, hexstring: String, iswitness: Option<bool>) -> Result<()>;
    fn get_custom_tx(&self, txid: bitcoin::Txid, blockhash: Option<String>) -> Result<()>;
    fn get_token(&self, symbol_key: String) -> Result<TokenResult>;
    fn list_tokens(&self) -> Result<TokenResult>;
    fn mint_tokens(&self, options: MintTokensOptions) -> Result<String>;
    fn update_token(&self, token: String, metadata: Option<UpdateTokenMetadata>) -> Result<String>;
}
