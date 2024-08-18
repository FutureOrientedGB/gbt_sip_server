use chrono;

use time;

use tracing_appender;
use tracing_subscriber;

use crate::utils::ansi_color as Color;

use super::cli::CommandLines;

pub fn open_daily_file_log(app_name: &str, app_version: &str, cli_args: &CommandLines) {
    let mut log_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    log_dir.push("log");

    tracing_subscriber::fmt()
        // .json()
        .with_writer(tracing_appender::rolling::daily(
            &log_dir,
            format!("{app_name}.{sip_port}.log", sip_port = cli_args.sip_port),
        ))
        .with_max_level(tracing::Level::INFO)
        .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
            time::UtcOffset::from_hms(8, 0, 0).unwrap(),
            time::macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
            ),
        ))
        .with_line_number(true)
        .with_thread_ids(true)
        // .with_ansi(false)
        .init();

    log_dir.push(format!(
        "{app_name}.{sip_port}.log.{date}",
        sip_port = cli_args.sip_port,
        date = chrono::Local::now().format("%Y-%m-%d")
    ));
    println!(
        "{}logging to: {}{}",
        Color::PURPLE,
        log_dir.to_str().unwrap(),
        Color::RESET
    );

    tracing::info!(
        "start services{}
+──────────────────────────────────────────────────+
│      ┌─┐┌┐┌┬┐  ┌─┐┬┌─┐  ┌─┐┌─┐┬─┐┬  ┬┌─┐┬─┐      │
│      │ ┬├┴┐│   └─┐│├─┘  └─┐├┤ ├┬┘└┐┌┘├┤ ├┬┘      │
│      └─┘└─┘┴   └─┘┴┴    └─┘└─┘┴└─ └┘ └─┘┴└─      │
│             {}              │
+──────────────────────────────────────────────────+
⮞ store_engine: {}
⮞ store_url: {}
⮞ user_agent: {}
⮞ host: {}
⮞ sip_ip: {}
⮞ sip_port: {}
⮞ sip_domain: {}
⮞ sip_id: {}
⮞ sip_password: {}
⮞ sip_algorithm: {}
⮞ sip_nonce: {}
⮞ sip_realm: {}
⮞ call_id: {}
⮞ socket_recv_buffer_size: {}
+──────────────────────────────────────────────────+{}",
        Color::PURPLE,
        app_version,
        &cli_args.store_engine,
        &cli_args.store_url,
        &cli_args.user_agent,
        &cli_args.host,
        &cli_args.sip_ip,
        &cli_args.sip_port,
        &cli_args.sip_domain,
        &cli_args.sip_id,
        &cli_args.sip_password,
        &cli_args.sip_algorithm,
        &cli_args.sip_nonce,
        &cli_args.sip_realm,
        &cli_args.call_id,
        &cli_args.socket_recv_buffer_size,
        Color::RESET
    );
}
