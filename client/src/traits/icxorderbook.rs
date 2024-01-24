pub trait ICXOrderBookRPC: RpcApi {
    fn claim_dfchtlc(
        &self,
        dfchtlc_tx_id: String,
        seed: String,
        utxos: Option<UTXO>,
    ) -> Result<ICXGenericResult>;
    fn close_offer(&self, offer_tx: String, utxos: Option<UTXO>) -> Result<ICXGenericResult>;
    fn close_order(&self, order_tx: String, utxos: Option<UTXO>) -> Result<ICXGenericResult>;
    fn create_order(&self, order: ICXOrder, utxos: Option<UTXO>) -> Result<ICXGenericResult>;
    fn get_order(&self, order_tx: String) -> Result<Record>;
    fn list_htl_cs(&self, options: ICXListHTLCOptions) -> Result<Record>;
    fn list_orders(&self) -> Result<Record>;
    fn make_offer(&self, offer: ICXOffer, utxos: Option<UTXO>) -> Result<ICXGenericResult>;
    fn submit_dfchtlc(&self, htlc: HTLC, utxos: Option<UTXO>) -> Result<ICXGenericResult>;
    fn submit_ext_htlc(&self, htlc: ExtHTLC, utxos: Option<UTXO>) -> Result<ICXGenericResult>;
}
