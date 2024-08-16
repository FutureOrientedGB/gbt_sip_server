use actix_web::{post, web, Responder};

use crate::{
    http::message::live::stop::{LiveStopRequest, LiveStopResponse},
    sip::handler::SipHandler,
};

#[post("/live/stop")]
async fn post_stop(
    data: web::Json<LiveStopRequest>,
    sip_handler: web::Data<std::sync::Arc<SipHandler>>,
) -> impl Responder {
    sip_handler.store.bye(&data.gb_code, data.stream_id);

    let result = LiveStopResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };
    web::Json(result)
}
