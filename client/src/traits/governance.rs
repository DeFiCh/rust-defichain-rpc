pub trait GovernanceRPC {
    fn create_gov_cfp(&self, data: CFPData, utxos: Option<UTXO>) -> Result<String>;
    fn create_gov_voc(&self, data: VOCData, utxos: Option<UTXO>) -> Result<String>;
    fn get_gov_proposal(&self, proposal_id: String) -> Result<ProposalInfo>;
    fn list_gov_proposal_votes(
        &self,
        options: Option<ListGovProposalVotesOptions>,
    ) -> Result<Vec<ListVotesResult>>;
    fn list_gov_proposals(&self) -> Result<Vec<ProposalInfo>>;
    fn vote_gov(&self, data: VoteData, utxos: Option<UTXO>) -> Result<String>;
}
