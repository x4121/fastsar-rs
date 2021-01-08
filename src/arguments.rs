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
    pub shell: str,

    #[structopt(short = "r", long = "region")]
    pub region: str,

    #[structopt(short = "a", long = "account")]
    pub account: str,
}

impl Arguments {
    pub fn get_groups_path(&self) -> PathBuf {
        Arguments::tilde(&self.groups_path)
    }

    pub fn get_toml_path(&self) -> PathBuf {
        Arguments::tilde(&self.toml_path)
    }

    fn tilde(path: &PathBuf) -> PathBuf {
        PathBuf::from(shellexpand::tilde(path.to_str().unwrap()).to_string())
    }
}
