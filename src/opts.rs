use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Cryptography Learning",
    about = "Using for cryptography learning"
)]
pub struct Opt {
    /// logging level by number of `v', default logging level is error,
    /// 1, 2, 3, 4 correspond to warn, info, debug, trace respectively
    #[structopt(short = "v", parse(from_occurrences))]
    pub log_level: u32,

    /// configuration file's, check example for more details
    #[structopt(default_value = "config.ron", env = "CONFIG", parse(from_os_str))]
    pub config: PathBuf,

    /// static file path to be serve
    #[structopt(short = "s", long = "static", env = "STATIC")]
    pub static_file_path: String,
}
