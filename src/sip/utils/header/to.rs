use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub fn to_old(&self, request: &sip_rs::Request) -> sip_rs::headers::To {
        let to = request.to_header().unwrap().typed().unwrap();
        to.with_tag(self.tag_new(8).into()).into()
    }

    pub fn to_new(&self, gb_code: &String) -> sip_rs::headers::To {
        let from_uri = format!("sip:{}@{}", gb_code, self.domain);
        sip_rs::typed::To {
            display_name: None,
            uri: sip_rs::Uri::try_from(from_uri).unwrap(),
            params: Default::default(),
        }
        .with_tag(self.tag_new(8).into())
        .into()
    }
}
