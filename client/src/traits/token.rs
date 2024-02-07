use crate::{into_json, obj_into_json, Client, Result, RpcApi};
use async_trait::async_trait;
use defichain_rpc_json::{common::UTXO, token::*};

#[async_trait]
pub trait TokenRPC: RpcApi {
    async fn burn_tokens(
        &self,
        amounts: String,
        from: Option<String>,
        context: Option<String>,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn create_token(
        &self,
        metadata: CreateTokenMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn decode_custom_tx(&self, hexstring: String, iswitness: Option<bool>) -> Result<()>;
    async fn get_custom_tx(&self, txid: String, blockhash: Option<String>) -> Result<()>;
    async fn get_token(&self, symbol_key: String) -> Result<TokenResult>;
    async fn list_tokens(
        &self,
        pagination: Option<TokenPagination>,
        verbose: Option<bool>,
    ) -> Result<TokenResult>;
    async fn mint_tokens(&self, options: MintTokensOptions) -> Result<String>;
    async fn update_token(
        &self,
        token: String,
        metadata: Option<UpdateTokenMetadata>,
    ) -> Result<String>;
}

#[async_trait]
impl TokenRPC for Client {
    async fn burn_tokens(
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
        .await
    }
    async fn create_token(
        &self,
        metadata: CreateTokenMetadata,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("createtoken", &[into_json(metadata)?, into_json(utxos)?]).await
    }
    async fn decode_custom_tx(&self, hexstring: String, iswitness: Option<bool>) -> Result<()> {
        self.call("decodecustomtx", &[into_json(hexstring)?, into_json(iswitness)?]).await
    }
    async fn get_custom_tx(&self, txid: String, blockhash: Option<String>) -> Result<()> {
        self.call("getcustomtx", &[into_json(txid)?, into_json(blockhash)?]).await
    }
    async fn get_token(&self, symbol_key: String) -> Result<TokenResult> {
        self.call("gettoken", &[into_json(symbol_key)?]).await
    }
    async fn list_tokens(
        &self,
        pagination: Option<TokenPagination>,
        verbose: Option<bool>,
    ) -> Result<TokenResult> {
        self.call("listtokens", &[obj_into_json(pagination)?, into_json(verbose)?]).await
    }
    async fn mint_tokens(&self, options: MintTokensOptions) -> Result<String> {
        self.call("minttokens", &[into_json(options)?]).await
    }
    async fn update_token(
        &self,
        token: String,
        metadata: Option<UpdateTokenMetadata>,
    ) -> Result<String> {
        self.call("updatetoken", &[into_json(token)?, into_json(metadata)?]).await
    }
}
