use actix_web::{post, web, Responder};

use crate::{
    http::message::live::play::{LivePlayRequest, LivePlayResponse},
    store::base::StoreEngine,
};

#[post("/live/play")]
async fn post_play(
    data: web::Json<LivePlayRequest>,
    _sip_socket: web::Data<std::sync::Arc<tokio::net::UdpSocket>>,
    store_engine: web::Data<std::sync::Arc<Box<dyn StoreEngine>>>,
) -> impl Responder {
    let (not_found, is_playing, stream_id) = store_engine.invite(&data.gb_code, true);

    let (mut code, mut msg) = (200, "OK");
    if not_found {
        (code, msg) = (404, "ipc device not found")
    }

    if is_playing {
        // dispatch
    } else {
        // invite ipc device
    }

    let result = LivePlayResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: code,
        msg: msg.to_string(),
        gb_code: data.gb_code.clone(),
        stream_id: stream_id,
    };
    web::Json(result)
}
