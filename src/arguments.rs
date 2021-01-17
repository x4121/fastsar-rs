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

    #[structopt(short, long, about = "Overwrite shell detection")]
    pub shell: Option<String>,

    #[structopt(short = "R", long, about = "Overwrite default region")]
    pub region: Option<String>,

    #[structopt(short, long)]
    pub role: Option<String>,

    #[structopt(short, long)]
    pub account: Option<String>,

    #[structopt(
        short = "x",
        long,
        about = "Execute command after assuming role instead of printing set-env statements"
    )]
    pub exec: Option<String>,
}

impl Arguments {
    pub fn get_config_path(&self) -> PathBuf {
        Arguments::tilde(&self.config_path)
    }

    fn tilde(path: &PathBuf) -> PathBuf {
        PathBuf::from(shellexpand::tilde(path.to_str().unwrap()).to_string())
    }
}
