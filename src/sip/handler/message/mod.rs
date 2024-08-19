pub mod on_device_status;
pub mod on_keep_alive;

use regex::Regex;

use rsip as sip_rs;

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub async fn on_req_message(
        &self,
        device_addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        request: sip_rs::Request,
    ) {
        // decode body
        let msg = self.decode_body(request.body());

        // dispatch
        let cmd_type = self.extract_cmd_type(&msg);
        match cmd_type.as_str() {
            "Keepalive" => {
                self.on_keep_alive(device_addr, tcp_stream, request, msg)
                    .await;
            }
            "DeviceStatus" => {
                self.on_device_status(device_addr, tcp_stream, request, msg)
                    .await;
            }
            _ => {}
        }
    }

    pub async fn on_rsp_message(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        _response: sip_rs::Response,
    ) {
    }

    fn extract_cmd_type(&self, body: &String) -> String {
        let regex = Regex::new(r"<CmdType>(\w+)</CmdType>").unwrap();
        if let Some(matches) = regex.captures(&body) {
            if let Some(s) = matches.get(1).map(|m| m.as_str()) {
                return String::from(s);
            }
        }

        return String::new();
    }
}
