use actix_web::{App, HttpServer};

use tracing;

use crate::cli::CommandLines;
use crate::http_handler;

pub async fn run_forever(cli_args: &CommandLines) -> Result<(), std::io::Error> {
    match HttpServer::new(|| {
        App::new()
            .service(http_handler::live::post_play)
            .service(http_handler::live::post_stop)
            .service(http_handler::live::post_keep_alive)
            .service(http_handler::replay::post_start)
            .service(http_handler::replay::post_stop)
            .service(http_handler::replay::post_keep_alive)
    })
    .bind((cli_args.host.clone(), cli_args.http_port))
    {
        Ok(h) => {
            tracing::info!("HttpServer::bind({}:{}) ok", &cli_args.host, cli_args.http_port);
            h.run().await
        }
        Err(e) => {
            tracing::error!("HttpServer::bind({}:{}) error, e: {:?}", &cli_args.host, cli_args.http_port, e);
            return Err(e);
        }
    }
}
