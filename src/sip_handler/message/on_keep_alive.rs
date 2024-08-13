use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip_handler::base::SipRequestHandler;

impl SipRequestHandler {
    pub async fn on_keep_alive(&mut self, request: rsip::Request) -> rsip::Response {
        let mut headers: rsip::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(request.from_header().unwrap().clone().into());
        let to = request.to_header().unwrap().typed().unwrap();
        headers.push(to.with_tag(self.random_tag(8).into()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(rsip::Header::ContentLength(Default::default()));

        rsip::Response {
            status_code: rsip::StatusCode::OK,
            headers,
            version: rsip::Version::V2,
            body: Default::default(),
        }
    }
}
