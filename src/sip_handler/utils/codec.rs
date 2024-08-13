use rand::Rng;

use rsip::{self, prelude::HeadersExt};

use crate::sip_handler::base::SipRequestHandler;

static CHARSET: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

impl SipRequestHandler {
    pub fn random_tag(&self, length: usize) -> String {
        let mut rng = rand::thread_rng();
        std::iter::repeat(())
            .take(length)
            .map(|_| {
                let index = rng.gen_range(0..CHARSET.len());
                CHARSET[index]
            })
            .collect()
    }

    pub fn extract_tag(&self, request: &rsip::Request) -> String {
        if let Ok(to) = request.to_header() {
            if let Ok(tag) = to.tag() {
                if let Some(tag) = tag {
                    return tag.to_string();
                }
            }
        }

        return String::new();
    }
}
