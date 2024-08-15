pub mod base;
pub use base::SipRequestHandler;
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

use rsip::{self, prelude::HasHeaders};

use tokio;

use crate::store::base::StoreEngine;
use crate::utils::ansi_color as Color;

impl SipRequestHandler {
    pub async fn dispatch(
        &mut self,
        store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: SocketAddr,
        request_data: &[u8],
    ) {
        match rsip::Request::try_from(request_data) {
            Err(e) => {
                tracing::error!(
                    "{}rsip::Request::try_from error, e: {}{}",
                    Color::RED,
                    e,
                    Color::RESET
                );
            }
            Ok(request) => {
                tracing::info!(
                    "{}<<<<< {}UdpSocket::recv_from({}) ok, amount: {:?}, request:{}\n{}",
                    Color::PURPLE,
                    Color::CYAN,
                    client_addr,
                    request_data.len(),
                    Color::RESET,
                    format!(
                        "{}{} {} {}{}\n{}{}",
                        Color::YELLOW,
                        request.method().to_string(),
                        request.version().to_string(),
                        request.uri().to_string(),
                        Color::RESET,
                        request.headers().to_string(),
                        Self::decode_body(&request)
                    )
                );

                match request.method() {
                    rsip::Method::Register => {
                        self.on_register(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Ack => {
                        self.on_ack(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Bye => {
                        self.on_bye(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Cancel => {
                        self.on_cancel(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Info => {
                        self.on_info(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Invite => {
                        self.on_invite(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Message => {
                        self.on_message(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Notify => {
                        self.on_notify(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Options => {
                        self.on_options(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::PRack => {
                        self.on_prack(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Publish => {
                        self.on_publish(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Refer => {
                        self.on_refer(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Subscribe => {
                        self.on_subscribe(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                    rsip::Method::Update => {
                        self.on_update(store_engine, sip_socket.clone(), client_addr, request)
                            .await
                    }
                };

                // let mut response_data: Vec<u8> = vec![];
                // response_data.extend(response.to_string().as_bytes());
                // if !body.is_empty() {
                //     // encode body
                //     let encoded = Self::encode_body(String::from_utf8(body).unwrap());
                //     response_data.extend(encoded);
                // }

                // sip::utils::sock::socket_send(
                //     sip_socket,
                //     client_addr,
                //     &response_data,
                //     response.to_string(),
                //     "response",
                // )
                // .await;
            }
        };
    }
}
