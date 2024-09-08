use rsip as sip_rs;

use super::SipHandler;

impl SipHandler {
    pub async fn on_req_refer(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        _request: sip_rs::Request,
    ) {
    }

    pub async fn on_rsp_refer(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        _response: sip_rs::Response,
    ) {
    }
}
