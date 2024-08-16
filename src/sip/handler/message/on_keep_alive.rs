use rsip::{self as sip_rs, prelude::HeadersExt};

use crate::{
    sip::handler::base::SipHandler, sip::message::KeepAlive, store::base::StoreEngine,
};

impl SipHandler {
    pub async fn on_keep_alive(
        &mut self,
        store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _client_addr: std::net::SocketAddr,
        request: sip_rs::Request,
        msg: String,
    ) {
        let data = KeepAlive::deserialize_from_xml(msg);
        if data.sn > 0 {
            store_engine.set_global_sn(data.sn);
        }

        let gb_code = request
            .from_header()
            .unwrap()
            .uri()
            .unwrap()
            .auth
            .unwrap()
            .to_string();
        store_engine.register_keep_alive(&gb_code);

        let mut headers: sip_rs::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(Self::from_old(&request, &self.id, &self.domain).into());
        headers.push(self.to_old(&request).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(sip_rs::Header::ContentLength(Default::default()));
    }
}
