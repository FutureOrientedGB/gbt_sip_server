use std::str::FromStr;

use rsip;

use crate::cli::CommandLines;

pub struct SipRequestHandler {
    pub user_name: String,
    pub password: String,
    pub algorithm: rsip::headers::auth::Algorithm,
    pub nonce: String,
    pub cnonce: String,
    pub realm: String,
}

impl SipRequestHandler {
    pub fn new(cli_args: &CommandLines) -> Self {
        SipRequestHandler {
            user_name: cli_args.sip_user_name.clone(),
            password: cli_args.sip_password.clone(),
            algorithm: rsip::headers::auth::Algorithm::from_str(&cli_args.sip_algorithm).unwrap(),
            nonce: cli_args.sip_nonce.clone(),
            cnonce: cli_args.sip_cnonce.clone(),
            realm: cli_args.sip_realm.clone(),
        }
    }
}
