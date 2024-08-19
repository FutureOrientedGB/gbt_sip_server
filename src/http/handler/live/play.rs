use actix_web::{post, web, Responder};

use crate::{
    http::message::live::play::{LivePlayRequest, LivePlayResponse},
    sip::{self, handler::SipHandler},
};

#[post("/live/play")]
async fn post_play(
    data: web::Json<LivePlayRequest>,
    sip_handler: web::Data<std::sync::Arc<SipHandler>>,
) -> impl Responder {
    let (not_found, is_playing, stream_id, device_addr, tcp_stream, branch) =
        sip_handler.store.invite(&data.gb_code, true);

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
            sip::message::sdp::SdpSessionType::Play,
            &data.gb_code,
            0,
            0,
        )
        .await;

    let result = LivePlayResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: code,
        msg: msg.to_string(),
        gb_code: data.gb_code.clone(),
        stream_id: stream_id,
    };
    web::Json(result)
}
