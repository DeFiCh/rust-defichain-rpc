use bitcoin::Txid;
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProposalType {
    CommunityFundProposal,
    VoteOfConfidence,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProposalStatus {
    Voting,
    Rejected,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ListProposalsType {
    Cfp,
    Voc,
    #[default]
    All,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ListProposalsStatus {
    Voting,
    Rejected,
    Completed,
    #[default]
    All,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoteDecision {
    Yes,
    No,
    Neutral,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VoteResult {
    Yes,
    No,
    Neutral,
    #[serde(rename = "Unknown")]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MasternodeType {
    Mine,
    All,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CFPData {
    title: String,
    context: String,
    context_hash: Option<String>,
    amount: i64,
    payout_address: String,
    cycles: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VOCData {
    title: String,
    context: String,
    context_hash: Option<String>,
    emergency: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalInfo {
    pub proposal_id: String,
    pub title: String,
    pub context: String,
    pub context_hash: String,
    pub r#type: ProposalType,
    pub status: ProposalStatus,
    pub amount: Option<f64>,
    pub current_cycle: u64,
    pub total_cycles: u64,
    pub creation_height: u64,
    pub cycle_end_height: u64,
    pub proposal_end_height: u64,
    pub payout_address: Option<String>,
    pub voting_period: u64,
    pub approval_threshold: String,
    pub quorum: String,
    pub votes_possible: Option<u64>,
    pub votes_present: Option<u64>,
    pub votes_present_pct: Option<String>,
    pub votes_yes: Option<u64>,
    pub votes_invalid: Option<u64>,
    pub votes_neutral: Option<u64>,
    pub votes_no: Option<u64>,
    pub votes_yes_pct: Option<String>,
    pub fee: f64,
    pub options: Option<Vec<String>>,
    pub fee_redistribution_per_vote: Option<f64>,
    pub fee_redistribution_total: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteData {
    proposal_id: String,
    masternode_id: String,
    decision: VoteDecision,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListVotesResult {
    pub proposal_id: String,
    masternode_id: String,
    cycle: u64,
    vote: VoteResult,
    valid: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProposalsOptions {
    pub r#type: Option<ListProposalsType>,
    pub status: Option<ListProposalsStatus>,
    pub cycle: Option<i64>,
    pub pagination: Option<ListProposalsPagination>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListProposalsPagination {
    pub start: Option<String>,
    pub including_start: Option<bool>,
    pub limit: Option<usize>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListGovProposalVotesOptions {
    pub proposal_id: Option<Txid>,
    pub masternode: Option<String>,
    pub cycle: Option<i64>,
    pub pagination: Option<ListGovProposalVotesPagination>,
    pub aggregate: Option<bool>,
    pub valid: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListGovProposalVotesPagination {
    pub start: Option<usize>,
    pub including_start: Option<bool>,
    pub limit: Option<usize>,
}
