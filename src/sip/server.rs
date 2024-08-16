use tokio;

use crate::sip::handler::SipHandler;
use crate::utils::cli::CommandLines;

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
    sip_handler: &std::sync::Arc<SipHandler>,
) -> Result<(), std::io::Error> {
    let mut recv_buff = Vec::<u8>::default();
    recv_buff.resize(cli_args.socket_recv_buffer_size, 0);

    loop {
        match sip_handler
            .clone()
            .sip_socket
            .recv_from(recv_buff.as_mut_slice())
            .await
        {
            Err(e) => {
                tracing::error!("UdpSocket::recv_from error, e: {:?}", e);
            }
            Ok((amount, client_addr)) => {
                sip_handler
                    .dispatch(client_addr, &recv_buff.as_slice()[..amount])
                    .await;
            }
        }
    }
}
