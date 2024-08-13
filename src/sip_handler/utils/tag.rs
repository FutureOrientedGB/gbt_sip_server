use encoding_rs;

use rsip;

use tracing;

use crate::sip_handler::base::SipRequestHandler;

impl SipRequestHandler {
    pub fn decode_body(&self, request: &rsip::Request) -> String {
        let (body, _encoding, has_error) = encoding_rs::GB18030.decode(&request.body());
        if has_error {
            tracing::error!("encoding_rs::GB18030.decode error");
            return String::new();
        }

        if body.find(r#"encoding="GB2312""#).is_some() {
            return body.replace(r#"encoding="GB2312""#, r#"encoding="utf-8""#);
        } else if body.find(r#"encoding="GB18030""#).is_some() {
            return body.replace(r#"encoding="GB18030""#, r#"encoding="utf-8""#);
        }
        return body.to_string();
    }

    pub fn encode_body(&self, data: String) -> Vec<u8> {
        let s = data.replace(r#"encoding="utf-8""#, r#"encoding="GB18030""#);
        let (msg, _encoding, has_error) = encoding_rs::GB18030.encode(&s);
        if has_error {
            tracing::error!("encoding_rs::GB18030.encode error");
            return vec![];
        }

        return msg.to_vec();
    }
}
