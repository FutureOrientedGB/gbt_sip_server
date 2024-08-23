use rsip::{self as sip_rs, prelude::UntypedHeader};

use uuid::Uuid;

use crate::sip::handler::SipHandler;

impl SipHandler {
    pub fn caller_id_new(&self) -> sip_rs::headers::CallId {
        format!(
            "{}@{}:{}",
            Uuid::new_v4().to_string().replace("-", "").to_uppercase(),
            self.ip,
            self.port
        )
        .into()
    }

    pub fn caller_id_str(&self) -> String {
        format!(
            "{}@{}:{}",
            Uuid::new_v4().to_string().replace("-", "").to_uppercase(),
            self.ip,
            self.port
        )
    }

    pub fn caller_id_from_str(&self, s: &String) -> sip_rs::headers::CallId {
        sip_rs::headers::CallId::new(s)
    }
}
