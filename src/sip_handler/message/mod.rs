use encoding_rs;

use regex::Regex;

use rsip;

use crate::sip_handler::base::SipRequestHandler;

pub mod on_keep_alive;

impl SipRequestHandler {
    pub async fn on_message(&mut self, request: rsip::Request) -> rsip::Response {
        // decode body
        let msg = self.decode_body(&request);

        // dispatch
        let cmd_type = self.extract_cmd_type(&msg);
        match cmd_type.as_str() {
            "Keepalive" => {
                return self.on_keep_alive(request, msg).await;
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
