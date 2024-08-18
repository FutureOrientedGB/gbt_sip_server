use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub fn from_old(&self, request: &sip_rs::Request) -> sip_rs::headers::From {
        let from_uri = format!("sip:{}@{}", self.id, self.domain);
        let from = request.from_header().unwrap().typed().unwrap();
        from.with_uri(sip_rs::Uri::try_from(from_uri).unwrap())
            .with_tag(self.tag_get(&request).into())
            .into()
    }

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
