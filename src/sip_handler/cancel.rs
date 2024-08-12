use rsip;

use crate::sip_handler::internal::SipRequestHandler;

impl SipRequestHandler {


    pub async fn on_cancel(&mut self, request: rsip::Request) -> String {
        return String::new();
    }
}