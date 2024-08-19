use actix_web::{post, web, Responder};

use crate::{
    http::message::replay::start::{ReplayStartRequest, ReplayStartResponse},
    sip::{self, handler::SipHandler},
};

#[post("/replay/start")]
async fn post_start(
    data: web::Json<ReplayStartRequest>,
    sip_handler: web::Data<std::sync::Arc<SipHandler>>,
) -> impl Responder {
    let (not_found, is_playing, stream_id, device_addr, tcp_stream, branch) =
        sip_handler.store.invite(&data.gb_code, false);

    let (mut code, mut msg) = (200, "OK");
    if not_found {
        (code, msg) = (404, "ipc device not found")
    }

    if is_playing {
        // dispatch
    }
    sip_handler
        .send_invite(
            device_addr,
            tcp_stream,
            &branch,
            &String::from("127.0.0.1"),
            12345,
            sip::message::sdp::SdpSessionType::Playback,
            &data.gb_code,
            data.start_ts,
            data.stop_ts,
        )
        .await;

    let result = ReplayStartResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: code,
        msg: msg.to_string(),
        gb_code: data.gb_code.clone(),
        stream_id: stream_id,
    };
    web::Json(result)
}
