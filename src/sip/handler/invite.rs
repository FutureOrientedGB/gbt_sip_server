use std::str::FromStr;

use rsip::{self as sip_rs, prelude::HeadersExt};

use sdp_rs;

use super::SipHandler;

impl SipHandler {
    pub async fn on_req_invite(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        _request: sip_rs::Request,
    ) {
    }

    pub async fn on_rsp_invite(
        &self,
        device_addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        response: sip_rs::Response,
    ) {
        if &rsip::StatusCode::Trying == response.status_code() {
            self.on_rsp_invite_100(device_addr, tcp_stream, response)
                .await;
        } else if &rsip::StatusCode::OK == response.status_code() {
            self.on_rsp_invite_200(device_addr, tcp_stream, response)
                .await;
        } else {
            tracing::warn!("unexpected response, method: {}", response.status_code());
        }
    }

    pub async fn on_rsp_invite_100(
        &self,
        _device_addr: std::net::SocketAddr,
        _tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        _response: sip_rs::Response,
    ) {
    }

    pub async fn on_rsp_invite_200(
        &self,
        device_addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        response: sip_rs::Response,
    ) {
        // decode body
        let sdp_msg = self.decode_body(response.body());

        match sdp_rs::SessionDescription::from_str(&sdp_msg) {
            Err(e) => {
                tracing::error!("sdp_rs::SessionDescription::from_str error, e: {:?}", e);
            }
            Ok(media_desc) => {
                let gb_code = media_desc.origin.username;
                if gb_code.is_empty() {
                    tracing::error!("invalid device");
                } else if self.store.find_device_by_gb_code(&gb_code).is_none() {
                    tracing::error!("device not found");
                } else {
                    let mut headers: sip_rs::Headers = Default::default();
                    headers.push(response.via_header().unwrap().clone().into());
                    headers.push(response.from_header().unwrap().clone().into());
                    headers.push(self.to_old(&response.to_header().unwrap()).into());
                    headers.push(response.call_id_header().unwrap().clone().into());
                    headers.push(response.cseq_header().unwrap().clone().into());
                    headers.push(sip_rs::Header::ContentLength(Default::default()));

                    let request = sip_rs::Request {
                        method: sip_rs::Method::Ack,
                        uri: sip_rs::Uri {
                            scheme: Some(sip_rs::Scheme::Sip),
                            auth: Some((gb_code.clone(), Option::<String>::None).into()),
                            host_with_port: sip_rs::Domain::from(self.domain.clone()).into(),
                            ..Default::default()
                        },
                        headers,
                        version: sip_rs::Version::V2,
                        body: Default::default(),
                    };

                    self.socket_send_request(device_addr, tcp_stream, request).await;
                }
            }
        }
    }
}
