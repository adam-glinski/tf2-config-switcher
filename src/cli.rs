use clap::{ArgMatches, Arg, Command};
use crate::VERSION;

pub fn run() -> ArgMatches {
    Command::new("tf2configswitcher")
        .version(VERSION)
        .about("Config switcher made for Team Fortress 2 by r00t.")
        .arg(Arg::new("config_name")
             .help("config that you want to switch to.")
             .required(true)
             )
        .get_matches()
}
