use clap::ArgMatches;

use crate::{cli, settings};

fn extract_config_name(matches: &ArgMatches) -> &String {
    return matches
        .get_one::<String>("config_name")
        .unwrap_or_else(|| panic!("Failed to extract config name from matches."));
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: Fix me
    let settings = settings::load().unwrap();
    let matches = cli::run();
    let config_name = extract_config_name(&matches);
    dbg!(&settings);

    settings::save(&settings)
}
