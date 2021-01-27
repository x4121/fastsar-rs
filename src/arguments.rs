use log::LevelFilter;
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

    /// Override detected shell
    #[structopt(short, long)]
    pub shell: Option<String>,

    /// Override default region
    #[structopt(short = "R", long)]
    pub region: Option<String>,

    #[structopt(short, long)]
    pub role: Option<String>,

    #[structopt(short, long)]
    pub account: Option<String>,

    /// MFA serial number (ARN)
    #[structopt(short, long)]
    pub mfa_id: Option<String>,

    #[structopt(short = "t", long)]
    pub mfa_token: Option<String>,

    /// Execute command after assuming role instead of printing set-env statements
    #[structopt(short = "x", long)]
    pub exec: Option<String>,

    /// Print debug output
    #[structopt(short, long)]
    debug: bool,
}

impl Arguments {
    pub fn get_config_path(&self) -> PathBuf {
        Arguments::tilde(&self.config_path)
    }

    pub fn get_debug(&self) -> LevelFilter {
        if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        }
    }

    fn tilde(path: &PathBuf) -> PathBuf {
        PathBuf::from(shellexpand::tilde(path.to_str().unwrap()).to_string())
    }
}
