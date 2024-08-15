use rsip::{self, prelude::HeadersExt};

use crate::{
    sip::handler::base::SipRequestHandler, sip::message::DeviceStatus, store::base::StoreEngine,
};

impl SipRequestHandler {
    pub async fn on_device_status(
        &mut self,
        _store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _client_addr: std::net::SocketAddr,
        request: rsip::Request,
        msg: String,
    ) {
        let _data = DeviceStatus::deserialize_from_xml(msg);

        let mut headers: rsip::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(Self::from_old(&request, &self.id, &self.domain).into());
        headers.push(self.to_old(&request).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(rsip::Header::ContentLength(Default::default()));

        // (
        //     rsip::Response {
        //         status_code: rsip::StatusCode::OK,
        //         headers,
        //         version: rsip::Version::V2,
        //         body: Default::default(),
        //     },
        //     vec![],
        // )
    }
}
