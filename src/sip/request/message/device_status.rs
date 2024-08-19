use rsip as sip_rs;

use crate::sip::handler::SipHandler;
use crate::{sip, version};

impl SipHandler {
    pub async fn send_device_status_query(
        &self,
        device_addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::TcpStream>>>,
        transport: rsip::Transport,
        branch: &String,
        gb_code: &String,
    ) -> bool {
        // body
        let text_body =
            sip::message::DeviceStatusQuery::new(self.store.add_fetch_global_sn(), gb_code)
                .serialize_to_xml();
        let bin_body = self.encode_body(&text_body);

        // headers
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(self.via(transport, branch).into());
        headers.push(sip_rs::headers::MaxForwards::default().into());
        headers.push(self.from_new().into());
        headers.push(self.to_new(gb_code).into());
        headers.push(sip_rs::headers::CallId::from(format!("{}@{}", self.call_id, self.ip)).into());
        headers.push(
            sip_rs::typed::CSeq {
                seq: self.store.add_fetch_global_sequence(),
                method: sip_rs::Method::Message,
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
        headers.push(sip_rs::headers::ContentLength::from(bin_body.len() as u32).into());

        // request
        let request = sip_rs::Request {
            method: sip_rs::Method::Message,
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
            .socket_send_request_with_body(device_addr, tcp_stream, request, bin_body, text_body)
            .await;
    }
}
