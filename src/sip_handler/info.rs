use rsip;

use crate::sip_handler::base::SipRequestHandler;

impl SipRequestHandler {

    pub async fn on_info(&mut self, request: rsip::Request) -> rsip::Response {
        return rsip::Response::default();
    }
}