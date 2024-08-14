use actix_web::{post, web, Responder};

use crate::{http_message::live::play::{LivePlayRequest, LivePlayResponse}, store::base::StoreEngine};

#[post("/live/play")]
async fn post_play(data: web::Json<LivePlayRequest>, store_engine: web::Data<std::sync::Arc<Box<dyn StoreEngine>>>) -> impl Responder {
    let (_exists, stream_id) = store_engine.invite(&data.gb_code, true);

    let result = LivePlayResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: stream_id,
    };
    web::Json(result)
}
