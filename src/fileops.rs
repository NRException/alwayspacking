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
            debug!("OS Type: Windows");
        }

        "linux" => {
            pathseperator = '/';
            config_home_paths = [
                format!("/home/{}", username),
                format!("/home/{}/.config", username),
            ].into();
            debug!("OS Type: Linux");
        }

        // Assume nix path construction always...
        _ => {
            pathseperator = '/';
            config_home_paths = [
                format!("/home/{}", username),
                format!("/home/{}/.config", username),
            ].into();
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
                debug!("Found valid config file {}", i);
                return Ok(i.to_string());
            }
            Err(e) => {
                debug!("Could not find valid config file with {}, ({})", i, e.to_string());
            }
        };
    }

    return Ok("".to_string());
}
