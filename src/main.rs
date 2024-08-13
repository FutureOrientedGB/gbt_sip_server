pub mod cli;
pub mod log;
pub mod sip_handler;
pub mod sip_message;
pub mod sip_server;
pub mod http_handler;
pub mod http_message;
pub mod http_server;

const APP_NAME: &str = "gbt_sip_server";
const APP_VERSION: &str = "2024.8.12.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let cli_args = cli::CommandLines::new(&APP_NAME, &APP_VERSION);

    // open daily log
    log::open_daily_file_log(&APP_NAME, cli_args.sip_port);

    // run sip server
    let sip_service = sip_server::run_forever(&cli_args);

    // run http server
    let http_service = http_server::run_forever(&cli_args);
    
    // wait
    let _ = tokio::join!(sip_service, http_service);

    Ok(())
}
