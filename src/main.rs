pub mod cli;
pub mod http_handler;
pub mod http_message;
pub mod http_server;
pub mod log;
pub mod sip_handler;
pub mod sip_message;
pub mod sip_server;
pub mod store;

const APP_NAME: &str = "gbt_sip_server";
const APP_VERSION: &str = "2024.8.14.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let cli_args = cli::CommandLines::new(&APP_NAME, &APP_VERSION);

    // open daily log
    log::open_daily_file_log(&APP_NAME, cli_args.sip_port);

    // parepare sip server
    let mut sip_server = sip_server::SipServer::default();
    let sip_socket = sip_server.bind(&cli_args).await?;
    let sip_socket_arc = std::sync::Arc::new(sip_socket);

    // connect store
    let store_engine = store::create_store(&cli_args, sip_socket_arc.clone());
    let store_engine_arc = std::sync::Arc::new(store_engine);
    if !store_engine_arc.clone().is_connected() {
        tracing::error!("create_store error");
        return Ok(());
    }

    // run sip server
    let sip_service = sip_server.run_forever(&cli_args, sip_socket_arc.clone(), store_engine_arc.clone());

    // run http server
    let http_service = http_server::run_forever(&cli_args, sip_socket_arc.clone(), store_engine_arc.clone());

    // wait
    let _ = tokio::join!(sip_service, http_service);

    Ok(())
}
