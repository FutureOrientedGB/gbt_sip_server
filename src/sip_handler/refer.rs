use rsip;

use crate::sip_handler::internal::SipRequestHandler;

impl SipRequestHandler {
   

    pub async fn on_refer(&mut self, request: rsip::Request) -> String {
        return String::new();
    }
}