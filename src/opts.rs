use std::path::PathBuf;
use structopt:: StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Cryptography Learning", about = "Using for cryptography learning")]
pub struct Opt {
  /// debug level by number of `l', default debug level is error,
  /// 1, 2, 3, 4 correspond to warn, info, debug, trace respectively
  #[structopt(short, long, parse(from_occurrences))]
  log_level: u32,

  /// configuration file's, check example for more details
  #[structopt(
    default_value = "config.toml",
    env = "CONFIG",
    parse(from_os_str))
  ]
  config: PathBuf,
}