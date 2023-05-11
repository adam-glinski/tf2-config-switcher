use crate::data_types::Settings;
use std::{fs::File, io::Read};

fn default_config() -> Settings {
    Settings {
        tf2_path: "C:\\Program Files\\Steam\\common\\Team Fortres 2\\".to_string(),
        safe_move: true,
        configs_directory: "C:\\Program Files\\Steam\\common\\Team Fortres 2\\.configs\\"
            .to_string(),
    }
}

pub fn save(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("settings.json").expect("Failed to get file handle!");
    serde_json::to_writer(file, &settings).expect("Failed to crate file!");

    Ok(())
}

pub fn load() -> Result<Settings, Box<dyn std::error::Error>> {
    let mut settings = Settings::default();
    match File::open("settings.json") {
        Ok(mut file) => {
            println!("[OK] Found settings");
            let mut temp = String::new();
            file.read_to_string(&mut temp)?;
            settings = serde_json::from_str(&temp).unwrap();
        }
        Err(error) => {
            if let std::io::ErrorKind::NotFound = error.kind() {
                println!("[INFO] No settings, loading default ones.");
                settings = default_config();
            }
        }
    }

    dbg!(&settings);
    Ok(settings)
}
