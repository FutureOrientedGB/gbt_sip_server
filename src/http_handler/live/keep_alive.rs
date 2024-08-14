use actix_web::{post, web, Responder};

use crate::{
    http_message::live::keep_alive::{LiveKeepAliveRequest, LiveKeepAliveResponse},
    store::base::StoreEngine,
};

#[post("/live/keep_alive")]
async fn post_keep_alive(
    data: web::Json<LiveKeepAliveRequest>,
    _sip_socket: web::Data<std::sync::Arc<tokio::net::UdpSocket>>,
    store_engine: web::Data<std::sync::Arc<Box<dyn StoreEngine>>>,
) -> impl Responder {
    store_engine.stream_keep_alive(&data.gb_code, data.stream_id);

    let result = LiveKeepAliveResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };
    web::Json(result)
}
