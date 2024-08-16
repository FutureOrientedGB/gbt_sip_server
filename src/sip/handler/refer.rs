use rsip as sip_rs;

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub async fn on_refer(&self, _client_addr: std::net::SocketAddr, _request: sip_rs::Request) {}
}
