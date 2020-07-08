use actix_web::{post, get, web, Responder, HttpResponse, Error, error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_session::Session;
use tera::{Tera, self};

use log::{info, warn, error};

use common::{result, db_util};

use dao::{models::articlemod::*, repos::articlerepo};
use super::web_util;
pub type DbConnection = articlerepo::DbConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[post("/api/article/admin/add")]
async fn admin_add_article(
    pool: web::Data<Pool>,
    session: Session, 
    article : web::Json<NewArticle>)  -> impl Responder {

        let username = match web_util::get_username_from_session(&session) {
            Some(username) => username,
            None => return result::forbidden_with_errmsg(String::from("请先登录"))
        };
        
        match articlerepo::add_article(&pool.get().unwrap(),   &article, &username) {
            Ok(_) =>  HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data()),
            Err(err) => {
                error!("add article error:{}", err);
                HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))}
        }
}

#[post("/api/article/admin/edit")]
async fn admin_edit_article( pool: web::Data<Pool>,   edit_article: web::Json<EditArticle> ) -> impl Responder {
        match articlerepo::edit_article_info(&pool.get().unwrap(),  &edit_article) {
            Ok(_) =>  HttpResponse::Ok().json(result::AjaxResult::<bool>::success_without_data()),
            Err(err) => HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))
        }
    }
#[get("/api/article/list/{page_no}/{page_size}")]
async fn list_article(
    pool: web::Data<Pool>,
    page: web::Path<(i64, i64)>
) -> Result<HttpResponse, Error> {
     let conn = match db_util::get_conn(&pool) {
         Some(conn) => conn,
         None => return Ok(result::internal_server_error(String::from("服务器繁忙请稍后再试")))
     };

    match articlerepo::list_new_article(&conn,  page.0, page.1) {
        Ok(articles) => Ok(HttpResponse::Ok().json(result::AjaxResult::success(Some(articles)))),
        Err(err) => Err(
            error::ErrorInternalServerError(err)
        )
    }
}
#[get("/article/admin/edit/{article_id}")]
async fn admin_edit_view(
    path_params: web::Path<(String,)>,
    session: Session, 
    pool: web::Data<Pool>,
    tmpl: web::Data<Tera>
) -> impl Responder{
     let tmpl_name = web_util::get_tmpl_from_session(&session) + "/admin/article/edit.html";
     let mut ctx = tera::Context::new();
     let article_id = &path_params.0 ;
     let is_edit = "add"  != article_id;
     ctx.insert("is_edit", &is_edit);

     if  is_edit  {//修改
        //检查当前用户是否是文章的所属者
        let username = web_util::get_username_from_session(&session).unwrap();
        match db_util::get_conn(&pool) {
            Some(conn) => {
                match articlerepo::find_article_by_id(&conn,  article_id) {
                    Ok(article) if article.creater == username  => {
                        ctx.insert("article",  &article);
                        match articlerepo::find_article_content_by_id(&conn,  article_id)  {
                            Ok(article_content) =>  ctx.insert("article_content",  &article_content.get_content()),
                            _ => ()
                        };
                       
                    },
                    _  =>   return HttpResponse::NotFound().content_type("text/html").body("文章不存在")
                };
              
            },
            None => return result::internal_server_error(String::from("服务器繁忙请稍后再试"))
        };
         
     };
     
     let body = tmpl.render(&tmpl_name,  &ctx).unwrap();
     HttpResponse::Ok().content_type("text/html").body(body)
}