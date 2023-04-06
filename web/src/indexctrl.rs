use super::web_util;
use actix_files as fs;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder, Result};
use tera::{self, Tera};

/// 网站favicon文件
#[get("/favicon.ico")]
pub(crate) async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/img/favicon.ico")?)
}

#[get("/")]
pub(crate) async fn index(session: Session, tmpl: web::Data<Tera>) -> impl Responder {
    let tmpl_name = web_util::get_tmpl_from_session(&session);
    let tmpl_name = tmpl_name + "/index.html";

    let username = match web_util::get_username_from_session(&session) {
        Some(username) => username,
        None => String::from(""),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("username", &username);
    let body = tmpl.render(&tmpl_name, &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(body)
}
