use crate::{into_json, obj_into_json, Client, Result, RpcApi};
use defichain_rpc_json::{common::UTXO, token::*};

pub trait TokenRPC: RpcApi {
    fn burn_tokens(
        &self,
        amounts: String,
        from: Option<String>,
        context: Option<String>,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn create_token(&self, metadata: CreateTokenMetadata, utxos: Option<UTXO>) -> Result<String>;
    fn decode_custom_tx(&self, hexstring: String, iswitness: Option<bool>) -> Result<()>;
    fn get_custom_tx(&self, txid: String, blockhash: Option<String>) -> Result<()>;
    fn get_token(&self, symbol_key: String) -> Result<TokenResult>;
    fn list_tokens(
        &self,
        pagination: Option<TokenPagination>,
        verbose: Option<bool>,
    ) -> Result<TokenResult>;
    fn mint_tokens(&self, options: MintTokensOptions) -> Result<String>;
    fn update_token(&self, token: String, metadata: Option<UpdateTokenMetadata>) -> Result<String>;
}
impl TokenRPC for Client {
    fn burn_tokens(
        &self,
        amounts: String,
        from: Option<String>,
        context: Option<String>,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call(
            "burntokens",
            &[into_json(amounts)?, into_json(from)?, into_json(context)?, into_json(utxos)?],
        )
    }
    fn create_token(&self, metadata: CreateTokenMetadata, utxos: Option<UTXO>) -> Result<String> {
        self.call("createtoken", &[into_json(metadata)?, into_json(utxos)?])
    }
    fn decode_custom_tx(&self, hexstring: String, iswitness: Option<bool>) -> Result<()> {
        self.call("decodecustomtx", &[into_json(hexstring)?, into_json(iswitness)?])
    }
    fn get_custom_tx(&self, txid: String, blockhash: Option<String>) -> Result<()> {
        self.call("getcustomtx", &[into_json(txid)?, into_json(blockhash)?])
    }
    fn get_token(&self, symbol_key: String) -> Result<TokenResult> {
        self.call("gettoken", &[into_json(symbol_key)?])
    }
    fn list_tokens(
        &self,
        pagination: Option<TokenPagination>,
        verbose: Option<bool>,
    ) -> Result<TokenResult> {
        self.call("listtokens", &[obj_into_json(pagination)?, into_json(verbose)?])
    }
    fn mint_tokens(&self, options: MintTokensOptions) -> Result<String> {
        self.call("minttokens", &[into_json(options)?])
    }
    fn update_token(&self, token: String, metadata: Option<UpdateTokenMetadata>) -> Result<String> {
        self.call("updatetoken", &[into_json(token)?, into_json(metadata)?])
    }
}
