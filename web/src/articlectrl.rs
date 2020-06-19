use actix_web::{post, get, web, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_session::Session;
use log::{info, warn};

use common::result;
use dao::{models::articlemod::*, repos::articlerepo};

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

#[post("/api/article/admin/add")]
async fn admin_edit_article( pool: web::Data<Pool>,   edit_article: web::Json<EditArticle> ) -> impl Responder {
        match articlerepo::edit_article_info(&pool.get().unwrap(),  &edit_article) {
            Ok(_) =>  HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data()),
            Err(err) => HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))
        }
    }