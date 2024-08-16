use rsip as sip_rs;
use tracing;

use crate::sip::handler::SipHandler;
use crate::utils::ansi_color as Color;

impl SipHandler {
    pub async fn socket_send_request_lite(
        &self,
        addr: std::net::SocketAddr,
        request: sip_rs::Request,
    ) -> bool {
        return self
            .socket_send(
                addr,
                request.to_string().as_bytes(),
                request.to_string(),
                "request",
            )
            .await;
    }

    pub async fn socket_send_request_heavy(
        &self,
        addr: std::net::SocketAddr,
        request: sip_rs::Request,
        bin_body: Vec<u8>,
        str_body: String,
    ) -> bool {
        let mut request_data: Vec<u8> = vec![];
        request_data.extend(request.to_string().as_bytes());
        request_data.extend(bin_body);
        return self
            .socket_send(
                addr,
                request_data.as_slice(),
                format!("{}{}", request.to_string(), str_body),
                "request",
            )
            .await;
    }

    pub async fn socket_send_response_lite(
        &self,
        addr: std::net::SocketAddr,
        response: sip_rs::Response,
    ) -> bool {
        return self
            .socket_send(
                addr,
                response.to_string().as_bytes(),
                response.to_string(),
                "response",
            )
            .await;
    }

    pub async fn socket_send_response_heavy(
        &self,
        addr: std::net::SocketAddr,
        response: sip_rs::Response,
        bin_body: Vec<u8>,
        str_body: String,
    ) -> bool {
        let mut response_data: Vec<u8> = vec![];
        response_data.extend(response.to_string().as_bytes());
        response_data.extend(bin_body);
        return self
            .socket_send(
                addr,
                response_data.as_slice(),
                format!("{}{}", response.to_string(), str_body),
                "response",
            )
            .await;
    }

    pub async fn socket_send(
        &self,
        addr: std::net::SocketAddr,
        data: &[u8],
        text: String,
        data_type: &str,
    ) -> bool {
        match self.sip_socket.send_to(data, addr).await {
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
