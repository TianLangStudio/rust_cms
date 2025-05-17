use std::sync::OnceLock;
use config::{Config, ConfigBuilder, Value};
static APP_CONFIG: OnceLock<Config> = OnceLock::new();
pub fn get_app_config() -> &'static Config {
    APP_CONFIG.get_or_init(init_app_config)
}
fn init_app_config() -> Config {
    let config_default = Config::builder()
        .add_source(config::File::with_name("conf/application"))
        .build()
        .expect("请提供配置文件confg/application.yaml");
    let mut config_builder = Config::builder().add_source(config_default.clone());
    config_builder = match config_default.get_string("tl.app.mode") {
        Ok(value) => {
            let config_file_name = format!("conf/application_{}", value);
            config_builder.add_source(config::File::with_name(&config_file_name))
        }
        _ => config_builder.add_source(config::File::with_name("conf/application_dev")),
    };
    config_builder
        .add_source(config::Environment::with_prefix("TL_APP"))
        .build()
        .unwrap()
}
pub fn is_prod() -> bool {
    matches!(get_app_config().get_string("tl.app.mode"), Ok(val) if val == "prod")
}

pub fn need_approval() -> bool {
    matches!(get_app_config().get_bool("tl.app.approval.enable"), Ok(val))
}

//set
pub fn is_approve_user(username: &str) -> bool {
    match get_app_config().get_string("tl.app.approval.users") {
        Ok(values) => values.split(",").any(|v| v == username),
        _ => false,
    }
}
