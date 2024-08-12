use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandLines {
    #[structopt(long, default_value = "0.0.0.0")]
    pub host: String,

    #[structopt(long, default_value = "5060")]
    pub port: i32,
}

impl CommandLines {
    pub fn new(name: &str, version: &str) -> CommandLines {
        let cli_app = CommandLines::clap().name(name).version(version);
        CommandLines::from_clap(&cli_app.get_matches())
    }
}
