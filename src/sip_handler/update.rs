use rsip;

use crate::{sip_handler::base::SipRequestHandler, store::base::StoreEngine};

impl SipRequestHandler {
    pub async fn on_update(&mut self, _store_engine: std::sync::Arc<Box<dyn StoreEngine>>, _request: rsip::Request) -> rsip::Response {
        return rsip::Response::default();
    }

}