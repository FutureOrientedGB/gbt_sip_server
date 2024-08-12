use rsip;

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
        algorithm: rsip::headers::auth::Algorithm,
        nonce: &str,
        cnonce: &str,
        realm: &str,
    ) -> Self {
        SipRequestHandler {
            user_name: String::from(user_name),
            password: String::from(password),
            algorithm: algorithm,
            nonce: String::from(nonce),
            cnonce: String::from(cnonce),
            realm: String::from(realm),
        }
    }

    pub fn is_authorized(
        &self,
        method: &rsip::Method,
        uri: &rsip::Uri,
        digest: &String,
    ) -> bool {
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
