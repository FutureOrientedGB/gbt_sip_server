use actix_web::{post, web, Responder};

use crate::http_message::replay::stop::{ReplayStopRequest, ReplayStopResponse};

#[post("/replay/stop")]
async fn post_stop(data: web::Json<ReplayStopRequest>) -> impl Responder {
    let result = ReplayStopResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };

    web::Json(result)
}
