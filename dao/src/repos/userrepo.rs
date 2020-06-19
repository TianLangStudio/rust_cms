use crate::models::usermod::NewLoginInfo;
use crate::schema::tb_login_info;
use diesel::prelude::*;
use diesel::result::Error;
use common::{sign_util, db_util};
use log::{info, warn};


pub type DbConnection = db_util::DbConnection;

pub fn add_login_info(conn:  &DbConnection, new_login_info: &NewLoginInfo) -> Result<usize,  Error>
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

pub fn change_password(conn: &DbConnection, login_info_id: i64,  new_password: &str ) -> Result<usize, Error> 
{
        use self::tb_login_info::dsl::*;
        let target = tb_login_info.filter(id.eq(login_info_id));
         let signed_passwd = signed_password(new_password);
        diesel::update(target).set(password.eq(signed_passwd)).execute(conn)
}

pub fn remove_login_info(conn: &DbConnection, login_info_id: i64) ->Result<usize, Error> {
    use self::tb_login_info::dsl::*;
    let target = tb_login_info.filter(id.eq(login_info_id));
    diesel::delete(target).execute(conn)
}

pub fn valid_login_info(conn: &DbConnection,  uname: &str, passwd: &str)  -> bool {

    let signed_passwd = signed_password(passwd);
    use crate::models::usermod::LoginInfoModel;
    use self::tb_login_info::dsl::*;
    match tb_login_info.filter(username.eq(uname))
                         .filter(password.eq(signed_passwd))
                        .load::<LoginInfoModel>(conn)   {
                                Ok(login_infos) if login_infos.len() == 1 => true,
                                Ok(login_infos) if login_infos.len() > 1 => {
                                    warn!("duplicat username {}", uname);
                                    false
                                }
                                _ => false
                        }
}

fn signed_password(passwd: &str) -> String {
    sign_util::blake2_sign(passwd)
}
