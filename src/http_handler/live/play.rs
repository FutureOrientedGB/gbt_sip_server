use actix_web::{post, web, Responder};

use crate::http_message::live::play::{LivePlayRequest, LivePlayResponse};

#[post("/live/play")]
async fn post_play(data: web::Json<LivePlayRequest>) -> impl Responder {
    let result = LivePlayResponse {
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: 0,
    };

    web::Json(result)
}
