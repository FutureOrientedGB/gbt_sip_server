use rsip;
use tokio;

use crate::store::base::StoreEngine;
use crate::{sip, version};

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
    let bin_body = sip::handler::SipRequestHandler::encode_body(&text_body);

    // headers
    let mut headers: rsip::Headers = Default::default();
    headers.push(sip::handler::SipRequestHandler::via(ip, port, branch).into());
    headers.push(rsip::headers::MaxForwards::default().into());
    headers.push(sip::handler::SipRequestHandler::from_new(id, domain).into());
    headers.push(sip::handler::SipRequestHandler::to_new(gb_code, domain).into());
    headers.push(
        rsip::typed::CSeq {
            seq: store_engine.add_fetch_global_sequence(),
            method: rsip::Method::Message,
        }
        .into(),
    );
    headers.push(
        rsip::headers::UserAgent::from(format!("{} {}", version::APP_NAME, version::APP_VERSION))
            .into(),
    );
    headers.push(rsip::headers::CallId::from(format!("{}@{}", call_id, ip)).into());
    headers.push(rsip::headers::ContentLength::from(bin_body.len() as u32).into());

    // request
    let request = rsip::Request {
        method: rsip::Method::Message,
        uri: rsip::Uri {
            scheme: Some(rsip::Scheme::Sip),
            auth: Some((gb_code.clone(), Option::<String>::None).into()),
            host_with_port: rsip::Domain::from(domain.clone()).into(),
            ..Default::default()
        },
        version: rsip::Version::V2,
        headers: headers,
        body: Default::default(),
    };

    return sip::handler::SipRequestHandler::socket_send_request_heavy(
        sip_socket,
        device_addr,
        request,
        bin_body,
        text_body,
    )
    .await;
}
