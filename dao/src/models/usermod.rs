use crate::schema::tb_login_info;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct LoginInfoModel {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = tb_login_info)]
pub struct NewLoginInfo<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct LoginInfo {
    pub id: Option<i64>,
    pub username: String,
    pub password: String,
}
