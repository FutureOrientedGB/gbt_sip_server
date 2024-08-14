use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::{
    sip::handler::base::SipRequestHandler, sip::message::KeepAlive, store::base::StoreEngine,
};

impl SipRequestHandler {
    pub async fn on_keep_alive(
        &mut self,
        _store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _client_addr: std::net::SocketAddr,
        request: rsip::Request,
        msg: String,
    ) -> rsip::Response {
        let _data = KeepAlive::deserialize_from_xml(msg);

        // _store_engine.register_keep_alive

        let mut headers: rsip::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        let from_uri = format!("sip:{}@{}", &self.id, &self.domain);
        let from = request.from_header().unwrap().typed().unwrap();
        headers.push(
            from.with_uri(rsip::Uri::try_from(from_uri).unwrap())
                .with_tag(self.extract_tag(&request).into())
                .into(),
        );
        let to = request.to_header().unwrap().typed().unwrap();
        headers.push(to.with_tag(self.random_tag(8).into()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(rsip::Header::ContentLength(Default::default()));

        rsip::Response {
            status_code: rsip::StatusCode::OK,
            headers,
            version: rsip::Version::V2,
            body: Default::default(),
        }
    }
}
