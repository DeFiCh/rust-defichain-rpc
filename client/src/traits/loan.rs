use crate::{into_json, Client, Result, RpcApi};
use async_trait::async_trait;
use defichain_rpc_json::common::{Interest, UTXO};
use defichain_rpc_json::loan::*;

#[async_trait]
pub trait LoanRPC: RpcApi {
    async fn create_loan_scheme(
        &self,
        scheme: CreateLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn destroy_loan_scheme(
        &self,
        scheme: DestroyLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn get_collateral_token(&self, token: String) -> Result<CollateralTokenDetail>;
    async fn get_interest(&self, id: String, token: Option<String>) -> Result<Vec<Interest>>;
    async fn get_loan_info(&self) -> Result<GetLoanInfoResult>;
    async fn get_loan_scheme(&self, id: String) -> Result<GetLoanSchemeResult>;
    async fn get_loan_token(&self, token: String) -> Result<LoanTokenResult>;
    async fn list_collateral_tokens(&self) -> Result<Vec<CollateralTokenDetail>>;
    async fn list_loan_schemes(&self) -> Result<Vec<LoanSchemeResult>>;
    async fn list_loan_tokens(&self) -> Result<Vec<LoanTokenResult>>;
    async fn payback_loan(&self, utxos: Option<UTXO>) -> Result<String>;
    async fn payback_with_collateral(&self, vault_id: String) -> Result<String>;
    async fn set_collateral_token(
        &self,
        collateral_token: SetCollateralToken,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn set_default_loan_scheme(&self, id: String, utxos: Option<UTXO>) -> Result<String>;
    async fn set_loan_token(&self, loan_token: SetLoanToken, utxos: Option<UTXO>)
        -> Result<String>;
    async fn take_loan(&self, metadata: TakeLoanMetadata, utxos: Option<UTXO>) -> Result<String>;
    async fn update_loan_scheme(
        &self,
        scheme: UpdateLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    async fn update_loan_token(
        &self,
        old_token: String,
        new_token_details: UpdateLoanToken,
        utxos: Option<UTXO>,
    ) -> Result<String>;
}

#[async_trait]
impl LoanRPC for Client {
    async fn create_loan_scheme(
        &self,
        scheme: CreateLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("createloanscheme", &[into_json(scheme)?, into_json(utxos)?]).await
    }
    async fn destroy_loan_scheme(
        &self,
        scheme: DestroyLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("destroyloanscheme", &[into_json(scheme)?, into_json(utxos)?]).await
    }
    async fn get_collateral_token(&self, token: String) -> Result<CollateralTokenDetail> {
        self.call("getcollateraltoken", &[into_json(token)?]).await
    }
    async fn get_interest(&self, id: String, token: Option<String>) -> Result<Vec<Interest>> {
        self.call("getinterest", &[into_json(id)?, into_json(token)?]).await
    }
    async fn get_loan_info(&self) -> Result<GetLoanInfoResult> {
        self.call("getloaninfo", &[]).await
    }
    async fn get_loan_scheme(&self, id: String) -> Result<GetLoanSchemeResult> {
        self.call("getloanscheme", &[into_json(id)?]).await
    }
    async fn get_loan_token(&self, token: String) -> Result<LoanTokenResult> {
        self.call("getloantoken", &[into_json(token)?]).await
    }
    async fn list_collateral_tokens(&self) -> Result<Vec<CollateralTokenDetail>> {
        self.call("listcollateraltokens", &[]).await
    }
    async fn list_loan_schemes(&self) -> Result<Vec<LoanSchemeResult>> {
        self.call("listloanschemes", &[]).await
    }
    async fn list_loan_tokens(&self) -> Result<Vec<LoanTokenResult>> {
        self.call("listloantokens", &[]).await
    }
    async fn payback_loan(&self, utxos: Option<UTXO>) -> Result<String> {
        self.call("paybackloan", &[into_json(utxos)?]).await
    }
    async fn payback_with_collateral(&self, vault_id: String) -> Result<String> {
        self.call("paybackwithcollateral", &[into_json(vault_id)?]).await
    }
    async fn set_collateral_token(
        &self,
        collateral_token: SetCollateralToken,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("setcollateraltoken", &[into_json(collateral_token)?, into_json(utxos)?]).await
    }
    async fn set_default_loan_scheme(&self, id: String, utxos: Option<UTXO>) -> Result<String> {
        self.call("setdefaultloanscheme", &[into_json(id)?, into_json(utxos)?]).await
    }
    async fn set_loan_token(
        &self,
        loan_token: SetLoanToken,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("setloantoken", &[into_json(loan_token)?, into_json(utxos)?]).await
    }
    async fn take_loan(&self, metadata: TakeLoanMetadata, utxos: Option<UTXO>) -> Result<String> {
        self.call("takeloan", &[into_json(metadata)?, into_json(utxos)?]).await
    }
    async fn update_loan_scheme(
        &self,
        scheme: UpdateLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("updateloanscheme", &[into_json(scheme)?, into_json(utxos)?]).await
    }
    async fn update_loan_token(
        &self,
        old_token: String,
        new_token_details: UpdateLoanToken,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call(
            "updateloantoken",
            &[into_json(old_token)?, into_json(new_token_details)?, into_json(utxos)?],
        )
        .await
    }
}
