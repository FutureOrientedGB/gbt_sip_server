use rsip as sip_rs;

use crate::sip::handler::SipHandler;

impl SipHandler {
    pub fn from_new(&self) -> sip_rs::headers::From {
        let from_uri = format!("sip:{}@{}", self.id, self.domain);
        sip_rs::typed::From {
            display_name: None,
            uri: sip_rs::Uri::try_from(from_uri).unwrap(),
            params: Default::default(),
        }
        .with_tag(self.tag_new(10).into())
        .into()
    }
}
