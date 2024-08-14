use actix_web::{post, web, Responder};

use crate::{http_message::live::play::{LivePlayRequest, LivePlayResponse}, store::base::StoreEngine};

#[post("/live/play")]
async fn post_play(data: web::Json<LivePlayRequest>, _store_engine: web::Data<std::sync::Arc<Box<dyn StoreEngine>>>) -> impl Responder {
    let result = LivePlayResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: 0,
    };

    web::Json(result)
}
