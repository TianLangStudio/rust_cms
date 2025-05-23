use crate::models::usermod::NewLoginInfo;
use crate::schema::tb_login_info;
use common::{db_util, sign_util};
use diesel::prelude::*;
use diesel::result::Error;
use log::{info, warn};

pub type DbConnection = db_util::DbConnection;

pub fn add_login_info(
    conn: &mut DbConnection,
    new_login_info: &NewLoginInfo,
) -> Result<usize, Error> {
    info!("add login info username:{}", new_login_info.username);
    let signed_login_info = NewLoginInfo {
        username: new_login_info.username,
        password: &signed_password(new_login_info.password, new_login_info.username),
    };
    diesel::insert_into(tb_login_info::table)
        .values(&signed_login_info)
        .execute(conn)
}

pub fn change_password(
    conn: &mut DbConnection,
    login_info_id: i64,
    old_password: &str,
    new_password: &str,
) -> Result<bool, String> {
    use self::tb_login_info::dsl::*;
    use crate::models::usermod::LoginInfoModel;
    let target = tb_login_info.find(login_info_id);
    match target.load::<LoginInfoModel>(conn) {
        Ok(login_infos) if login_infos.len() == 1 => {
            let login_info = &login_infos[0];
            let old_password_signed = signed_password(old_password, &login_info.username);
            if old_password_signed == login_info.password {
                let signed_passwd = signed_password(new_password, &login_info.username);
                match diesel::update(target)
                    .set(password.eq(signed_passwd))
                    .execute(conn)
                {
                    Ok(_) => Ok(true),
                    Err(err) => Err(err.to_string()),
                }
            } else {
                Err("修改密码失败".to_string())
            }
        }
        _ => Err("修改密码失败".to_string()),
    }
}

pub fn remove_login_info(conn: &mut DbConnection, login_info_id: i64) -> Result<usize, Error> {
    use self::tb_login_info::dsl::*;
    let target = tb_login_info.filter(id.eq(login_info_id));
    diesel::delete(target).execute(conn)
}

pub fn valid_login_info(conn: &mut DbConnection, uname: &str, passwd: &str) -> bool {
    let signed_passwd = signed_password(passwd, uname);
    info!("signed_passwd:{}, username: {}", &signed_passwd, uname);
    use self::tb_login_info::dsl::*;
    use crate::models::usermod::LoginInfoModel;
    match tb_login_info
        .filter(username.eq(uname))
        .filter(password.eq(signed_passwd))
        .load::<LoginInfoModel>(conn)
    {
        Ok(login_infos) if login_infos.len() == 1 => true,
        Ok(login_infos) if login_infos.len() > 1 => {
            warn!("duplicat username {}", uname);
            false
        }
        _ => false,
    }
}

fn signed_password(passwd: &str, username: &str) -> String {
    sign_util::blake2_sign_with_salt(passwd, username)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_signed_password() {
        let username = "tianlang";
        let passwd = "KeepWriting";
        println!(
            "username:{},passwd:{}, new passwd: {}",
            username,
            passwd,
            signed_password(passwd, username)
        );
    }
}
