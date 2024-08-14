pub mod base;
pub mod memory;
pub mod not_impl;
pub mod postgre;
pub mod redis;

use crate::cli;

pub fn create_store(
    cli_args: &cli::CommandLines,
    sip_socket: std::sync::Arc<tokio::net::UdpSocket>,
) -> Box<dyn base::StoreEngine> {
    match cli_args.store_engine.as_str() {
        "memory" => {
            return Box::new(memory::MemoryStore::new(sip_socket, cli_args));
        }
        "postgre" => {
            return Box::new(postgre::PostgreStore::new(sip_socket, cli_args));
        }
        "redis" => {
            return Box::new(redis::RedisStore::new(sip_socket, cli_args));
        }
        _ => {
            tracing::error!("not impl store engine: {}", &cli_args.store_engine);
            return Box::new(not_impl::NotImplStore::new(sip_socket, cli_args));
        }
    }
}
