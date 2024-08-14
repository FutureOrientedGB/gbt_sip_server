use tokio;

use crate::sip::handler::SipRequestHandler;
use crate::store::base::StoreEngine;
use crate::utils::cli::CommandLines;

const MAX_UDP_SIZE: usize = 65535;

pub async fn bind(cli_args: &CommandLines) -> Result<tokio::net::UdpSocket, std::io::Error> {
    let local_addr = format!(
        "{host}:{port}",
        host = cli_args.host,
        port = cli_args.sip_port
    );

    match tokio::net::UdpSocket::bind(&local_addr).await {
        Err(e) => {
            tracing::error!("UdpSocket::bind({}) error, e: {:?}", &local_addr, e);
            return Err(e);
        }
        Ok(socket) => {
            tracing::info!("UdpSocket::bind({}) ok", &local_addr);
            return Ok(socket);
        }
    }
}

pub async fn run_forever(
    cli_args: &CommandLines,
    sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
    store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
) -> Result<(), std::io::Error> {
    let mut req_recv_buff = [0; MAX_UDP_SIZE];
    let mut sip_request_handler = SipRequestHandler::new(&cli_args);

    loop {
        match sip_socket.clone().recv_from(&mut req_recv_buff).await {
            Err(e) => {
                tracing::error!("UdpSocket::recv_from error, e: {:?}", e);
            }
            Ok((amount, client_addr)) => {
                sip_request_handler
                    .dispatch(
                        store_engine.clone(),
                        sip_socket.clone(),
                        client_addr,
                        &req_recv_buff[..amount],
                    )
                    .await;
            }
        }
    }
}
