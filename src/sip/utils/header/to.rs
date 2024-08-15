use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipRequestHandler;

impl SipRequestHandler {
    pub fn to_old(&self, request: &rsip::Request) -> rsip::headers::To {
        let to = request.to_header().unwrap().typed().unwrap();
        to.with_tag(Self::tag_new(8).into()).into()
    }

    pub fn to_new(gb_code: &String, domain: &String) -> rsip::headers::To {
        let from_uri = format!("sip:{}@{}", gb_code, domain);
        rsip::typed::To {
            display_name: None,
            uri: rsip::Uri::try_from(from_uri).unwrap(),
            params: Default::default(),
        }
        .with_tag(Self::tag_new(8).into())
        .into()
    }
}
