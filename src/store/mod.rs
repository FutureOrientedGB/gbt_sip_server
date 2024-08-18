pub mod base;
pub mod memory;
pub mod not_impl;
pub mod pg;
pub mod redis;

use crate::utils::cli::CommandLines;

pub fn create_store(
    cli_args: &CommandLines,
) -> Box<dyn base::StoreEngine> {
    match cli_args.store_engine.as_str() {
        "memory" => {
            return Box::new(memory::MemoryStore::new(cli_args));
        }
        "postgresql" => {
            return Box::new(pg::PostgreSqlStore::new(cli_args));
        }
        "redis" => {
            return Box::new(redis::RedisStore::new(cli_args));
        }
        _ => {
            tracing::error!("not impl store engine: {}", &cli_args.store_engine);
            return Box::new(not_impl::NotImplStore::new(cli_args));
        }
    }
}
