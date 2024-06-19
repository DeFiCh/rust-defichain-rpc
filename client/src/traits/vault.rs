use crate::{Result, RpcApi};
use async_trait::async_trait;
use defichain_rpc_json::{common::UTXO, vault::*};

use crate::{obj_into_json, Client, Result, RpcApi};
use async_trait::async_trait;
use defichain_rpc_json::vault::*;

use crate::{into_json, Client, Result, RpcApi};

#[async_trait]
pub trait VaultRPC: RpcApi {
    async fn close_vault(&self, close_vault: CloseVault, utxos: Option<UTXO>) -> Result<String>;
    async fn create_vault(&self, vault: CreateVault, utxos: Option<UTXO>) -> Result<String>;
    async fn deposit_to_vault(
        &self,
        deposit_vault: DepositVault,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn estimate_collateral(
        &self,
        target_ratio: u64,
        token_split: TokenPercentageSplit,
    ) -> Result<()>;
    async fn estimate_loan(
        &self,
        vault_id: String,
        token_split: TokenPercentageSplit,
        target_ratio: Option<u64>,
    ) -> Result<()>;
    async fn estimate_vault(&self) -> Result<VaultEstimation>;
    async fn get_vault(&self, vault_id: String, verbose: Option<bool>) -> Result<VaultResult>;
    async fn list_auction_history(
        &self,
        owner: Option<String>,
        pagination: Option<ListAuctionHistoryPagination>,
    ) -> Result<Vec<ListAuctionHistoryDetail>>;
    async fn list_auctions(&self, pagination: Option<AuctionPagination>) -> Result<Vec<VaultLiquidation>>;
    async fn list_vaults(
        &self,
        options: ListVaultOptions,
        pagination: VaultPagination,
    ) -> Result<Vec<VaultResult>>;
    async fn place_auction_bid(
        &self,
        place_auction_bid: PlaceAuctionBid,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn update_vault(
        &self,
        vault_id: String,
        vault: UpdateVault,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn withdraw_from_vault(
        &self,
        withdraw_vault: WithdrawVault,
        utxos: Option<UTXO>,
    ) -> Result<String>;
}

#[async_trait]
impl VaultRPC for Client {
    async fn close_vault(&self, close_vault: CloseVault, utxos: Option<UTXO>) -> Result<String> {
        self.call("closevault", &[into_json(close_vault)?, into_json(utxos)?]).await
    }
    async fn create_vault(&self, vault: CreateVault, utxos: Option<UTXO>) -> Result<String> {
        self.call("createvault", &[into_json(vault)?, into_json(utxos)?]).await
    }
    async fn deposit_to_vault(
        &self,
        deposit_vault: DepositVault,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("deposittovault", &[into_json(deposit_vault)?, into_json(utxos)?]).await
    }
    async fn estimate_collateral(
        &self,
        target_ratio: u64,
        token_split: TokenPercentageSplit,
    ) -> Result<()> {
        self.call("estimatecollateral", &[into_json(target_ratio)?, into_json(token_split)?]).await
    }
    async fn estimate_loan(
        &self,
        vault_id: String,
        token_split: TokenPercentageSplit,
        target_ratio: Option<u64>,
    ) -> Result<()> {
        self.call(
            "estimateloan",
            &[into_json(vault_id)?, into_json(token_split)?, into_json(target_ratio)?],
        )
        .await
    }
    async fn estimate_vault(&self) -> Result<VaultEstimation> {
        self.call("estimatevault", &[]).await
    }
    async fn get_vault(&self, vault_id: String, verbose: Option<bool>) -> Result<VaultResult> {
        self.call("getvault", &[into_json(vault_id)?, into_json(verbose.unwrap_or_default())?])
            .await
    }
    async fn list_auction_history(
        &self,
        owner: Option<String>,
        pagination: Option<ListAuctionHistoryPagination>,
    ) -> Result<Vec<ListAuctionHistoryDetail>> {
        self.call("listauctionhistory", &[into_json(owner)?, into_json(pagination)?]).await
    }
    async fn list_auctions(
        &self,
        pagination: Option<AuctionPagination>
    ) -> Result<Vec<VaultLiquidation>> {
        self.call("listauctions", &[obj_into_json(pagination)?]).await
    }
    async fn list_vaults(
        &self,
        options: ListVaultOptions,
        pagination: VaultPagination,
    ) -> Result<Vec<VaultResult>> {
        self.call("listvaults", &[into_json(options)?, into_json(pagination)?]).await
    }
    async fn place_auction_bid(
        &self,
        place_auction_bid: PlaceAuctionBid,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("placeauctionbid", &[into_json(place_auction_bid)?, into_json(utxos)?]).await
    }
    async fn update_vault(
        &self,
        vault_id: String,
        vault: UpdateVault,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("updatevault", &[into_json(vault_id)?, into_json(vault)?, into_json(utxos)?])
            .await
    }
    async fn withdraw_from_vault(
        &self,
        withdraw_vault: WithdrawVault,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("withdrawfromvault", &[into_json(withdraw_vault)?, into_json(utxos)?]).await
    }
}
