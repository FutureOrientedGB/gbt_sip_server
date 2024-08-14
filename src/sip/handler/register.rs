use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::{sip::handler::base::SipRequestHandler, store::base::StoreEngine};

impl SipRequestHandler {
    pub async fn on_register(
        &mut self,
        store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: std::net::SocketAddr,
        request: rsip::Request,
    ) -> rsip::Response {
        if let Some(auth) = request.authorization_header() {
            if let Ok(auth) = auth.typed() {
                if self.is_authorized(&auth.username, request.method(), &auth.uri, &auth.response) {
                    return self
                        .on_register_200(store_engine, sip_socket, client_addr, request)
                        .await;
                }
            }
        }

        return self
            .on_register_401(store_engine, sip_socket, client_addr, request)
            .await;
    }

    async fn on_register_401(
        &self,
        _store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        _client_addr: std::net::SocketAddr,
        request: rsip::Request,
    ) -> rsip::Response {
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
        headers.push(to.with_tag(self.random_tag(10).into()).into());
        headers.push(request.call_id_header().unwrap().clone().into());
        headers.push(request.cseq_header().unwrap().clone().into());
        headers.push(rsip::Header::ContentLength(Default::default()));

        headers.push(
            rsip::typed::WwwAuthenticate {
                realm: self.realm.clone(),
                nonce: self.nonce.clone(),
                algorithm: Some(self.algorithm),
                opaque: Some("".into()),
                ..Default::default()
            }
            .into(),
        );

        rsip::Response {
            status_code: rsip::StatusCode::Unauthorized,
            headers,
            version: rsip::Version::V2,
            body: Default::default(),
        }
    }

    async fn on_register_200(
        &self,
        store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
        _sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
        client_addr: std::net::SocketAddr,
        request: rsip::Request,
    ) -> rsip::Response {
        let gb_code = request.from_header().unwrap().uri().unwrap().auth.unwrap().to_string();
        if let Some(exp) = request.expires_header() {
            if let Ok(seconds) = exp.seconds() {
                if 0 == seconds {
                    store_engine.unregister(&gb_code);
                } else {
                    store_engine.register(&gb_code, client_addr);
                }
            }
        }

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
