// The default armstrong config file. This file is copied to
// $XDG_HOME_CONFIG/armstrong/config.toml if the file does not already exist or
// $HOME/.config/armstrong/config.toml if $XDG_HOME_CONFIG is unset.
pub const DEFAULT_CONFIG_TOML: &str = r#"
[downloads]
download_dir = "$HOME/Downloads/"
"#;
