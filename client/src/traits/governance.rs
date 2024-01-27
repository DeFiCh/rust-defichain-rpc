use defichain_rpc_json::{bitcoin::Txid, governance::*};

use crate::{into_json, Client, Result, RpcApi};

pub trait GovernanceRPC: RpcApi {
    // fn create_gov_cfp(&self, data: CFPData, utxos: Option<UTXO>) -> Result<String>;
    // fn create_gov_voc(&self, data: VOCData, utxos: Option<UTXO>) -> Result<String>;
    fn get_gov_proposal(&self, proposal_id: Txid) -> Result<ProposalInfo>;
    fn list_gov_proposal_votes(
        &self,
        options: Option<ListGovProposalVotesOptions>,
    ) -> Result<Vec<ListVotesResult>>;
    fn list_gov_proposals(&self, opts: Option<ListProposalsOptions>) -> Result<Vec<ProposalInfo>>;
    // fn vote_gov(&self, data: VoteData, utxos: Option<UTXO>) -> Result<String>;
}

impl GovernanceRPC for Client {
    fn get_gov_proposal(&self, proposal_id: Txid) -> Result<ProposalInfo> {
        self.call("getgovproposal", &[into_json(proposal_id)?])
    }
    fn list_gov_proposals(&self, opts: Option<ListProposalsOptions>) -> Result<Vec<ProposalInfo>> {
        self.call("listgovproposals", &[into_json(opts)?])
    }
    fn list_gov_proposal_votes(
        &self,
        opts: Option<ListGovProposalVotesOptions>,
    ) -> Result<Vec<ListVotesResult>> {
        self.call("listgovproposalvotes", &[into_json(opts)?])
    }
}
