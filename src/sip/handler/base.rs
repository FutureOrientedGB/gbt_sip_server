use std::str::FromStr;

use rsip;

use crate::utils::cli::CommandLines;

pub struct SipRequestHandler {
    pub domain: String,
    pub id: String,
    pub password: String,
    pub algorithm: rsip::headers::auth::Algorithm,
    pub nonce: String,
    pub realm: String,
}

impl SipRequestHandler {
    pub fn new(cli_args: &CommandLines) -> Self {
        SipRequestHandler {
            domain: cli_args.sip_domain.clone(),
            id: cli_args.sip_id.clone(),
            password: cli_args.sip_password.clone(),
            algorithm: rsip::headers::auth::Algorithm::from_str(&cli_args.sip_algorithm).unwrap(),
            nonce: cli_args.sip_nonce.clone(),
            realm: cli_args.sip_realm.clone(),
        }
    }
}
