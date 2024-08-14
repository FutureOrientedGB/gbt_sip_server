use actix_web::{web, App, HttpServer};

use tracing;

use crate::http;
use crate::store::base::StoreEngine;
use crate::utils::cli::CommandLines;

pub async fn run_forever(
    cli_args: &CommandLines,
    sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
    store_engine: std::sync::Arc<Box<dyn StoreEngine>>,
) -> Result<(), std::io::Error> {
    match HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sip_socket.clone()))
            .app_data(web::Data::new(store_engine.clone()))
            .service(http::handler::live::post_play)
            .service(http::handler::live::post_stop)
            .service(http::handler::live::post_keep_alive)
            .service(http::handler::replay::post_start)
            .service(http::handler::replay::post_stop)
            .service(http::handler::replay::post_keep_alive)
    })
    .bind((cli_args.host.clone(), cli_args.http_port))
    {
        Ok(h) => {
            tracing::info!(
                "HttpServer::bind({}:{}) ok",
                &cli_args.host,
                cli_args.http_port
            );
            h.run().await
        }
        Err(e) => {
            tracing::error!(
                "HttpServer::bind({}:{}) error, e: {:?}",
                &cli_args.host,
                cli_args.http_port,
                e
            );
            return Err(e);
        }
    }
}
