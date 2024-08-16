use rsip;
use tokio;
use tracing;

use crate::sip::handler::SipRequestHandler;
use crate::utils::ansi_color as Color;

impl SipRequestHandler {
    pub async fn socket_send_request_lite(
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        addr: std::net::SocketAddr,
        request: rsip::Request,
    ) -> bool {
        return Self::socket_send(
            sip_socket,
            addr,
            request.to_string().as_bytes(),
            request.to_string(),
            "request",
        )
        .await;
    }

    pub async fn socket_send_request_heavy(
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        addr: std::net::SocketAddr,
        request: rsip::Request,
        bin_body: Vec<u8>,
        text_body: String,
    ) -> bool {
        let mut request_data: Vec<u8> = vec![];
        request_data.extend(request.to_string().as_bytes());
        request_data.extend(bin_body);
        return Self::socket_send(
            sip_socket,
            addr,
            request_data.as_slice(),
            format!("{}{}", request.to_string(), text_body),
            "request",
        )
        .await;
    }

    pub async fn socket_send_response_lite(
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        addr: std::net::SocketAddr,
        response: rsip::Response,
    ) -> bool {
        return Self::socket_send(
            sip_socket,
            addr,
            response.to_string().as_bytes(),
            response.to_string(),
            "response",
        )
        .await;
    }

    pub async fn socket_send_response_heavy(
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        addr: std::net::SocketAddr,
        response: rsip::Response,
        bin_body: Vec<u8>,
        text_body: String,
    ) -> bool {
        let mut response_data: Vec<u8> = vec![];
        response_data.extend(response.to_string().as_bytes());
        response_data.extend(bin_body);
        return Self::socket_send(
            sip_socket,
            addr,
            response_data.as_slice(),
            format!("{}{}", response.to_string(), text_body),
            "response",
        )
        .await;
    }

    pub async fn socket_send(
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        addr: std::net::SocketAddr,
        data: &[u8],
        text: String,
        data_type: &str,
    ) -> bool {
        match sip_socket.send_to(data, addr).await {
            Err(e) => {
                tracing::error!(
                    "{}UdpSocket::send_to({}) error, e: {}, {}data: {}",
                    Color::RED,
                    addr,
                    e,
                    Color::RESET,
                    text
                );
                return false;
            }
            Ok(amount) => {
                tracing::info!(
                    "{}⮞⮞⮞⮞⮞ {}UdpSocket::send_to({}) ok, amount: {:?}, {}:{}\n{}",
                    Color::GREEN,
                    Color::CYAN,
                    addr,
                    amount,
                    data_type,
                    Color::RESET,
                    text
                );
                return true;
            }
        }
    }
}
