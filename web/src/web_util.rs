use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use common::config_util;
use common::result;
use common::sign_util::*;

pub const SESSION_USER_KEY: &str = "user_info";
pub const SESSION_USER_KEY_SIGN: &str = "user_info_sign";

pub fn get_username_from_session(session: &Session) -> Option<String>{
    let username =  match session.get::<String>(SESSION_USER_KEY) {
        Ok(uname) => uname?,
        _ =>  return None
    };
    let user_key_sign = blake2_sign(&username);
    match session.get::<String>(SESSION_USER_KEY_SIGN) {
        Ok(Some(user_key_sign_session))  if user_key_sign == user_key_sign_session => {
                Some(username)
        }
        _ => {
            None
        }
    }
}

pub fn get_tmpl_from_session(_session: &Session) -> String  {
       config_util::APP_CONFIG.get_str("tl.app.template.name").expect("default template name is required")
}

pub fn ok_without_data() -> impl Responder {
    HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data())
}