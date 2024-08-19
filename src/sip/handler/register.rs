use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub async fn on_req_register(
        &self,
        device_addr: std::net::SocketAddr,
        request: sip_rs::Request,
    ) {
        if let Some(auth) = request.authorization_header() {
            if let Ok(auth) = auth.typed() {
                if self.is_authorized(&auth.username, request.method(), &auth.uri, &auth.response) {
                    return self
                        .on_req_register_200(device_addr, &request, &auth.username)
                        .await;
                }
            }
        }

        return self.on_req_register_401(device_addr, &request).await;
    }

    async fn on_req_register_401(
        &self,
        device_addr: std::net::SocketAddr,
        request: &sip_rs::Request,
    ) {
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(request.from_header().unwrap().clone().into());
        headers.push(self.to_old(request.to_header().unwrap()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(sip_rs::Header::ContentLength(Default::default()));

        headers.push(
            sip_rs::typed::WwwAuthenticate {
                realm: self.realm.clone(),
                nonce: self.nonce.clone(),
                algorithm: Some(self.algorithm),
                opaque: Some("".into()),
                ..Default::default()
            }
            .into(),
        );

        let response = sip_rs::Response {
            status_code: sip_rs::StatusCode::Unauthorized,
            headers,
            version: sip_rs::Version::V2,
            body: Default::default(),
        };

        self.socket_send_response(device_addr, response).await;
    }

    async fn on_req_register_200(
        &self,
        device_addr: std::net::SocketAddr,
        request: &sip_rs::Request,
        gb_code: &String,
    ) {
        let mut is_register = false;
        if let Some(exp) = request.expires_header() {
            if let Ok(seconds) = exp.seconds() {
                if 0 == seconds {
                    self.store.unregister(&gb_code);
                } else {
                    is_register = true;
                    let branch = request
                        .via_header()
                        .unwrap()
                        .typed()
                        .unwrap()
                        .branch()
                        .unwrap()
                        .to_string();
                    self.store.register(&branch, &gb_code, device_addr);
                }
            }
        }

        let mut headers: sip_rs::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(request.from_header().unwrap().clone().into());
        headers.push(self.to_old(request.to_header().unwrap()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(sip_rs::Header::ContentLength(Default::default()));

        let response = sip_rs::Response {
            status_code: sip_rs::StatusCode::OK,
            headers,
            version: sip_rs::Version::V2,
            body: Default::default(),
        };

        self.socket_send_response(device_addr, response).await;

        if is_register {
            self.send_device_status_query(
                device_addr,
                &Self::branch_get(request.via_header().unwrap()),
                gb_code,
            )
            .await;
        }
    }

    pub async fn on_rsp_register(
        &self,
        _device_addr: std::net::SocketAddr,
        _response: sip_rs::Response,
    ) {
    }
}
