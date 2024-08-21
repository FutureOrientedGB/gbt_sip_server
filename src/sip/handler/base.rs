use std::str::FromStr;

use rsip as sip_rs;

use crate::store::base::StoreEngine;
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
    pub store: Box<dyn StoreEngine>,
    pub sip_udp_socket: tokio::net::UdpSocket,
    pub sip_tcp_listener: tokio::net::TcpListener,
}

impl SipHandler {
    pub fn new(
        cli_args: &CommandLines,
        store: Box<dyn StoreEngine>,
        sip_udp_socket: tokio::net::UdpSocket,
        sip_tcp_listener: tokio::net::TcpListener,
    ) -> Self {
        SipHandler {
            ip: cli_args.my_ip.clone(),
            port: cli_args.sip_port,
            domain: cli_args.sip_domain.clone(),
            id: cli_args.sip_id.clone(),
            password: cli_args.sip_password.clone(),
            algorithm: sip_rs::headers::auth::Algorithm::from_str(&cli_args.sip_algorithm).unwrap(),
            nonce: cli_args.sip_nonce.clone(),
            realm: cli_args.sip_realm.clone(),
            store: store,
            sip_udp_socket: sip_udp_socket,
            sip_tcp_listener: sip_tcp_listener,
        }
    }
}
