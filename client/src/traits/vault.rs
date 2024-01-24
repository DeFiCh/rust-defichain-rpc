pub trait VaultRPC {
    fn close_vault(&self, close_vault: CloseVault, utxos: Option<UTXO>) -> Result<String>;
    fn create_vault(&self, vault: CreateVault, utxos: Option<UTXO>) -> Result<String>;
    fn deposit_to_vault(&self, deposit_vault: DepositVault, utxos: Option<UTXO>) -> Result<String>;
    fn estimate_collateral(&self, target_ratio: u64) -> Result<()>;
    fn estimate_loan(
        &self,
        vault_id: String,
        token_split: TokenPercentageSplit,
        target_ratio: Option<u64>,
    ) -> Result<()>;
    fn estimate_vault(&self) -> Result<VaultEstimation>;
    fn get_vault(&self, vault_id: String) -> Result<()>;
    fn list_auction_history(
        &self,
        pagination: Option<ListAuctionHistoryPagination>,
    ) -> Result<Vec<ListAuctionHistoryDetail>>;
    fn list_auctions(&self) -> Result<Vec<VaultLiquidation>>;
    fn list_vaults(&self) -> Result<Array>;
    fn place_auction_bid(
        &self,
        place_auction_bid: PlaceAuctionBid,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn update_vault(
        &self,
        vault_id: String,
        vault: UpdateVault,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn withdraw_from_vault(
        &self,
        withdraw_vault: WithdrawVault,
        utxos: Option<UTXO>,
    ) -> Result<String>;
}
