use actix_web::{post, web, Responder};

use crate::http_message::live::keep_alive::{LiveKeepAliveRequest, LiveKeepAliveResponse};

#[post("/live/keep_alive")]
async fn post_keep_alive(data: web::Json<LiveKeepAliveRequest>) -> impl Responder {
    let result = LiveKeepAliveResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };

    web::Json(result)
}
