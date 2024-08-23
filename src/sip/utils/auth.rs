use rsip as sip_rs;

use crate::sip::handler::SipHandler;

impl SipHandler {
    pub fn is_authorized(&self, user_name: &String, method: &sip_rs::Method, uri: &sip_rs::Uri, digest: &String) -> bool {
        let generator = sip_rs::services::DigestGenerator {
            username: user_name,
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
}
