pub mod internal;
use std::net::SocketAddr;

pub use internal::SipRequestHandler;
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

use rsip::{self, prelude::HasHeaders};

use tokio;

impl SipRequestHandler {
    pub async fn dispatch(
        &mut self,
        socket: &tokio::net::UdpSocket,
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

                let response_data = match request.method() {
                    rsip::Method::Register => self.on_register(request).await,
                    rsip::Method::Ack => self.on_ack(request).await,
                    rsip::Method::Bye => self.on_bye(request).await,
                    rsip::Method::Cancel => self.on_cancel(request).await,
                    rsip::Method::Info => self.on_info(request).await,
                    rsip::Method::Invite => self.on_invite(request).await,
                    rsip::Method::Message => self.on_message(request).await,
                    rsip::Method::Notify => self.on_notify(request).await,
                    rsip::Method::Options => self.on_options(request).await,
                    rsip::Method::PRack => self.on_prack(request).await,
                    rsip::Method::Publish => self.on_publish(request).await,
                    rsip::Method::Refer => self.on_refer(request).await,
                    rsip::Method::Subscribe => self.on_subscribe(request).await,
                    rsip::Method::Update => self.on_update(request).await,
                };

                if response_data.is_empty() {
                    tracing::error!("skip empty response");
                    return;
                }

                match socket.send_to(response_data.as_bytes(), client_addr).await {
                    Err(e) => {
                        tracing::error!("UdpSocket::send_to({}) error, e: {:?}", client_addr, e);
                    }
                    Ok(amount) => {
                        tracing::info!(
                            "UdpSocket::send_to({}) ok, amount: {:?}, data: \n{}",
                            client_addr,
                            amount,
                            response_data
                        );
                    }
                }
            }
        };
    }
}
