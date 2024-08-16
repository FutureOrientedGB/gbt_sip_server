use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub fn to_old(&self, request: &sip_rs::Request) -> sip_rs::headers::To {
        let to = request.to_header().unwrap().typed().unwrap();
        to.with_tag(Self::tag_new(8).into()).into()
    }

    pub fn to_new(gb_code: &String, domain: &String) -> sip_rs::headers::To {
        let from_uri = format!("sip:{}@{}", gb_code, domain);
        sip_rs::typed::To {
            display_name: None,
            uri: sip_rs::Uri::try_from(from_uri).unwrap(),
            params: Default::default(),
        }
        .with_tag(Self::tag_new(8).into())
        .into()
    }
}
