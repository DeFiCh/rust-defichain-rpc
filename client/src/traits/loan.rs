use crate::{into_json, Client, Result, RpcApi};
use defichain_rpc_json::common::{Interest, UTXO};
use defichain_rpc_json::loan::*;

pub trait LoanRPC: RpcApi {
    fn create_loan_scheme(&self, scheme: CreateLoanScheme, utxos: Option<UTXO>) -> Result<String>;
    fn destroy_loan_scheme(&self, scheme: DestroyLoanScheme, utxos: Option<UTXO>)
        -> Result<String>;
    fn get_collateral_token(&self, token: String) -> Result<CollateralTokenDetail>;
    fn get_interest(&self, id: String, token: Option<String>) -> Result<Vec<Interest>>;
    fn get_loan_info(&self) -> Result<GetLoanInfoResult>;
    fn get_loan_scheme(&self, id: String) -> Result<GetLoanSchemeResult>;
    fn get_loan_token(&self, token: String) -> Result<LoanTokenResult>;
    fn list_collateral_tokens(&self) -> Result<Vec<CollateralTokenDetail>>;
    fn list_loan_schemes(&self) -> Result<Vec<LoanSchemeResult>>;
    fn list_loan_tokens(&self) -> Result<Vec<LoanTokenResult>>;
    fn payback_loan(&self, utxos: Option<UTXO>) -> Result<String>;
    fn payback_with_collateral(&self, vault_id: String) -> Result<String>;
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
}
impl LoanRPC for Client {
    fn create_loan_scheme(&self, scheme: CreateLoanScheme, utxos: Option<UTXO>) -> Result<String> {
        self.call("createloanscheme", &[into_json(scheme)?, into_json(utxos)?])
    }
    fn destroy_loan_scheme(
        &self,
        scheme: DestroyLoanScheme,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("destroyloanscheme", &[into_json(scheme)?, into_json(utxos)?])
    }
    fn get_collateral_token(&self, token: String) -> Result<CollateralTokenDetail> {
        self.call("getcollateraltoken", &[into_json(token)?])
    }
    fn get_interest(&self, id: String, token: Option<String>) -> Result<Vec<Interest>> {
        self.call("getinterest", &[into_json(id)?, into_json(token)?])
    }
    fn get_loan_info(&self) -> Result<GetLoanInfoResult> {
        self.call("getloaninfo", &[])
    }
    fn get_loan_scheme(&self, id: String) -> Result<GetLoanSchemeResult> {
        self.call("getloanscheme", &[into_json(id)?])
    }
    fn get_loan_token(&self, token: String) -> Result<LoanTokenResult> {
        self.call("getloantoken", &[into_json(token)?])
    }
    fn list_collateral_tokens(&self) -> Result<Vec<CollateralTokenDetail>> {
        self.call("listcollateraltokens", &[])
    }
    fn list_loan_schemes(&self) -> Result<Vec<LoanSchemeResult>> {
        self.call("listloanschemes", &[])
    }
    fn list_loan_tokens(&self) -> Result<Vec<LoanTokenResult>> {
        self.call("listloantokens", &[])
    }
    fn payback_loan(&self, utxos: Option<UTXO>) -> Result<String> {
        self.call("paybackloan", &[into_json(utxos)?])
    }
    fn payback_with_collateral(&self, vault_id: String) -> Result<String> {
        self.call("paybackwithcollateral", &[into_json(vault_id)?])
    }
    fn set_collateral_token(
        &self,
        collateral_token: SetCollateralToken,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call("setcollateraltoken", &[into_json(collateral_token)?, into_json(utxos)?])
    }
    fn set_default_loan_scheme(&self, id: String, utxos: Option<UTXO>) -> Result<String> {
        self.call("setdefaultloanscheme", &[into_json(id)?, into_json(utxos)?])
    }
    fn set_loan_token(&self, loan_token: SetLoanToken, utxos: Option<UTXO>) -> Result<String> {
        self.call("setloantoken", &[into_json(loan_token)?, into_json(utxos)?])
    }
    fn take_loan(&self, metadata: TakeLoanMetadata, utxos: Option<UTXO>) -> Result<String> {
        self.call("takeloan", &[into_json(metadata)?, into_json(utxos)?])
    }
    fn update_loan_scheme(&self, scheme: UpdateLoanScheme, utxos: Option<UTXO>) -> Result<String> {
        self.call("updateloanscheme", &[into_json(scheme)?, into_json(utxos)?])
    }
    fn update_loan_token(
        &self,
        old_token: String,
        new_token_details: UpdateLoanToken,
        utxos: Option<UTXO>,
    ) -> Result<String> {
        self.call(
            "updateloantoken",
            &[into_json(old_token)?, into_json(new_token_details)?, into_json(utxos)?],
        )
    }
}
