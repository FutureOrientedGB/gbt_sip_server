use rsip::{self as sip_rs, prelude::UntypedHeader};

use crate::sip::handler::SipHandler;
use crate::version;

impl SipHandler {
    pub async fn send_bye(
        &self,
        device_addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        branch: &String,
        caller_id: &String,
        gb_code: &String,
    ) -> bool {
        // headers
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(
            self.via(
                if tcp_stream.is_some() {
                    rsip::Transport::Tcp
                } else {
                    rsip::Transport::Udp
                },
                branch,
            )
            .into(),
        );
        headers.push(sip_rs::headers::MaxForwards::default().into());
        headers.push(self.from_new().into());
        headers.push(self.to_new(gb_code).into());
        headers.push(
            sip_rs::headers::Contact::new(format!("<sip:{}@{}:{}>", self.id, self.ip, self.port))
                .into(),
        );
        headers.push(self.caller_id_from_str(caller_id).into());
        headers.push(
            sip_rs::typed::CSeq {
                seq: self.store.add_fetch_global_sequence(),
                method: sip_rs::Method::Bye,
            }
            .into(),
        );
        headers.push(
            sip_rs::headers::UserAgent::from(format!(
                "{} {}",
                version::APP_NAME,
                version::APP_VERSION
            ))
            .into(),
        );

        // request
        let request = sip_rs::Request {
            method: sip_rs::Method::Bye,
            uri: sip_rs::Uri {
                scheme: Some(sip_rs::Scheme::Sip),
                auth: Some((gb_code.clone(), Option::<String>::None).into()),
                host_with_port: sip_rs::Domain::from(self.domain.clone()).into(),
                ..Default::default()
            },
            version: sip_rs::Version::V2,
            headers: headers,
            body: Default::default(),
        };

        return self
            .socket_send_request(device_addr, tcp_stream, request)
            .await;
    }
}
