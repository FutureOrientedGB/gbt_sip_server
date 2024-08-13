use rsip;

use crate::sip_handler::base::SipRequestHandler;

impl SipRequestHandler {

    pub async fn on_subscribe(&mut self, request: rsip::Request) -> rsip::Response {
        return rsip::Response::default();
    }

}