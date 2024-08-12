use chrono;

use time;

use tracing_appender;
use tracing_subscriber;

pub fn open_daily_file_log(name: &str, port: i32) {
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
            format!("{name}.{port}.log"),
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
        .with_ansi(false)
        .init();

    log_dir.push(format!(
        "{name}.{port}.log.{}",
        chrono::Local::now().format("%Y-%m-%d")
    ));
    println!("logging to: {:?}", log_dir);
}
