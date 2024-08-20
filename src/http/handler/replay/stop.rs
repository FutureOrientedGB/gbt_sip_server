use actix_web::{post, web, Responder};

use crate::{
    http::message::replay::stop::{ReplayStopRequest, ReplayStopResponse},
    sip::handler::SipHandler,
};

#[post("/replay/stop")]
async fn post_stop(
    data: web::Json<ReplayStopRequest>,
    sip_handler: web::Data<std::sync::Arc<SipHandler>>,
) -> impl Responder {
    if let (by_to_device, Some((branch, device_addr, tcp_stream))) =
        sip_handler.store.bye(&data.gb_code, data.stream_id)
    {
        if by_to_device {
            sip_handler
                .send_bye(device_addr, tcp_stream, &branch, &data.gb_code)
                .await;
        }
    }

    let result = ReplayStopResponse {
        locate: format!("{}#L{}", file!(), line!()),
        code: 0,
        msg: String::from("OK"),
        gb_code: data.gb_code.clone(),
        stream_id: data.stream_id,
    };
    web::Json(result)
}
