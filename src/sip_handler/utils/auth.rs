use rsip;

use crate::sip_handler::base::SipRequestHandler;

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
}
