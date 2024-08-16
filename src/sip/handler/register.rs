use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::{
    sip::handler::base::SipHandler,
    store::base::StoreEngine,
};

impl SipHandler {
    pub async fn on_register(
        &mut self,
        store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: std::net::SocketAddr,
        request: sip_rs::Request,
    ) {
        if let Some(auth) = request.authorization_header() {
            if let Ok(auth) = auth.typed() {
                if self.is_authorized(&auth.username, request.method(), &auth.uri, &auth.response) {
                    return self
                        .on_register_200(
                            &store_engine,
                            &sip_socket,
                            client_addr,
                            &request,
                            &auth.username,
                        )
                        .await;
                }
            }
        }

        return self
            .on_register_401(&store_engine, &sip_socket, client_addr, &request)
            .await;
    }

    async fn on_register_401(
        &self,
        _store_engine: &std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: std::net::SocketAddr,
        request: &sip_rs::Request,
    ) {
        let mut headers: sip_rs::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(Self::from_old(&request, &self.id, &self.domain).into());
        headers.push(self.to_old(&request).into());
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

        Self::socket_send_response_lite(&sip_socket, client_addr, response).await;
    }

    async fn on_register_200(
        &self,
        store_engine: &std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: &std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: std::net::SocketAddr,
        request: &sip_rs::Request,
        gb_code: &String,
    ) {
        let mut is_register = false;
        if let Some(exp) = request.expires_header() {
            if let Ok(seconds) = exp.seconds() {
                if 0 == seconds {
                    store_engine.unregister(&gb_code);
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
                    store_engine.register(&branch, &gb_code, client_addr);
                }
            }
        }

        let mut headers: sip_rs::Headers = Default::default();
        headers.push(request.via_header().unwrap().clone().into());
        headers.push(Self::from_old(&request, &self.id, &self.domain).into());
        headers.push(self.to_old(&request).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(sip_rs::Header::ContentLength(Default::default()));

        let response = sip_rs::Response {
            status_code: sip_rs::StatusCode::OK,
            headers,
            version: sip_rs::Version::V2,
            body: Default::default(),
        };

        Self::socket_send_response_lite(&sip_socket, client_addr, response).await;

        if is_register {
            Self::query_device_status(
                &store_engine,
                sip_socket,
                client_addr,
                &self.ip,
                self.port,
                &self.id,
                &self.domain,
                &Self::branch_get(&request),
                gb_code,
                &self.call_id,
            )
            .await;
        }
    }
}
