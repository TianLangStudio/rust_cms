use super::models::NewLoginInfo;
use super::schema::tb_login_info;
use diesel::prelude::*;
use diesel::result::Error;
use common::sign_util;
use log::info;

pub fn add_login_info(conn:  &MysqlConnection, new_login_info: &NewLoginInfo) -> Result<usize,  Error>
 {
        info!("add login info username:{}",    new_login_info.username);
        let signed_login_info = NewLoginInfo {
            username: new_login_info.username,
            password: &sign_util::blake2_sign(new_login_info.password),
        };
       diesel::insert_into(tb_login_info::table)   
                     .values(&signed_login_info)
                     .execute(conn) 
}



