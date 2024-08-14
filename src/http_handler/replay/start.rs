use actix_web::{post, web, Responder};

use crate::{http_message::replay::start::{ReplayStartRequest, ReplayStartResponse}, store::base::StoreEngine};

#[post("/replay/start")]
async fn post_start(data: web::Json<ReplayStartRequest>, store_engine: web::Data<std::sync::Arc<Box<dyn StoreEngine>>>) -> impl Responder {
    let stream_id = store_engine.invite(&data.gb_code, false).1;

    let result = ReplayStartResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: stream_id,
    };
    web::Json(result)
}
