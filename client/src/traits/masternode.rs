pub trait MasternodeRPC {
    fn clear_mempool(&self) -> Result<()>;
    fn create_masternode(
        &self,
        owner_address: String,
        operator_address: Option<String>,
    ) -> Result<String>;
    fn get_active_masternode_count(&self) -> Result<u64>;
    fn get_anchor_teams(&self, block_height: Option<u64>) -> Result<AnchorTeamResult>;
    fn get_gov(&self, name: String) -> Result<Record>;
    fn get_masternode(&self, masternode_id: String) -> Result<MasternodeResult>;
    fn get_masternode_blocks(
        &self,
        identifier: MasternodeBlock,
        depth: Option<u64>,
    ) -> Result<MasternodeResult>;
    fn is_applied_custom_transaction(
        &self,
        transaction_id: String,
        block_height: u64,
    ) -> Result<bool>;
    fn list_anchors(&self) -> Result<MasternodeResult>;
    fn list_govs(&self) -> Result<Array>;
    fn list_masternodes(
        &self,
        pagination: Option<MasternodePagination>,
        verbose: Option<bool>,
    ) -> Result<MasternodeResult>;
    fn resign_masternode(&self, masternode_id: String, utxos: Option<UTXO>) -> Result<String>;
    fn set_gov(&self, input: Record, utxos: Option<UTXO>) -> Result<String>;
    fn set_gov_height(
        &self,
        input: Record,
        activation_height: u64,
        utxos: Option<UTXO>,
    ) -> Result<String>;
    fn unset_gov(&self, variables: Record, utxos: Option<UTXO>) -> Result<String>;
    fn update_masternode(
        &self,
        masternode_id: String,
        values: UpdateMasternodeValues,
        utxos: Option<UTXO>,
    ) -> Result<String>;
}
