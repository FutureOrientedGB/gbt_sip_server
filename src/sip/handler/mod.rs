pub mod ack;
pub mod bye;
pub mod cancel;
pub mod info;
pub mod invite;
pub mod message;
pub mod notify;
pub mod options;
pub mod prack;
pub mod publish;
pub mod refer;
pub mod register;
pub mod subscribe;
pub mod update;

use std::net::SocketAddr;
use std::str::FromStr;

use rsip::{
    self as sip_rs,
    prelude::{HasHeaders, HeadersExt, ToTypedHeader},
};

use super::server::{DOUBLE_CR_LF_BYTES, SIP_BYTES};

use crate::store::base::StoreEngine;

use crate::utils::{ansi_color as Color, cli::CommandLines};

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

impl SipHandler {
    pub async fn dispatch(
        &self,
        device_addr: SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        sip_data: &[u8],
    ) {
        if sip_data.len() == DOUBLE_CR_LF_BYTES.len() && sip_data == DOUBLE_CR_LF_BYTES {
            return;
        }

        if sip_data.len() > SIP_BYTES.len() && sip_data[..SIP_BYTES.len()] == SIP_BYTES {
            self.dispatch_response(device_addr, tcp_stream, sip_data)
                .await;
        } else {
            self.dispatch_request(device_addr, tcp_stream, sip_data)
                .await;
        }
    }

    pub async fn dispatch_request(
        &self,
        device_addr: SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        sip_data: &[u8],
    ) {
        match sip_rs::Request::try_from(sip_data) {
            Err(e) => {
                tracing::error!(
                    "{}sip_rs::Request::try_from error, e: {}, {}request: {}",
                    Color::RED,
                    e,
                    Color::RESET,
                    String::from_utf8_lossy(sip_data)
                );
            }
            Ok(request) => {
                tracing::info!(
                    "{}⮜⮜⮜⮜⮜ {}sip_rs::Request::try_from({}) ok, amount: {:?}, request:{}\n{}",
                    Color::PURPLE,
                    Color::CYAN,
                    device_addr,
                    sip_data.len(),
                    Color::RESET,
                    format!(
                        "{}{} {} {}{}\n{}{}",
                        Color::YELLOW,
                        request.method().to_string(),
                        request.version().to_string(),
                        request.uri().to_string(),
                        Color::RESET,
                        request.headers().to_string(),
                        self.decode_body(request.body())
                    )
                );

                let seq = request.cseq_header().unwrap().typed().unwrap().seq;
                if seq > 0 {
                    if request.method() == &sip_rs::Method::Register {
                        self.store.set_register_sequence(seq);
                    } else {
                        self.store.set_global_sequence(seq);
                    }
                }

                match request.method() {
                    sip_rs::Method::Register => {
                        self.on_req_register(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Ack => self.on_req_ack(device_addr, tcp_stream, request).await,
                    sip_rs::Method::Bye => self.on_req_bye(device_addr, tcp_stream, request).await,
                    sip_rs::Method::Cancel => {
                        self.on_req_cancel(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Info => {
                        self.on_req_info(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Invite => {
                        self.on_req_invite(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Message => {
                        self.on_req_message(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Notify => {
                        self.on_req_notify(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Options => {
                        self.on_req_options(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::PRack => {
                        self.on_req_prack(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Publish => {
                        self.on_req_publish(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Refer => {
                        self.on_req_refer(device_addr, tcp_stream, request).await
                    }
                    sip_rs::Method::Subscribe => {
                        self.on_req_subscribe(device_addr, tcp_stream, request)
                            .await
                    }
                    sip_rs::Method::Update => {
                        self.on_req_update(device_addr, tcp_stream, request).await
                    }
                };
            }
        };
    }

    pub async fn dispatch_response(
        &self,
        device_addr: SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        sip_data: &[u8],
    ) {
        match sip_rs::Response::try_from(sip_data) {
            Err(e) => {
                tracing::error!(
                    "{}sip_rs::Response::try_from error, e: {}, {}response: {}",
                    Color::RED,
                    e,
                    Color::RESET,
                    String::from_utf8_lossy(sip_data)
                );
            }
            Ok(response) => {
                tracing::info!(
                    "{}⮜⮜⮜⮜⮜ {}sip_rs::Response::try_from({}) ok, amount: {:?}, response:{}\n{}",
                    Color::PURPLE,
                    Color::CYAN,
                    device_addr,
                    sip_data.len(),
                    Color::RESET,
                    format!(
                        "{} {}\n{}\n{}",
                        response.version().to_string(),
                        response.status_code().to_string(),
                        response.headers().to_string(),
                        self.decode_body(response.body())
                    )
                );

                if let Ok(seq) = response.cseq_header() {
                    if let Ok(method) = seq.method() {
                        match method {
                            sip_rs::Method::Register => {
                                self.on_rsp_register(device_addr, response).await
                            }
                            sip_rs::Method::Ack => {
                                self.on_rsp_ack(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Bye => {
                                self.on_rsp_bye(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Cancel => {
                                self.on_rsp_cancel(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Info => {
                                self.on_rsp_info(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Invite => {
                                self.on_rsp_invite(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Message => {
                                self.on_rsp_message(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Notify => {
                                self.on_rsp_notify(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Options => {
                                self.on_rsp_options(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::PRack => {
                                self.on_rsp_prack(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Publish => {
                                self.on_rsp_publish(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Refer => {
                                self.on_rsp_refer(device_addr, tcp_stream, response).await
                            }
                            sip_rs::Method::Subscribe => {
                                self.on_rsp_subscribe(device_addr, tcp_stream, response)
                                    .await
                            }
                            sip_rs::Method::Update => {
                                self.on_rsp_update(device_addr, tcp_stream, response).await
                            }
                        }
                    }
                }
            }
        }
    }
}
