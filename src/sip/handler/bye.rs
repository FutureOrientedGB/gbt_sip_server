use rsip as sip_rs;

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub async fn on_req_bye(&self, _client_addr: std::net::SocketAddr, _request: sip_rs::Request) {}

    pub async fn on_rsp_bye(
        &self,
        _client_addr: std::net::SocketAddr,
        _response: sip_rs::Response,
    ) {
    }
}
