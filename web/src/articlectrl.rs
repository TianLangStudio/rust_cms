use actix_web::{post, get, web, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_session::Session;
use tera::{Tera, self};

use log::{info, warn};

use common::result;
use dao::{models::articlemod::*, repos::articlerepo};
use super::web_util;
pub type DbConnection = articlerepo::DbConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[post("/api/article/admin/add")]
async fn admin_add_article(
    pool: web::Data<Pool>,
    article : web::Json<NewArticle>)  -> impl Responder {
        match articlerepo::add_article(&pool.get().unwrap(),   &article) {
            Ok(_) =>  HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data()),
            Err(err) => HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))
        }
}

#[post("/api/article/admin/edit")]
async fn admin_edit_article( pool: web::Data<Pool>,   edit_article: web::Json<EditArticle> ) -> impl Responder {
        match articlerepo::edit_article_info(&pool.get().unwrap(),  &edit_article) {
            Ok(_) =>  HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data()),
            Err(err) => HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))
        }
    }
#[get("/article/admin/edit/{article_id}")]
async fn admin_edit_view(
    path_params: web::Path<(String,)>,
    session: Session, 
    tmpl: web::Data<Tera>
) -> impl Responder{
     let tmpl_name = web_util::get_tmpl_from_session(&session) + "/admin/article/edit.html";
     let mut ctx = tera::Context::new();
     let article_id = &path_params.0 ;
     let is_edit = "add"  != article_id;
     ctx.insert("is_edit", &is_edit);

     if  is_edit  {//修改
         ctx.insert("article_id",  article_id)
     };
     
     let body = tmpl.render(&tmpl_name,  &ctx).unwrap();
     HttpResponse::Ok().content_type("text/html").body(body)
}