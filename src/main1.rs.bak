use chrono::Local;

use encoding_rs::GB18030;

use structopt::StructOpt;

use time::{macros::format_description, UtcOffset};

use tracing::Level;
use tracing_appender;
use tracing_subscriber::{self, fmt::time::OffsetTime};

use tokio::net::UdpSocket;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, default_value = "0.0.0.0")]
    host: String,

    #[structopt(long, default_value = "5060")]
    port: i32,
}

pub struct GbHander {}

impl GbHander {
    pub fn on_request(headers: Vec<(String, String)>, body: String) -> String {
        match headers[0].0.as_str() {
            "REGISTER" => {
                return Self::on_register(headers, body);
            }
            _ => {
                tracing::error!("invalid request, headers: {:?}, body: {:?}", headers, body);
                return String::from("");
            }
        }
    }

    pub fn on_register(headers: Vec<(String, String)>, body: String) -> String {
        return String::from("");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let name = "gbt_sip_server";
    let version = "2024.8.9.1";
    let cli_app = Cli::clap().name(name).version(version);
    let cli_args = Cli::from_clap(&cli_app.get_matches());

    // open daily log
    let log_dir = format!(
        "{}/log",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned()
            .into_os_string()
            .into_string()
            .unwrap()
            .to_string()
    );
    tracing_subscriber::fmt()
        .json()
        .with_writer(tracing_appender::rolling::daily(
            &log_dir,
            format!("{name}.{port}.log", port = cli_args.port),
        ))
        .with_max_level(Level::INFO)
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
        ))
        .with_line_number(true)
        .with_thread_ids(true)
        .with_ansi(false)
        .init();
    println!(
        "loggging to: {}/{}.{}.log.{}",
        &log_dir,
        &name,
        &cli_args.port,
        Local::now().format("%Y-%m-%d")
    );

    // listen udp sock
    match UdpSocket::bind(format!(
        "{host}:{port}",
        host = &cli_args.host,
        port = &cli_args.port
    ))
    .await
    {
        Err(e) => {
            tracing::error!(
                "UdpSocket::bind({}:{}) error, e: {:?}",
                &cli_args.host,
                &cli_args.port,
                e
            );
            return Ok(());
        }
        Ok(socket) => {
            tracing::info!(
                "UdpSocket::bind({}:{}) ok, addr: {:?}",
                &cli_args.host,
                &cli_args.port,
                socket.local_addr()?
            );

            let mut buf = [0; 65535];
            loop {
                let (amount, _src_addr) = socket.recv_from(&mut buf).await?;

                let (text, _encoding, has_error) = GB18030.decode(&buf[..amount]);
                if has_error {
                    tracing::error!("GB18030.decode error");
                    continue;
                }

                if let Some(pos) = text.find("Content-Length") {
                    // content
                    let mut content = String::from("");
                    if let Some(i) = text[pos..].find("\n") {
                        content = text[pos + i + 1..].trim().to_owned();
                    }

                    // headers
                    let mut headers = vec![];
                    for (index, line) in text[..pos].lines().enumerate() {
                        if 0 == index {
                            if let Some(pos) = line.find(" ") {
                                headers.push((
                                    line[0..pos].trim().to_owned(),
                                    line[pos + 1..].trim().to_owned(),
                                ));
                            }
                        } else {
                            if let Some(pos) = line.find(":") {
                                headers.push((
                                    line[0..pos].trim().to_owned(),
                                    line[pos + 1..].trim().to_owned(),
                                ));
                            }
                        }
                    }

                    // handle
                    GbHander::on_request(headers, content);
                }
            }
        }
    }
}
