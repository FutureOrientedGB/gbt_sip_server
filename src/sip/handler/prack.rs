use rsip as sip_rs;

use super::SipHandler;

impl SipHandler {
    pub async fn on_req_prack(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        _request: sip_rs::Request,
    ) {
    }

    pub async fn on_rsp_prack(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        _response: sip_rs::Response,
    ) {
    }
}
