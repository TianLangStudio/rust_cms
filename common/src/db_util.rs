use std::time::Duration;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use lazy_static::lazy_static;
use log::info;

use super::config_util;

pub type DbConnection = MysqlConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

lazy_static! {
    pub static ref POOL: Pool = {
        info!("db pool init");
        let connspec = config_util::APP_CONFIG
            .get_string("tl.app.db.url")
            .expect("db url is required");
        let manager = ConnectionManager::<DbConnection>::new(connspec);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    };
}

pub fn get_conn(pool: &Pool) -> Option<PooledConnection> {
    match pool.get_timeout(Duration::new(10, 0)) {
        Ok(conn) => Some(conn),
        Err(err) => None,
    }
}

pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn page2limit_offset(page_no: i64, page_size: i64) -> (i64, i64) {
    let page_no = page_no - 1;
    let page_no = 0.max(page_no);
    let page_size = 1000.min(page_size).max(0);
    info!("page_no:{}, page_size:{}", &page_no, &page_size);

    (page_size, page_no * page_size)
}
