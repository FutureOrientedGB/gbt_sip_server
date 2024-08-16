use rsip as sip_rs;

use crate::{sip::handler::base::SipHandler, store::base::StoreEngine};

impl SipHandler {
    pub async fn on_subscribe(
        &mut self,
        _store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _client_addr: std::net::SocketAddr,
        _request: sip_rs::Request,
    ) {
        
    }
}
