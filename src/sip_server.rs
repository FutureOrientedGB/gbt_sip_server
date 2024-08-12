use encoding_rs;

use tokio;

use crate::sip_handler::SipRequestHander;

const MAX_UDP_SIZE: usize = 65535;

pub async fn run_forever(host: &String, port: i32) -> Result<(), std::io::Error> {
    let local_addr = format!("{host}:{port}");

    match tokio::net::UdpSocket::bind(&local_addr).await {
        Err(e) => {
            tracing::error!("UdpSocket::bind({}) error, e: {:?}", &local_addr, e);
            return Err(e);
        }
        Ok(socket) => {
            tracing::info!(
                "UdpSocket::bind({}) ok",
                &local_addr
            );

            let mut buf = [0; MAX_UDP_SIZE];
            let mut gbt_hander = SipRequestHander::default();

            loop {
                match socket.recv_from(&mut buf).await {
                    Err(e) => {
                        tracing::error!("UdpSocket::recv_from error, e: {:?}", e);
                    }
                    Ok((amount, client_addr)) => {
                        let (msg, _encoding, has_error) =
                            encoding_rs::GB18030.decode(&buf[..amount]);
                        if has_error {
                            tracing::error!("encoding_rs::GB18030.decode error");
                            continue;
                        }

                        tracing::info!(
                            "UdpSocket::recv_from({}) ok, amount: {}, msg: {}",
                            client_addr,
                            amount,
                            msg
                        );

                        gbt_hander.dispatch(&socket, client_addr, &buf[..amount]).await;
                    }
                }
            }
        }
    }
}
