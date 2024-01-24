pub trait SpvRPC: RpcApi {
    fn claim_htlc(
        &self,
        script_address: String,
        destination_address: String,
        options: ClaimHtlcOptions,
    ) -> Result<SendMessageResult>;
    fn create_anchor(&self, reward_address: String) -> Result<CreateAnchorResult>;
    fn create_htlc(
        &self,
        receiver_pub_key: String,
        owner_pub_key: String,
        options: CreateHtlcOptions,
    ) -> Result<CreateHtlcResult>;
    fn decode_htlc_script(&self, redeem_script: String) -> Result<DecodeHtlcResult>;
    fn get_address_pub_key(&self, address: String) -> Result<String>;
    fn get_htlc_seed(&self, address: String) -> Result<String>;
    fn get_new_address(&self) -> Result<String>;
    fn list_anchor_auths(&self) -> Result<Vec<ListAnchorAuthsResult>>;
    fn list_anchor_reward_confirms(&self) -> Result<Vec<ListAnchorRewardConfirmsResult>>;
    fn list_anchor_rewards(&self) -> Result<Vec<ListAnchorRewardsResult>>;
    fn list_anchors(&self) -> Result<Vec<ListAnchorsResult>>;
    fn list_anchors_pending(&self) -> Result<Vec<ListAnchorsResult>>;
    fn list_anchors_unrewarded(&self) -> Result<Vec<ListAnchorsResult>>;
    fn list_htlc_outputs(
        &self,
        script_address: Option<String>,
    ) -> Result<Vec<ListHtlcsOutputsResult>>;
    fn list_received_by_address(
        &self,
        address: Option<String>,
    ) -> Result<Vec<ReceivedByAddressInfo>>;
    fn refund_htlc(
        &self,
        script_address: String,
        destination_address: String,
    ) -> Result<SendMessageResult>;
    fn refund_htlc_all(&self, destination_address: String) -> Result<()>;
    fn send_to_address(&self, address: String, amount: BigNumber) -> Result<SendMessageResult>;
    fn set_last_height(&self, height: u64) -> Result<()>;
}
