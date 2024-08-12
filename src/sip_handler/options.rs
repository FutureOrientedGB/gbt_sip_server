use rsip;

use crate::sip_handler::internal::SipRequestHander;

impl SipRequestHander {
    pub async fn on_options(&mut self, request: rsip::Request) -> Vec<u8> {
        return vec![];
    }

}