use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "skastsar", about = "skim AWS sts AssumeRole")]
pub struct Arguments {
    #[structopt(
        short = "c",
        long = "config",
        default_value = "~/.aws/sts.json",
        parse(from_os_str)
    )]
    config_path: PathBuf,

    #[structopt(short = "s", long = "shell")]
    pub shell: Option<String>,

    #[structopt(short = "r", long = "region")]
    pub region: Option<String>,

    #[structopt(short = "a", long = "account")]
    pub account: Option<String>,
}

impl Arguments {
    pub fn get_config_path(&self) -> PathBuf {
        Arguments::tilde(&self.config_path)
    }

    fn tilde(path: &PathBuf) -> PathBuf {
        PathBuf::from(shellexpand::tilde(path.to_str().unwrap()).to_string())
    }
}
