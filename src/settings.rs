// The default armstrong config file. This file is copied to
// $XDG_HOME_CONFIG/armstrong/config.toml if the file does not already exist or
// $HOME/.config/armstrong/config.toml if $XDG_HOME_CONFIG is unset.
const DEFAULT_CONFIG_TOML: &str = r#"
[downloads]
download_dir = "$HOME/Downloads/"
"#;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// use cursive::theme::{Color, Palette, Theme};

pub fn create_config_file(override_path: &str) {
    let config_path: &Path;
    let config_path_string: String;

    if override_path.is_empty() {
        let home_dir = env::var("HOME").unwrap();
        config_path_string = format!("{}/.config/armstrong/config.toml", 
                                    home_dir);
        config_path = Path::new(&config_path_string);
    } else {
        config_path = Path::new(override_path);
    }

    let mut file = File::create(&config_path).unwrap();
    match file.write(DEFAULT_CONFIG_TOML.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't write to {}: {}",
                    config_path.display(),
                    e),
    }                    
}

// pub fn load_theme() -> Theme {
//     let mut palette = Palette::default();
//     palette.set_color
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn create_config_file_works() {
        create_config_file("/tmp/config.toml");
        let mut config = File::open("/tmp/config.toml").unwrap();
        let mut s = String::new();
        config.read_to_string(&mut s).expect("Couldn't open file.");
        assert_eq!(s, DEFAULT_CONFIG_TOML);
    }
}
