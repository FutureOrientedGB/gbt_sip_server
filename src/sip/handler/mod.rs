pub mod base;
pub use base::SipHandler;
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

use rsip::{
    self as sip_rs,
    prelude::{HasHeaders, HeadersExt, ToTypedHeader},
};

use crate::utils::ansi_color as Color;

const INVALID_PREFIX: [u8; 4] = [b'\r', b'\n', b'\r', b'\n'];
const RESPONSE_PREFIX: [u8; 3] = [b'S', b'I', b'P'];

impl SipHandler {
    pub async fn dispatch(&self, device_addr: SocketAddr, sip_data: &[u8]) {
        if sip_data.len() == INVALID_PREFIX.len() && sip_data == INVALID_PREFIX {
            return;
        }
        
        if sip_data.len() > RESPONSE_PREFIX.len()
            && sip_data[..RESPONSE_PREFIX.len()] == RESPONSE_PREFIX
        {
            self.dispatch_response(device_addr, sip_data).await;
        } else {
            self.dispatch_request(device_addr, sip_data).await;
        }
    }

    pub async fn dispatch_request(&self, device_addr: SocketAddr, sip_data: &[u8]) {
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
                    "{}⮜⮜⮜⮜⮜ {}UdpSocket::recv_from({}) ok, amount: {:?}, request:{}\n{}",
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
                        Self::decode_body(request.body())
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
                    sip_rs::Method::Register => self.on_req_register(device_addr, request).await,
                    sip_rs::Method::Ack => self.on_req_ack(device_addr, request).await,
                    sip_rs::Method::Bye => self.on_req_bye(device_addr, request).await,
                    sip_rs::Method::Cancel => self.on_req_cancel(device_addr, request).await,
                    sip_rs::Method::Info => self.on_req_info(device_addr, request).await,
                    sip_rs::Method::Invite => self.on_req_invite(device_addr, request).await,
                    sip_rs::Method::Message => self.on_req_message(device_addr, request).await,
                    sip_rs::Method::Notify => self.on_req_notify(device_addr, request).await,
                    sip_rs::Method::Options => self.on_req_options(device_addr, request).await,
                    sip_rs::Method::PRack => self.on_req_prack(device_addr, request).await,
                    sip_rs::Method::Publish => self.on_req_publish(device_addr, request).await,
                    sip_rs::Method::Refer => self.on_req_refer(device_addr, request).await,
                    sip_rs::Method::Subscribe => self.on_req_subscribe(device_addr, request).await,
                    sip_rs::Method::Update => self.on_req_update(device_addr, request).await,
                };
            }
        };
    }

    pub async fn dispatch_response(&self, device_addr: SocketAddr, sip_data: &[u8]) {
        match sip_rs::Response::try_from(sip_data) {
            Err(e) => {
                tracing::error!(
                    "{}sip_rs::Request::try_from error, e: {}, {}response: {}",
                    Color::RED,
                    e,
                    Color::RESET,
                    String::from_utf8_lossy(sip_data)
                );
            }
            Ok(response) => {
                tracing::info!(
                    "{}⮜⮜⮜⮜⮜ {}UdpSocket::recv_from({}) ok, amount: {:?}, response:{}\n{}",
                    Color::PURPLE,
                    Color::CYAN,
                    device_addr,
                    sip_data.len(),
                    Color::RESET,
                    format!(
                        "{} {}\n{}{}",
                        response.version().to_string(),
                        response.status_code().to_string(),
                        response.headers().to_string(),
                        Self::decode_body(response.body())
                    )
                );

                if let Ok(seq) = response.cseq_header() {
                    if let Ok(method) = seq.method() {
                        match method {
                            sip_rs::Method::Register => self.on_rsp_register(device_addr, response).await,
                            sip_rs::Method::Ack => self.on_rsp_ack(device_addr, response).await,
                            sip_rs::Method::Bye => self.on_rsp_bye(device_addr, response).await,
                            sip_rs::Method::Cancel => self.on_rsp_cancel(device_addr, response).await,
                            sip_rs::Method::Info => self.on_rsp_info(device_addr, response).await,
                            sip_rs::Method::Invite => self.on_rsp_invite(device_addr, response).await,
                            sip_rs::Method::Message => self.on_rsp_message(device_addr, response).await,
                            sip_rs::Method::Notify => self.on_rsp_notify(device_addr, response).await,
                            sip_rs::Method::Options => self.on_rsp_options(device_addr, response).await,
                            sip_rs::Method::PRack => self.on_rsp_prack(device_addr, response).await,
                            sip_rs::Method::Publish => self.on_rsp_publish(device_addr, response).await,
                            sip_rs::Method::Refer => self.on_rsp_refer(device_addr, response).await,
                            sip_rs::Method::Subscribe => self.on_rsp_subscribe(device_addr, response).await,
                            sip_rs::Method::Update => self.on_rsp_update(device_addr, response).await,
                        }
                    }
                }
            }
        }
    }
}
