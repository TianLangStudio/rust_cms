use std::sync::OnceLock;
use std::time::Duration;

use crate::config_util::get_app_config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use log::info;

pub type DbConnection = MysqlConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;
// Define the static POOL with OnceLock
static POOL: OnceLock<Pool> = OnceLock::new();
// Usage example (you’d call this wherever POOL is accessed)
pub fn get_pool() -> &'static Pool {
    POOL.get_or_init(init_pool)
}
fn init_pool() -> Pool {
    info!("db pool init");
    let conn_url = get_app_config()
        .get_string("tl.app.db.url")
        .expect("db url is required");
    let manager = ConnectionManager::<DbConnection>::new(conn_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
pub fn get_conn(pool: &Pool) -> Option<PooledConnection> {
    pool.get_timeout(Duration::new(10, 0)).ok()
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
