pub mod cli;
pub mod log;
pub mod sip_handler;
pub mod sip_server;

const APP_NAME: &str = "gbt_sip_server";
const APP_VERSION: &str = "2024.8.12.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let cli_args = cli::CommandLines::new(&APP_NAME, &APP_VERSION);

    // open daily log
    log::open_daily_file_log(&APP_NAME, cli_args.port);

    // listen udp sock
    Ok(sip_server::run_forever(
        &cli_args.host,
        cli_args.port,
        &cli_args.user_name,
        &cli_args.password,
        &cli_args.algorithm,
        &cli_args.nonce,
        &cli_args.cnonce,
        &cli_args.realm,
    )
    .await?)
}
