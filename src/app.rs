use clap::ArgMatches;

use crate::cli;

fn extract_config_name(matches: &ArgMatches) -> &String {
    return matches
        .get_one::<String>("config_name")
        .unwrap_or_else(|| panic!("Failed to extract config name from matches."));
}

pub fn run() {
    let matches = cli::run();
    let _config_name = extract_config_name(&matches);
}
