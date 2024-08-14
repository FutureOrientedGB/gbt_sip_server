pub mod http;
pub mod sip;
pub mod store;
pub mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let app_name: &str = "gbt_sip_server";
    let app_version = "6ce6093.20240814.233429";
    let cli_args = utils::cli::CommandLines::new(&app_name, &app_version);

    // open daily log
    utils::log::open_daily_file_log(&app_name, cli_args.sip_port, app_version);

    // prepare sip server
    let sip_socket = sip::server::bind(&cli_args).await?;
    let sip_socket_arc = std::sync::Arc::new(sip_socket);

    // connect store
    let store_engine = store::create_store(&cli_args, sip_socket_arc.clone());
    let store_engine_arc = std::sync::Arc::new(store_engine);
    if !store_engine_arc.clone().is_connected() {
        tracing::error!("create_store error");
        return Ok(());
    }

    // run sip server
    let sip_service = sip::server::run_forever(&cli_args, sip_socket_arc.clone(), store_engine_arc.clone());

    // run http server
    let http_service = http::server::run_forever(&cli_args, sip_socket_arc.clone(), store_engine_arc.clone());

    // wait
    let _ = tokio::join!(sip_service, http_service);

    Ok(())
}
