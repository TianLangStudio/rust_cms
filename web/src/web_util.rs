use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use common::config_util;
use common::{db_util, result};
use common::sign_util::*;

pub const SESSION_USER_KEY: &str = "user_info";
pub const SESSION_USER_KEY_SIGN: &str = "user_info_sign";

pub fn get_username_from_session(session: &Session) -> Option<String> {
    let username = match session.get::<String>(SESSION_USER_KEY) {
        Ok(uname) => uname?,
        _ => return None,
    };
    let user_key_sign = blake2_sign_temp(&username);
    match session.get::<String>(SESSION_USER_KEY_SIGN) {
        Ok(Some(user_key_sign_session)) if user_key_sign == user_key_sign_session => Some(username),
        _ => None,
    }
}

pub fn get_username(session: &Session) -> String {
    match get_username_from_session(session) {
        Some(username) => username,
        None => String::from(""),
    }
}

pub fn new_render_context(session: &Session) -> tera::Context {
    let mut render_context = tera::Context::new();
    render_context.insert("username", &get_username(session));
    render_context
}

pub fn render_html(
    session: &Session,
    render_context: &tera::Context,
    tmpl: &tera::Tera,
    name: &str,
) -> HttpResponse {
    let tmpl_name = get_tmpl_from_session(&session) + "/" + name + ".html";
    let body = tmpl.render(&tmpl_name, render_context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}
pub fn get_tmpl_from_session(_session: &Session) -> String {
    config_util::APP_CONFIG
        .get_string("tl.app.template.name")
        .expect("default template name is required")
}

pub fn ok_without_data() -> impl Responder {
    HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data())
}

pub fn get_conn_or_busy_error(pool: &db_util::Pool) -> Result<db_util::PooledConnection, actix_web::Error> {
    db_util::get_conn(pool).ok_or(actix_web::error::ErrorInternalServerError("Server is busy"))
}
#[derive(Serialize, Deserialize)]
pub struct Page {
    pub page_no: Option<i64>,
    pub page_size: Option<i64>,
}
