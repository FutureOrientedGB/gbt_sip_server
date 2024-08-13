use actix_web::{post, web, Responder};

use crate::http_message::replay::keep_alive::{ReplayKeepAliveRequest, ReplayKeepAliveResponse};

#[post("/replay/keep_alive")]
async fn post_keep_alive(data: web::Json<ReplayKeepAliveRequest>) -> impl Responder {
    let result = ReplayKeepAliveResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };

    web::Json(result)
}
