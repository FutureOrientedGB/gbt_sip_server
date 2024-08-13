use actix_web::{post, web, Responder};

use crate::http_message::live::stop::{LiveStopRequest, LiveStopResponse};

#[post("/live/stop")]
async fn post_stop(data: web::Json<LiveStopRequest>) -> impl Responder {
    let result = LiveStopResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };

    web::Json(result)
}
