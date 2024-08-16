use std::str::FromStr;

use rsip as sip_rs;

use crate::utils::cli::CommandLines;
use crate::store::base::StoreEngine;

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
    pub store: Box<dyn StoreEngine>,
    pub sip_socket: tokio::net::UdpSocket,
}

impl SipHandler {
    pub fn new(cli_args: &CommandLines, store: Box<dyn StoreEngine>, sip_socket: tokio::net::UdpSocket) -> Self {
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
            store: store,
            sip_socket: sip_socket,
        }
    }
}
