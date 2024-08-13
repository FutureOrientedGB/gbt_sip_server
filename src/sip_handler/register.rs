use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip_handler::internal::SipRequestHandler;

impl SipRequestHandler {
    pub async fn on_register(&mut self, request: rsip::Request) -> String {
        if let Some(auth) = request.authorization_header() {
            if let Ok(auth) = auth.typed() {
                if self.is_authorized(request.method(), &auth.uri, &auth.response) {
                    return self.on_register_200(request).await;
                }
            }
        }

        return self.on_register_401(request).await;
    }

    async fn on_register_401(&self, request: rsip::Request) -> String {
        let mut headers: rsip::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(request.from_header().unwrap().clone().into());
        let to = request.to_header().unwrap().typed().unwrap();
        headers.push(to.with_tag(self.random_tag(10).into()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(rsip::Header::ContentLength(Default::default()));

        headers.push(
            rsip::typed::WwwAuthenticate {
                realm: self.realm.clone(),
                nonce: self.nonce.clone(),
                algorithm: Some(self.algorithm),
                opaque: Some("".into()),
                ..Default::default()
            }
            .into(),
        );

        let response = rsip::Response {
            status_code: rsip::StatusCode::Unauthorized,
            headers,
            version: rsip::Version::V2,
            body: Default::default(),
        };

        return response.to_string();
    }

    async fn on_register_200(&self, request: rsip::Request) -> String {
        let mut headers: rsip::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(request.from_header().unwrap().clone().into());
        let to = request.to_header().unwrap().typed().unwrap();
        headers.push(to.with_tag(self.random_tag(8).into()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(rsip::Header::ContentLength(Default::default()));

        let response = rsip::Response {
            status_code: rsip::StatusCode::OK,
            headers,
            version: rsip::Version::V2,
            body: Default::default(),
        };

        return response.to_string();
    }
}
