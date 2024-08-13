use rand::Rng;

use rsip::{self, prelude::HeadersExt};

static CHARSET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct SipRequestHandler {
    pub user_name: String,
    pub password: String,
    pub algorithm: rsip::headers::auth::Algorithm,
    pub nonce: String,
    pub cnonce: String,
    pub realm: String,
}

impl SipRequestHandler {
    pub fn new(
        user_name: &str,
        password: &str,
        algorithm: &str,
        nonce: &str,
        cnonce: &str,
        realm: &str,
    ) -> Self {
        SipRequestHandler {
            user_name: String::from(user_name),
            password: String::from(password),
            algorithm: rsip::headers::auth::Algorithm::try_from(algorithm).unwrap(),
            nonce: String::from(nonce),
            cnonce: String::from(cnonce),
            realm: String::from(realm),
        }
    }

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
}
