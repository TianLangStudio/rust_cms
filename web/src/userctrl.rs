use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web};
use diesel::r2d2::{self, ConnectionManager};

use common::db_util;
use log::{info, warn};

use super::web_util;
use common::result::AjaxResult;
use common::sign_util::blake2_sign_temp as blake2_sign;

use dao::{models::usermod::*, repos::userrepo};

pub type DbConnection = userrepo::DbConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[get("/admin/test")]
pub(crate) async fn admin_test(session: Session) -> impl Responder {
    let username = web_util::get_username_from_session(&session).unwrap();
    format!("Hello, {}", username)
}
#[post("/api/register")]
pub(crate) async fn register(
    pool: web::Data<Pool>,
    login_info: web::Json<LoginInfo>,
) -> impl Responder {
    let new_login_info = NewLoginInfo {
        username: &login_info.username,
        password: &login_info.password,
    };
    let mut conn = db_util::get_conn(&pool).unwrap();
    match userrepo::add_login_info(&mut conn, &new_login_info) {
        Ok(info) => HttpResponse::Ok().json(AjaxResult::success_with_single(info)),
        Err(err) => HttpResponse::Forbidden().json(AjaxResult::<String>::fail(err.to_string())),
    }
}

const SESSION_USER_KEY: &str = web_util::SESSION_USER_KEY;
const SESSION_USER_KEY_SIGN: &str = web_util::SESSION_USER_KEY_SIGN;

#[post("/api/login")]
pub(crate) async fn login(
    session: Session,
    pool: web::Data<Pool>,
    login_info: web::Json<LoginInfo>,
) -> impl Responder {
    match session.get::<String>(SESSION_USER_KEY) {
        Ok(Some(user_info)) if user_info == login_info.username => {
            info!("already logged in");
            let user_key_sign = blake2_sign(&user_info);
            match session.get::<String>(SESSION_USER_KEY_SIGN) {
                Ok(Some(user_key_sign_session)) if user_key_sign == user_key_sign_session => {
                    HttpResponse::Ok().json(AjaxResult::<bool>::success_without_data())
                }
                _ => {
                    warn!("illegal request username:{}", login_info.username);
                    session.remove(SESSION_USER_KEY_SIGN);
                    session.remove(SESSION_USER_KEY);
                    HttpResponse::Forbidden()
                        .json(AjaxResult::<bool>::fail("Login time expired".to_string()))
                }
            }
        }
        _ => {
            info!("{} login now", login_info.username);
            let mut conn = db_util::get_conn(&pool).unwrap();
            match userrepo::valid_login_info(&mut conn, &login_info.username, &login_info.password)
            {
                true => {
                    let user_key_sign = blake2_sign(&login_info.username);
                    session
                        .insert(SESSION_USER_KEY_SIGN, user_key_sign)
                        .unwrap();
                    session
                        .insert(SESSION_USER_KEY, login_info.username.clone())
                        .unwrap();
                    HttpResponse::Ok().json(AjaxResult::<bool>::success_without_data())
                }
                false => HttpResponse::Forbidden().json(AjaxResult::<bool>::fail(
                    "password does not match username".to_string(),
                )),
            }
        }
    }
}

#[get("/api/logout")]
pub(crate) async fn logout(session: Session) -> impl Responder {
    session.remove(SESSION_USER_KEY_SIGN);
    session.remove(SESSION_USER_KEY);
    web_util::ok_without_data()
}
