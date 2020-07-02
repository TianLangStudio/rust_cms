use std::time::Duration;

use lazy_static::lazy_static;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::info;

use super::config_util;

pub type DbConnection = MysqlConnection;
pub type Pool =  r2d2::Pool<ConnectionManager<DbConnection>> ;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

lazy_static!  {
    pub static ref POOL:  Pool = {
                info!("db pool init");
                let connspec = config_util::APP_CONFIG.get_str("tl.app.db.url") .expect("db url is required");
                let manager = ConnectionManager::<DbConnection>::new(connspec);
                 r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create pool.")
    };
}
pub fn get_conn(pool: &Pool) -> Option<PooledConnection> {
     match pool.get_timeout(Duration::new(10, 0)) {
         Ok(conn) => Some(conn),
         Err(err) =>  None
     }
}
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

