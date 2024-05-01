use std::{env::consts::OS, fs};
use std::io::{self};
use log::debug;

const DEFAULT_CONFIG_NAMES: &[&str] = &["ap.toml", "alwayspacking.toml"];

pub fn read_config_file(path: String) -> Result<String, io::Error> {
    let file_read: String = std::fs::read_to_string(path)?;
    Ok(file_read)
}

pub fn get_default_config_file_location() -> Result<String, io::Error> {
    let os_culture = OS;
    let username = whoami::username();
    let config_home_paths: Vec<String>;
    let pathseperator: char;

    match os_culture {
        "windows" => { 
            pathseperator = '\\';
            config_home_paths = [
                format!("c:\\users\\{}", username),
                format!("c:\\users\\{}\\.config", username),
            ].into();
            debug!("os type: windows");
        }

        "linux" => {
            pathseperator = '/';
            config_home_paths = [
                format!("/home/{}", username),
                format!("/home/{}/.config", username),
            ].into();
            debug!("os type: linux");
        }

        // Assume nix path construction always...
        _ => {
            pathseperator = '/';
            config_home_paths = [
                format!("/home/{}", username),
                format!("/home/{}/.config", username),
            ].into();
            debug!("os type: defaulting (assuming *nix)");
        }
    }

    let mut constructed_file_paths: Vec<String> = Vec::new();
    for i in config_home_paths.iter() {
        for j in DEFAULT_CONFIG_NAMES.iter() {
            constructed_file_paths.push(format!("{}{}{}", i, pathseperator, j.to_string()));
        }
    }

    for i in constructed_file_paths.iter() {
        let md = fs::metadata(i);
        let _isvalid = match md {
            Ok(_) => {
                debug!("found valid config file: {}", i);
                return Ok(i.to_string());
            }
            Err(e) => {
                debug!("could not find valid config file with {}, ({})", i, e.to_string());
            }
        };
    }

    return Ok("".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_config_file_location_ok() {
        let res = get_default_config_file_location();
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_default_config_file_not_nil() {
        let res = get_default_config_file_location();
        assert_ne!(res.unwrap(), "");
    }

    #[test]
    fn test_read_config_file_ok() {
        let default_config = get_default_config_file_location().unwrap();
        let res = read_config_file(default_config);
        assert!(res.is_ok());
    }

    #[test]
    fn test_read_config_file_not_nil() {
        let default_config = get_default_config_file_location().unwrap();
        let res = read_config_file(default_config);
        assert_ne!(res.unwrap(), "");
    }
}
