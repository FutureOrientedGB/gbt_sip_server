use actix_web::{post, web, Responder};

use crate::{http_message::live::stop::{LiveStopRequest, LiveStopResponse}, store::base::StoreEngine};

#[post("/live/stop")]
async fn post_stop(data: web::Json<LiveStopRequest>, store_engine: web::Data<std::sync::Arc<Box<dyn StoreEngine>>>) -> impl Responder {
    store_engine.bye(&data.gb_code, data.stream_id);

    let result = LiveStopResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };
    web::Json(result)
}
