use toml;

pub mod data {
    use std::vec;

    use serde::Deserialize;

    /* Top level unmarshal data struct for config file */
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct ConfigurationData {
        global: Global,
        platform_override: Option<Override>
    }

    /* Mid level unmarshal structs for config file */
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Global {
        mode: KindMode,
        package_manager_override: Option<String>,
        ensure: Option<vec::Vec<String>>
    }
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Override {
        windows: Option<Windows>,
        linux: Option<Linux>
    }
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Windows {
        mode: KindMode,
        ensure: Option<vec::Vec<String>>
    }
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Linux {
        mode: KindMode,
        ensure: Option<vec::Vec<String>>
    }

    /* Derivative types */
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub enum KindMode {
        Absolute,
        Relative
    }
}

pub fn unmarshal_toml_config_file(toml: String) -> Result<data::ConfigurationData, toml::de::Error> {
    let ret: Result<data::ConfigurationData, toml::de::Error> = toml::from_str(&toml);

    match ret {
        Ok(_) => {
            log::debug!("unmarshalled config file contents successfully: {:?}", &ret);
            return ret;
        }
        Err(e) => panic!("Could not unmarshal configuration data: {}", e.message()),
    };
}

#[cfg(test)]
mod tests {
    use crate::fileops;
    use super::*;

    #[test]
    fn test_unmarshal_toml_config_file() {
        let content = fileops::read_config_file("/home/nrexception/.config/ap.toml".into()).unwrap();
        let res = unmarshal_toml_config_file(content);
        assert!(res.is_ok());
    }
}
