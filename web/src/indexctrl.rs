use super::web_util;
use actix_files as fs;
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, Result, get, web};
use common::result::internal_server_error;
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

    let username = web_util::get_username_from_session(&session).unwrap_or_default();

    let mut ctx = tera::Context::new();
    ctx.insert("username", &username);
    let body = tmpl.render(&tmpl_name, &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(body)
}
#[get("/health")]
pub(crate) async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
#[get("/sitemap.xml")]
pub(crate) async fn sitemap(request: HttpRequest, tmpl: web::Data<Tera>) -> impl Responder {
    let url = request.full_url().to_string();
    log::info!("url: {}", url);
    //let uri = request.uri().to_string();
    //log::info!("uri: {}", uri);
    let mut base_path = url.splitn(2, "/sitemap.xml").collect::<Vec<&str>>();
    let base_path = base_path.remove(0);
    let tmpl_name = "sitemap.xml";
    let mut ctx = tera::Context::new();
    ctx.insert("base_path", base_path);
    match tmpl.render(tmpl_name, &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/xml").body(body),
        Err(e) => internal_server_error(format!("{:?}", e)),
    }
}
