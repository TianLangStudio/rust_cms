use config::{Config, ConfigBuilder};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_CONFIG: Config = {
        let config_default = Config::builder()
                .add_source(config::File::with_name("conf/application"))
                .build()
                .expect("请提高配置文件confg/application.yaml");
        let mut config_builder = Config::builder().add_source(config_default.clone());
        config_builder = match config_default.get_string("tl.app.mode") {
            Ok(value) => {
                let config_file_name = format!("conf/application_{}", value);
                config_builder.add_source(config::File::with_name(&config_file_name))
            }
            _ => {
                config_builder
                    .add_source(config::File::with_name("conf/application_dev"))
            }
        };
        config_builder
            .add_source(config::Environment::with_prefix("TL_APP"))
            .build()
            .unwrap()
    };
}

pub fn is_prod() -> bool {
    match APP_CONFIG.get_string("tl.app.mode") {
        Ok(value) if value == "prod" => true,
        _ => false,
    }
}
