use std::net::SocketAddr;

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
pub mod utils;

use rsip::{self, prelude::HasHeaders};

use tokio;

use crate::store::base::StoreEngine;

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
                tracing::error!("rsip::Request::try_from error, e: {:?}", e);
            }
            Ok(request) => {
                tracing::info!(
                    "UdpSocket::recv_from({}) ok, amount: {:?}, data: \n{}",
                    client_addr,
                    request_data.len(),
                    request.headers().to_string() + &self.decode_body(&request)
                );

                let response = match request.method() {
                    rsip::Method::Register => self.on_register(store_engine, request).await,
                    rsip::Method::Ack => self.on_ack(store_engine, request).await,
                    rsip::Method::Bye => self.on_bye(store_engine, request).await,
                    rsip::Method::Cancel => self.on_cancel(store_engine, request).await,
                    rsip::Method::Info => self.on_info(store_engine, request).await,
                    rsip::Method::Invite => self.on_invite(store_engine, request).await,
                    rsip::Method::Message => self.on_message(store_engine, request).await,
                    rsip::Method::Notify => self.on_notify(store_engine, request).await,
                    rsip::Method::Options => self.on_options(store_engine, request).await,
                    rsip::Method::PRack => self.on_prack(store_engine, request).await,
                    rsip::Method::Publish => self.on_publish(store_engine, request).await,
                    rsip::Method::Refer => self.on_refer(store_engine, request).await,
                    rsip::Method::Subscribe => self.on_subscribe(store_engine, request).await,
                    rsip::Method::Update => self.on_update(store_engine, request).await,
                };

                let mut response_data: Vec<u8> = vec![];
                if response.body().is_empty() {
                    response_data.extend(response.to_string().as_bytes());
                } else {
                    // encode body
                    let encoded =
                        self.encode_body(String::from_utf8(response.body().to_vec()).unwrap());
                    response_data.extend(encoded);
                }

                match sip_socket.send_to(response_data.as_slice(), client_addr).await {
                    Err(e) => {
                        tracing::error!("UdpSocket::send_to({}) error, e: {:?}", client_addr, e);
                    }
                    Ok(amount) => {
                        tracing::info!(
                            "UdpSocket::send_to({}) ok, amount: {:?}, data: \n{}",
                            client_addr,
                            amount,
                            response.to_string()
                        );
                    }
                }
            }
        };
    }
}
