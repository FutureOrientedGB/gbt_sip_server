use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub fn from_old(request: &sip_rs::Request, id: &String, domain: &String) -> sip_rs::headers::From {
        let from_uri = format!("sip:{}@{}", id, domain);
        let from = request.from_header().unwrap().typed().unwrap();
        from.with_uri(sip_rs::Uri::try_from(from_uri).unwrap())
            .with_tag(Self::tag_get(&request).into())
            .into()
    }

    pub fn from_new(id: &String, domain: &String) -> sip_rs::headers::From {
        let from_uri = format!("sip:{}@{}", id, domain);
        sip_rs::typed::From {
            display_name: None,
            uri: sip_rs::Uri::try_from(from_uri).unwrap(),
            params: Default::default(),
        }
        .with_tag(Self::tag_new(8).into())
        .into()
    }
}
