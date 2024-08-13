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
            user_name: cli_args.user_name.clone(),
            password: cli_args.password.clone(),
            algorithm: rsip::headers::auth::Algorithm::from_str(&cli_args.algorithm).unwrap(),
            nonce: cli_args.nonce.clone(),
            cnonce: cli_args.cnonce.clone(),
            realm: cli_args.realm.clone(),
        }
    }
}
