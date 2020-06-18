use lazy_static::lazy_static;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::info;

use super::config_util;

lazy_static!  {
    pub static ref POOL:  r2d2::Pool<ConnectionManager<MysqlConnection>> = {
                info!("db pool init");
                let connspec = config_util::APP_CONFIG.get_str("tl.app.db.url") .expect("db url is required");
                let manager = ConnectionManager::<MysqlConnection>::new(connspec);
                 r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create pool.")
    };
}