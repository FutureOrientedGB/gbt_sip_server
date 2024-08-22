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
    let (mut code, mut msg) = (200, "OK");

    let mut stream_id = 0;
    let call_id = sip_handler.caller_id_str();
    match sip_handler.store.invite(&data.gb_code, &call_id, true) {
        None => (code, msg) = (404, "ipc device not found"),
        Some((is_playing, id, branch, device_addr, tcp_stream)) => {
            stream_id = id;
            
            if is_playing {
                // dispatch
            }
            sip_handler
                .send_invite(
                    device_addr,
                    tcp_stream,
                    &branch,
                    &call_id,
                    &String::from("127.0.0.1"),
                    7080,
                    sip::message::sdp::SdpSessionType::Play,
                    &data.gb_code,
                    &data.setup_type,
                    0,
                    0,
                )
                .await;
        }
    };

    let result = LivePlayResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: code,
        msg: msg.to_string(),
        gb_code: data.gb_code.clone(),
        stream_id: stream_id,
    };
    web::Json(result)
}
