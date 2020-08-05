use config::Config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_CONFIG: Config = {
        let mut _config = Config::new();
        _config
            .merge(config::File::with_name("conf/application"))
            .unwrap();

        match _config.get_str("tl.app.mode") {
            Ok(value) => {
                let config_file_name = format!("conf/application_{}", value);
                _config
                    .merge(config::File::with_name(&config_file_name))
                    .unwrap();
                if value == "prod" {
                    true
                } else {
                    false
                }
            }
            _ => {
                _config
                    .merge(config::File::with_name("conf/application_dev"))
                    .unwrap();
                false
            }
        };
        _config
            .merge(config::Environment::with_prefix("TL_APP"))
            .unwrap();
        _config
    };
}

pub fn is_prod() -> bool {
    match APP_CONFIG.get_str("tl.app.mode") {
        Ok(value) if value == "prod" => true,
        _ => false,
    }
}
