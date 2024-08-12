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

use rsip;

use tokio;

impl SipRequestHandler {
    pub async fn dispatch(&mut self, socket: &tokio::net::UdpSocket, client_addr: SocketAddr, request_data: String){
        match rsip::Request::try_from(request_data.as_bytes()) {
            Err(e) => {
                tracing::error!("rsip::Request::try_from error, e: {:?}", e);
            }
            Ok(request) => {
                let response_data = match request.method() {
                    rsip::Method::Register => {
                        self.on_register(request).await
                    }
                    rsip::Method::Ack => {
                        self.on_ack(request).await
                    }
                    rsip::Method::Bye => {
                        self.on_bye(request).await
                    }
                    rsip::Method::Cancel => {
                        self.on_cancel(request).await
                    }
                    rsip::Method::Info => {
                        self.on_info(request).await
                    }
                    rsip::Method::Invite => {
                        self.on_invite(request).await
                    }
                    rsip::Method::Message => {
                        self.on_message(request).await
                    }
                    rsip::Method::Notify => {
                        self.on_notify(request).await
                    }
                    rsip::Method::Options => {
                        self.on_options(request).await
                    }
                    rsip::Method::PRack => {
                        self.on_prack(request).await
                    }
                    rsip::Method::Publish => {
                        self.on_publish(request).await
                    }
                    rsip::Method::Refer => {
                        self.on_refer(request).await
                    }
                    rsip::Method::Subscribe => {
                        self.on_subscribe(request).await
                    }
                    rsip::Method::Update => {
                        self.on_update(request).await
                    }
                };

                if response_data.is_empty() {
                    return;
                }

                let (msg, _encoding, has_error) =
                    encoding_rs::GB18030.encode(&response_data);

                match socket.send_to(msg.to_vec().as_slice(), client_addr).await {
                    Err(e) => {
                        tracing::error!(
                            "UdpSocket::send_to({}) error, e: {:?}",
                            client_addr,
                            e
                        );
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
