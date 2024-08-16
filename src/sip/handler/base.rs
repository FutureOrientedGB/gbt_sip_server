use std::str::FromStr;

use rsip as sip_rs;

use crate::utils::cli::CommandLines;

pub struct SipHandler {
    pub ip: String,
    pub port: u16,
    pub domain: String,
    pub id: String,
    pub password: String,
    pub algorithm: sip_rs::headers::auth::Algorithm,
    pub nonce: String,
    pub realm: String,
    pub call_id: String,
}

impl SipHandler {
    pub fn new(cli_args: &CommandLines) -> Self {
        SipHandler {
            ip: cli_args.sip_ip.clone(),
            port: cli_args.sip_port,
            domain: cli_args.sip_domain.clone(),
            id: cli_args.sip_id.clone(),
            password: cli_args.sip_password.clone(),
            algorithm: sip_rs::headers::auth::Algorithm::from_str(&cli_args.sip_algorithm).unwrap(),
            nonce: cli_args.sip_nonce.clone(),
            realm: cli_args.sip_realm.clone(),
            call_id: cli_args.call_id.clone(),
        }
    }
}
