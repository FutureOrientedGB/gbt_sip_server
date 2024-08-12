use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandLines {
    #[structopt(long, default_value = "0.0.0.0")]
    pub host: String,

    #[structopt(long, default_value = "5060")]
    pub port: i32,

    #[structopt(long, default_value = "ce665764")]
    pub user_name: String,

    #[structopt(long, default_value = "d383cf85b0e8ce0b")]
    pub password: String,

    #[structopt(long, default_value = "md5")]
    pub algorithm: String,

    #[structopt(long, default_value = "f89d0eaccaf1c90453e2f84688ec800f05")]
    pub nonce: String,

    #[structopt(long, default_value = "edf7270a")]
    pub cnonce: String,

    #[structopt(long, default_value = "gbt@future_oriented.com")]
    pub realm: String,
}

impl CommandLines {
    pub fn new(name: &str, version: &str) -> CommandLines {
        let cli_app = CommandLines::clap().name(name).version(version);
        CommandLines::from_clap(&cli_app.get_matches())
    }
}
