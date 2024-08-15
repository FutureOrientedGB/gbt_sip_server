use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipRequestHandler;

impl SipRequestHandler {
    pub fn from_old(request: &rsip::Request, id: &String, domain: &String) -> rsip::headers::From {
        let from_uri = format!("sip:{}@{}", id, domain);
        let from = request.from_header().unwrap().typed().unwrap();
        from.with_uri(rsip::Uri::try_from(from_uri).unwrap())
            .with_tag(Self::tag_get(&request).into())
            .into()
    }

    pub fn from_new(id: &String, domain: &String) -> rsip::headers::From {
        let from_uri = format!("sip:{}@{}", id, domain);
        rsip::typed::From {
            display_name: None,
            uri: rsip::Uri::try_from(from_uri).unwrap(),
            params: Default::default(),
        }
        .with_tag(Self::tag_new(8).into())
        .into()
    }
}
