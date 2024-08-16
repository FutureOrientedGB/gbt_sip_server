use rsip as sip_rs;
use tokio;

use crate::sip::handler::SipHandler;
use crate::store::base::StoreEngine;
use crate::{sip, version};

impl SipHandler {
    pub async fn query_device_status(
        store_engine: &std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        device_addr: std::net::SocketAddr,
        ip: &String,
        port: u16,
        id: &String,
        domain: &String,
        branch: &String,
        gb_code: &String,
        call_id: &String,
    ) -> bool {
        // body
        let text_body =
            sip::message::DeviceStatusQuery::new(store_engine.add_fetch_global_sn(), gb_code)
                .serialize_to_xml();
        let bin_body = Self::encode_body(&text_body);

        // headers
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(Self::via(ip, port, branch).into());
        headers.push(sip_rs::headers::MaxForwards::default().into());
        headers.push(Self::from_new(id, domain).into());
        headers.push(Self::to_new(gb_code, domain).into());
        headers.push(sip_rs::headers::CallId::from(format!("{}@{}", call_id, ip)).into());
        headers.push(
            sip_rs::typed::CSeq {
                seq: store_engine.add_fetch_global_sequence(),
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
            text_body,
        )
        .await;
    }
}
