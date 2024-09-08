use rsip::{self as sip_rs, prelude::UntypedHeader};

use crate::sip::handler::SipHandler;
use crate::{sip, version};

impl SipHandler {
    pub async fn send_invite(
        &self,
        device_addr: std::net::SocketAddr,
        tcp_stream: Option<std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>>,
        branch: &String,
        caller_id: &String,
        media_server_ip: &String,
        media_server_port: u16,
        session_type: sip::message::SdpSessionType,
        gb_code: &String,
        setup_type: &String,
        start_ts: u64,
        stop_ts: u64,
    ) -> bool {
        // body
        let str_body = sip::message::generate_media_sdp(
            media_server_ip,
            media_server_port,
            gb_code,
            setup_type,
            session_type,
            start_ts,
            stop_ts,
        );
        let bin_body = str_body.as_bytes().to_vec();

        // headers
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(self.via(if tcp_stream.is_some() {rsip::Transport::Tcp} else {rsip::Transport::Udp}, branch).into());
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
                method: sip_rs::Method::Invite,
            }
            .into(),
        );
        headers.push(
            sip_rs::headers::typed::Allow::from(vec![
                sip_rs::common::Method::Invite,
                sip_rs::common::Method::Ack,
                sip_rs::common::Method::Bye,
                sip_rs::common::Method::Cancel,
                sip_rs::common::Method::Update,
                sip_rs::common::Method::PRack,
            ])
            .into(),
        );
        headers.push(sip_rs::headers::Supported::from(String::from("100rel")).into());
        headers.push(sip_rs::headers::Subject::from(format!("{gb_code}:0")).into());
        headers.push(
            sip_rs::headers::UserAgent::from(format!(
                "{} {}",
                version::APP_NAME,
                version::APP_VERSION
            ))
            .into(),
        );
        headers.push(sip_rs::headers::ContentType::from("application/sdp").into());
        headers.push(sip_rs::headers::ContentLength::from(bin_body.len() as u32).into());

        // request
        let request = sip_rs::Request {
            method: sip_rs::Method::Invite,
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
            .socket_send_request_with_body(device_addr, tcp_stream, request, bin_body, str_body)
            .await;
    }
}
