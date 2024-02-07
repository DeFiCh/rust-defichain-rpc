use async_trait::async_trait;
use defichain_rpc_json::{bitcoin::Txid, governance::*};

use crate::{into_json, Client, Result, RpcApi};

#[async_trait]
pub trait GovernanceRPC: RpcApi {
    // async fn create_gov_cfp(&self, data: CFPData, utxos: Option<UTXO>) -> Result<String>;
    // async fn create_gov_voc(&self, data: VOCData, utxos: Option<UTXO>) -> Result<String>;
    async fn get_gov_proposal(&self, proposal_id: Txid) -> Result<ProposalInfo>;
    async fn list_gov_proposal_votes(
        &self,
        options: Option<ListGovProposalVotesOptions>,
    ) -> Result<Vec<ListVotesResult>>;
    async fn list_gov_proposals(
        &self,
        opts: Option<ListProposalsOptions>,
    ) -> Result<Vec<ProposalInfo>>;
    // async fn vote_gov(&self, data: VoteData, utxos: Option<UTXO>) -> Result<String>;
}

#[async_trait]
impl GovernanceRPC for Client {
    async fn get_gov_proposal(&self, proposal_id: Txid) -> Result<ProposalInfo> {
        self.call("getgovproposal", &[into_json(proposal_id)?]).await
    }
    async fn list_gov_proposals(
        &self,
        opts: Option<ListProposalsOptions>,
    ) -> Result<Vec<ProposalInfo>> {
        self.call("listgovproposals", &[into_json(opts)?]).await
    }
    async fn list_gov_proposal_votes(
        &self,
        opts: Option<ListGovProposalVotesOptions>,
    ) -> Result<Vec<ListVotesResult>> {
        self.call("listgovproposalvotes", &[into_json(opts)?]).await
    }
}
