use encoding_rs;

use rand::Rng;

use rsip::{self, prelude::HeadersExt};

use tracing;

use crate::sip_handler::base::SipRequestHandler;

static CHARSET: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

impl SipRequestHandler {
    pub fn is_authorized(&self, method: &rsip::Method, uri: &rsip::Uri, digest: &String) -> bool {
        let generator = rsip::services::DigestGenerator {
            username: &self.user_name,
            password: &self.password,
            algorithm: self.algorithm,
            nonce: &self.nonce,
            method: method,
            qop: None,
            uri: &uri,
            realm: &self.realm,
        };

        return generator.verify(digest);
    }

    pub fn random_tag(&self, length: usize) -> String {
        let mut rng = rand::thread_rng();
        std::iter::repeat(())
            .take(length)
            .map(|_| {
                let index = rng.gen_range(0..CHARSET.len());
                CHARSET[index]
            })
            .collect()
    }

    pub fn extract_tag(&self, request: &rsip::Request) -> String {
        if let Ok(to) = request.to_header() {
            if let Ok(tag) = to.tag() {
                if let Some(tag) = tag {
                    return tag.to_string();
                }
            }
        }

        return String::new();
    }

    pub fn decode_body(&self, request: &rsip::Request) -> String {
        let (body, _encoding, has_error) = encoding_rs::GB18030.decode(&request.body());
        if has_error {
            tracing::error!("encoding_rs::GB18030.decode error");
            return String::new();
        }

        return body.to_string();
    }

    pub fn encode_body(&self, data: String) -> Vec<u8> {
        let (msg, _encoding, has_error) = encoding_rs::GB18030.encode(&data);
        if has_error {
            tracing::error!("encoding_rs::GB18030.encode error");
            return vec![];
        }

        return msg.to_vec();
    }
}
