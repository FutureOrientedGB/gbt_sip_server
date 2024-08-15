use chrono;

use time;

use tracing_appender;
use tracing_subscriber;

use crate::utils::ansi_color as Color;

pub fn open_daily_file_log(app_name: &str, app_version: &str, sip_port: u16) {
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
            format!("{app_name}.{sip_port}.log"),
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
        "{app_name}.{sip_port}.log.{}",
        chrono::Local::now().format("%Y-%m-%d")
    ));
    println!(
        "{}logging to: {}{}",
        Color::PURPLE,
        log_dir.to_str().unwrap(),
        Color::RESET
    );

    tracing::info!(
        "start services{}
+────────────────────────────────────────+
│ ┌─┐┌┐┌┬┐  ┌─┐┬┌─┐  ┌─┐┌─┐┬─┐┬  ┬┌─┐┬─┐ │
│ │ ┬├┴┐│   └─┐│├─┘  └─┐├┤ ├┬┘└┐┌┘├┤ ├┬┘ │
│ └─┘└─┘┴   └─┘┴┴    └─┘└─┘┴└─ └┘ └─┘┴└─ │
│        {}         │
+────────────────────────────────────────+{}",
        Color::PURPLE,
        app_version,
        Color::RESET
    );
}
