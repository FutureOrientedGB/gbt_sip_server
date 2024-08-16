use rsip as sip_rs;
use rsip::prelude::UntypedHeader;
use tokio;

use crate::sip::handler::SipHandler;
use crate::store::base::StoreEngine;
use crate::{sip, version};

impl SipHandler {
    pub async fn invite_device(
        store_engine: &std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        device_addr: std::net::SocketAddr,
        ip: &String,
        port: u16,
        id: &String,
        domain: &String,
        branch: &String,
        call_id: &String,
        media_server_ip: &String,
        media_server_port: u16,
        session_type: sip::message::SdpSessionType,
        gb_code: &String,
        start_ts: u64,
        stop_ts: u64,
    ) -> bool {
        // body
        let str_body = sip::message::generate_media_sdp(
            media_server_ip,
            media_server_port,
            gb_code,
            session_type,
            start_ts,
            stop_ts,
        );
        let bin_body = str_body.as_bytes().to_vec();

        // headers
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(Self::via(ip, port, branch).into());
        headers.push(sip_rs::headers::MaxForwards::default().into());
        headers.push(Self::from_new(id, domain).into());
        headers.push(Self::to_new(gb_code, domain).into());
        headers.push(sip_rs::headers::Contact::new(format!(
            "<sip:{}@{}:{}>",
            id, ip, port
        )).into());
        headers.push(sip_rs::headers::CallId::from(format!("{}@{}", call_id, ip)).into());
        headers.push(
            sip_rs::typed::CSeq {
                seq: store_engine.add_fetch_global_sequence(),
                method: sip_rs::Method::Invite,
            }
            .into(),
        );
        headers.push(sip_rs::headers::typed::Allow::from(vec![
            sip_rs::common::Method::Invite,
            sip_rs::common::Method::Ack,
            sip_rs::common::Method::Bye,
            sip_rs::common::Method::Cancel,
            sip_rs::common::Method::Update,
            sip_rs::common::Method::PRack,
        ]).into());
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
            method: sip_rs::Method::Message,
            uri: sip_rs::Uri {
                scheme: Some(sip_rs::Scheme::Sip),
                auth: Some((gb_code.clone(), Option::<String>::None).into()),
                host_with_port: sip_rs::Domain::from(domain.clone()).into(),
                ..Default::default()
            },
            version: sip_rs::Version::V2,
            headers: headers,
            body: Default::default(),
        };

        return Self::socket_send_request_heavy(
            sip_socket,
            device_addr,
            request,
            bin_body,
            str_body,
        )
        .await;
    }
}
