use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub tf2_path: String,          // Path to the tf2 directory
    pub safe_move: bool, // True -> Copy instead of moving || False -> Move the config file
    pub configs_directory: String, // directory to save your configs
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tf2_path: "C:\\Program Files\\Steam\\common\\Team Fortres 2\\".to_string(),
            safe_move: true,
            configs_directory: "C:\\Program Files\\Steam\\common\\Team Fortres 2\\.configs\\"
                .to_string(),
        }
    }
}
