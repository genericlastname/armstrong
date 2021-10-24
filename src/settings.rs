use std::env;
use std::fs::File;
use std::path::Path;

pub fn create_config_file(override_path: &str) {
    let mut config_path: &str;

    if override_path.is_empty() {
        let home_dir = env::var("HOME") {
            Ok(h) => h.to_owned(),
            Err(e) => return Err(e),
        }

        let config_dir = match env::var("XDG_HOME_CONFIG") {
            Ok(c) => c.to_owned(),
            Err(e) => format!("{}/.config", home_dir),
        }

        config_path = Path::new(format!("{}/armstrong/config.toml", config_dir));
    } else {
        config_path = Path::new(override_path);
    }

    let mut file = match File::create(&config_path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't create {}: {}",
                    config_path.display(),
                    e),
    }
    match file.write_all(settings::constants::DEFAULT_CONFIG_TOML.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't write to {}: {}",
                    config_path.display(),
                    e),
    }                    
}
