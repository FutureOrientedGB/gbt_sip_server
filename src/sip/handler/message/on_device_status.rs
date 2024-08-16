use rsip::{self as sip_rs, prelude::HeadersExt};

use crate::{
    sip::handler::base::SipHandler, sip::message::DeviceStatus,
};

impl SipHandler {
    pub async fn on_device_status(
        &self,
        _client_addr: std::net::SocketAddr,
        request: sip_rs::Request,
        msg: String,
    ) {
        let data = DeviceStatus::deserialize_from_xml(msg);
        if data.sn > 0 {
            self.store.set_global_sn(data.sn);
        }

        let mut headers: sip_rs::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(self.from_old(&request).into());
        headers.push(self.to_old(&request).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(sip_rs::Header::ContentLength(Default::default()));
    }
}
