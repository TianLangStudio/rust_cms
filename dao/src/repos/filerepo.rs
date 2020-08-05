use diesel::prelude::*;
use diesel::result::Error;
use log::warn;

use crate::models::filemod::{FileMod, NewFileMod};
use crate::schema::tb_file;
use common::db_util;
pub type DbConnection = db_util::DbConnection;

pub fn add_file(conn: &db_util::PooledConnection, new_file: &NewFileMod) -> Result<usize, Error> {
    diesel::insert_into(tb_file::table)
        .values(new_file)
        .execute(conn)
}

pub fn load_file_by_id(conn: &DbConnection, file_id: &str) -> Option<FileMod> {
    use self::tb_file::dsl::*;

    match tb_file.find(file_id).first::<FileMod>(conn) {
        Ok(file_mod) => Some(file_mod),
        Err(err) => {
            warn!("can not find file by id:[{}]", file_id);
            None
        }
    }
}
