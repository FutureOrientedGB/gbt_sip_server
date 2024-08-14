use regex::Regex;

use rsip;

use crate::sip::handler::base::SipRequestHandler;
use crate::store::base::StoreEngine;

pub mod on_keep_alive;

impl SipRequestHandler {
    pub async fn on_message(
        &mut self,
        store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: std::net::SocketAddr,
        request: rsip::Request,
    ) -> rsip::Response {
        // decode body
        let msg = self.decode_body(&request);

        // dispatch
        let cmd_type = self.extract_cmd_type(&msg);
        match cmd_type.as_str() {
            "Keepalive" => {
                return self.on_keep_alive(store_engine, sip_socket, client_addr, request, msg).await;
            }
            _ => {
                return rsip::Response::default();
            }
        }
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