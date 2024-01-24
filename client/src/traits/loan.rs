pub trait LoanRPC {
    fn close_vault(&self, close_vault: CloseVault, utxos: Option<UTXO>) -> Result<String>;
    fn create_loan_scheme(&self, scheme: CreateLoanScheme, utxos: Option<UTXO>) -> Result<String>;
    fn create_vault(&self, vault: CreateVault, utxos: Option<UTXO>) -> Result<String>;
    fn deposit_to_vault(&self, deposit_vault: DepositVault, utxos: Option<UTXO>) -> Result<String>;
    fn destroy_loan_scheme(&self, scheme: DestroyLoanScheme, utxos: Option<UTXO>)
        -> Result<String>;
    fn get_collateral_token(&self, token: String) -> Result<CollateralTokenDetail>;
    fn get_interest(&self, id: String, token: Option<String>) -> Result<Vec<Interest>>;
    fn get_loan_info(&self) -> Result<GetLoanInfoResult>;
    fn get_loan_scheme(&self, id: String) -> Result<GetLoanSchemeResult>;
    fn get_loan_token(&self, token: String) -> Result<LoanTokenResult>;
    fn get_vault(&self, vault_id: String) -> Result<()>;
    fn list_auction_history(
        &self,
        pagination: Option<ListAuctionHistoryPagination>,
    ) -> Result<Vec<ListAuctionHistoryDetail>>;
    fn list_auctions(&self) -> Result<Vec<VaultLiquidation>>;
    fn list_collateral_tokens(&self) -> Result<Vec<CollateralTokenDetail>>;
    fn list_loan_schemes(&self) -> Result<Vec<LoanSchemeResult>>;
    fn list_loan_tokens(&self) -> Result<Vec<LoanTokenResult>>;
    fn list_vaults(&self) -> Result<Array>;
    fn payback_loan(&self, utxos: Option<UTXO>) -> Result<String>;
    fn payback_with_collateral(&self, vault_id: String) -> Result<String>;
    fn place_auction_bid(
        &self,
        place_auction_bid: PlaceAuctionBid,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn set_collateral_token(
        &self,
        collateral_token: SetCollateralToken,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn set_default_loan_scheme(&self, id: String, utxos: Option<UTXO>) -> Result<String>;
    fn set_loan_token(&self, loan_token: SetLoanToken, utxos: Option<UTXO>) -> Result<String>;
    fn take_loan(&self, metadata: TakeLoanMetadata, utxos: Option<UTXO>) -> Result<String>;
    fn update_loan_scheme(&self, scheme: UpdateLoanScheme, utxos: Option<UTXO>) -> Result<String>;
    fn update_loan_token(
        &self,
        old_token: String,
        new_token_details: UpdateLoanToken,
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
