use rsip as sip_rs;

use tokio;

use tokio::io::AsyncWriteExt;
use tracing;

use crate::sip::handler::SipHandler;
use crate::utils::ansi_color as Color;

impl SipHandler {
    pub async fn socket_send_request(
        &self,
        addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        request: sip_rs::Request,
    ) -> bool {
        return self
            .socket_send(
                addr,
                tcp_stream,
                request.to_string().as_bytes(),
                request.to_string(),
                "request",
            )
            .await;
    }

    pub async fn socket_send_request_with_body(
        &self,
        addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        request: sip_rs::Request,
        bin_body: Vec<u8>,
        str_body: String,
    ) -> bool {
        let mut contents: Vec<u8> = vec![];
        contents.extend(request.to_string().as_bytes());
        contents.extend(bin_body);
        return self
            .socket_send(
                addr,
                tcp_stream,
                contents.as_slice(),
                format!("{}{}", request.to_string(), str_body),
                "request",
            )
            .await;
    }

    pub async fn socket_send_response(
        &self,
        addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        response: sip_rs::Response,
    ) -> bool {
        return self
            .socket_send(
                addr,
                tcp_stream,
                response.to_string().as_bytes(),
                response.to_string(),
                "response",
            )
            .await;
    }

    pub async fn socket_send_response_with_body(
        &self,
        addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        response: sip_rs::Response,
        bin_body: Vec<u8>,
        str_body: String,
    ) -> bool {
        let mut contents: Vec<u8> = vec![];
        contents.extend(response.to_string().as_bytes());
        contents.extend(bin_body);
        return self
            .socket_send(
                addr,
                tcp_stream,
                contents.as_slice(),
                format!("{}{}", response.to_string(), str_body),
                "response",
            )
            .await;
    }

    pub async fn socket_send(
        &self,
        addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        data: &[u8],
        text: String,
        data_type: &str,
    ) -> bool {
        if let Some(stream) = tcp_stream {
            return self
                .tcp_socket_send(addr, stream, data, text, data_type)
                .await;
        } else {
            return self.udp_socket_send(addr, data, text, data_type).await;
        }
    }

    pub async fn udp_socket_send(
        &self,
        addr: std::net::SocketAddr,
        data: &[u8],
        text: String,
        data_type: &str,
    ) -> bool {
        match self.sip_udp_socket.send_to(data, addr).await {
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

    pub async fn tcp_socket_send(
        &self,
        addr: std::net::SocketAddr,
        tcp_stream: std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>,
        data: &[u8],
        text: String,
        data_type: &str,
    ) -> bool {
        match tcp_stream.lock().await.write_all(data).await {
            Err(e) => {
                tracing::error!(
                    "{}TcpStream::send_to({}) error, e: {}, {}data: {}",
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
                    "{}⮞⮞⮞⮞⮞ {}TcpStream::send_to({}) ok, amount: {:?}, {}:{}\n{}",
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
