use actix_web::{post, web, Responder};

use crate::http_message::replay::start::{ReplayStartRequest, ReplayStartResponse};

#[post("/replay/start")]
async fn post_start(data: web::Json<ReplayStartRequest>) -> impl Responder {
    let result = ReplayStartResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: 0,
    };

    web::Json(result)
}
