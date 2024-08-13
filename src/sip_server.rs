use encoding_rs;

use tokio;

use crate::cli::CommandLines;
use crate::sip_handler::SipRequestHandler;

const MAX_UDP_SIZE: usize = 65535;

pub async fn run_forever(cli_args: &CommandLines) -> Result<(), std::io::Error> {
    let local_addr = format!("{host}:{port}", host = cli_args.host, port = cli_args.port);

    match tokio::net::UdpSocket::bind(&local_addr).await {
        Err(e) => {
            tracing::error!("UdpSocket::bind({}) error, e: {:?}", &local_addr, e);
            return Err(e);
        }
        Ok(socket) => {
            tracing::info!("UdpSocket::bind({}) ok", &local_addr);

            let mut req_recv_buff = [0; MAX_UDP_SIZE];
            let mut sip_request_handler = SipRequestHandler::new(&cli_args);

            loop {
                match socket.recv_from(&mut req_recv_buff).await {
                    Err(e) => {
                        tracing::error!("UdpSocket::recv_from error, e: {:?}", e);
                    }
                    Ok((amount, client_addr)) => {
                        let (msg, _encoding, has_error) =
                            encoding_rs::GB18030.decode(&req_recv_buff[..amount]);
                        if has_error {
                            tracing::error!("encoding_rs::GB18030.decode error");
                            continue;
                        }

                        tracing::info!(
                            "UdpSocket::recv_from({}) ok, amount: {}, msg:\n{}",
                            client_addr,
                            amount,
                            &msg
                        );

                        sip_request_handler
                            .dispatch(&socket, client_addr, msg.to_owned().to_string())
                            .await;
                    }
                }
            }
        }
    }
}
