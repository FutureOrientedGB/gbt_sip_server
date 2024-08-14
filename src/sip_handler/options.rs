use rsip;

use crate::{sip_handler::base::SipRequestHandler, store::base::StoreEngine};

impl SipRequestHandler {
    pub async fn on_options(
        &mut self,
        _store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _client_addr: std::net::SocketAddr,
        _request: rsip::Request,
    ) -> rsip::Response {
        return rsip::Response::default();
    }
}
