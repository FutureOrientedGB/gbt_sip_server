use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct CommandLines {
    #[structopt(long, default_value = "memory", help = "memory, postgre or redis")]
    pub store_engine: String,

    #[structopt(long, default_value = "", help = "connect url for store_engine, like redis://user:pass@host:port/db or postgresql://user:pass@host:port/db")]
    pub store_url: String,

    #[structopt(long, default_value = "0.0.0.0")]
    pub host: String,

    #[structopt(long, default_value = "5060")]
    pub sip_port: u16,

    #[structopt(long, default_value = "8080")]
    pub http_port: u16,

    #[structopt(long, default_value = "3402000000")]
    pub sip_domain: String,

    #[structopt(long, default_value = "34020000002000000001")]
    pub sip_id: String,

    #[structopt(long, default_value = "d383cf85b0e8ce0b")]
    pub sip_password: String,

    #[structopt(long, default_value = "md5")]
    pub sip_algorithm: String,

    #[structopt(long, default_value = "f89d0eaccaf1c90453e2f84688ec800f05")]
    pub sip_nonce: String,

    #[structopt(long, default_value = "gbt@future_oriented.com")]
    pub sip_realm: String,
}

impl CommandLines {
    pub fn new(name: &str, version: &str) -> CommandLines {
        let cli_app = CommandLines::clap().name(name).version(version);
        CommandLines::from_clap(&cli_app.get_matches())
    }
}
