use actix_session::Session;
use actix_web::{error, get, post, web, Error, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use tera::{self, Tera};

use log::{error, info, warn};

use common::{db_util, result};

use super::web_util;
use dao::{models::articlemod::*, repos::articlerepo};
pub type DbConnection = articlerepo::DbConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[derive(Deserialize, Serialize)]
struct PublishParams {
    article_id:  String,
    content_id: String,
}

#[post("/api/article/admin/save")]
async fn admin_save_article(
    pool: web::Data<Pool>,
    session: Session,
    edit_article: web::Json<EditArticle>,
) -> impl Responder {
    let conn = match db_util::get_conn(&pool) {
        Some(conn) => conn,
        None => return result::server_busy_error(),
    };
    let username = web_util::get_username_from_session(&session).unwrap();
    if edit_article.id.as_ref().is_none() || edit_article.id.as_ref().unwrap().len() < 2 {
        match articlerepo::add_article(&pool.get().unwrap(), edit_article.0, &username) {
            Ok(article_id) => {
                let ids = PublishParams {
                    article_id: article_id.clone(),
                    content_id: article_id.clone(),
                };
                HttpResponse::Ok().json(result::AjaxResult::<PublishParams>::success_with_single(
                    ids,
                ))
            }
            Err(err) => {
                error!("add article error:{}", err);
                HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))
            }
        }
    } else {
        let   article_id = edit_article.id.as_ref().unwrap();
        match articlerepo::edit_article_info(&conn, &edit_article) {
            Ok(_) => {
                if let Some(content) = &edit_article.content {
                    let content_id = db_util::uuid();
                    let new_article_content = NewArticleContentModel ::new(
                        &content_id,
                        &edit_article.id.as_ref().unwrap(),
                        &content
                    );
                    match articlerepo::save_article_content(&conn, &new_article_content) {
                        Ok(_) => {

                            let _ = articlerepo::remove_article_content(
                                &conn,
                                3,
                                &edit_article.id.as_ref().unwrap(),
                            );
                            let ids = PublishParams {
                                article_id:  article_id.clone(),
                                content_id: content_id.clone(),
                            };
                            return    HttpResponse::Ok().json(result::AjaxResult::<PublishParams>::success_with_single(
                                ids,
                            ));
                        }
                        Err(err) => return result::forbidden_with_errmsg(err.to_string()),
                    };
                }
                let ids = PublishParams {
                    article_id:  article_id.clone(),
                    content_id: "".to_string()
                };
                HttpResponse::Ok().json(result::AjaxResult::<PublishParams>::success_with_single(
                    ids,
                ))
            }
            Err(err) => {
                HttpResponse::Forbidden().json(result::AjaxResult::<bool>::fail(err.to_string()))
            }
        }
    }
}
#[get("/api/article/list/{page_no}/{page_size}")]
async fn list_article(
    pool: web::Data<Pool>,
    page: web::Path<(i64, i64)>,
) -> Result<HttpResponse, Error> {
    let conn = match db_util::get_conn(&pool) {
        Some(conn) => conn,
        None => return Ok(result::server_busy_error()),
    };

    match articlerepo::list_new_article(&conn, page.0, page.1) {
        Ok(articles) => Ok(HttpResponse::Ok().json(result::AjaxResult::success(Some(articles)))),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}

#[post("/api/article/admin/publish")]
async fn admin_publish_article(
    pool: web::Data<Pool>,
    params: web::Json<PublishParams>,
) -> Result<HttpResponse, Error> {
    let article_id = &params.article_id;
    let content_id = &params.content_id;
    let conn = db_util::get_conn(&pool).unwrap();
    match articlerepo::publish_article(&article_id, content_id, &conn) {
        Ok(_) => Ok(result::ok_without_data()),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}

#[get("/view/articles")]
async fn view_articles(
    query_param: web::Query<web_util::Page>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let query_param = query_param.0;
    let page_no = query_param.page_no.unwrap_or(1);
    let page_size = query_param.page_size.unwrap_or(7);

    info!("page_no:{}, page_size:{}", page_no, page_size);
    let mut render_context = web_util::new_render_context(&session);
    render_context.insert("page_no", &page_no);
    render_context.insert("page_size", &page_size);

    let username = web_util::get_username_from_session(&session).unwrap_or(String::from(""));
    render_context.insert("username", &username);

    Ok(web_util::render_html(
        &session,
        &render_context,
        &tmpl,
        "articles",
    ))
}

#[get("/view/article/{article_id}")]
async fn view_article_by_id(
    path_params: web::Path<(String,)>,
    session: Session,
    pool: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let mut render_context = tera::Context::new();
    let conn = match db_util::get_conn(&pool) {
        Some(conn) => conn,
        _ => {
            return Ok(result::internal_server_error(String::from(
                "服务器繁忙请稍后再试",
            )))
        }
    };

    let article_id = &path_params.0;

    let article_info = articlerepo::find_article_by_id(&conn, &article_id);

    let article_content = articlerepo::find_article_content_by_id(&conn, &article_id);

    let article_content = match article_content {
        Ok(article_content) => article_content,
        _ => {
            return Ok(result::internal_server_error(String::from(
                "服务器繁忙请稍后再试",
            )))
        }
    };
    let article_info = match article_info {
        Ok(article_info) => article_info,
        _ => {
            return Ok(result::internal_server_error(String::from(
                "服务器繁忙请稍后再试",
            )))
        }
    };

    render_context.insert("article_info", &article_info);
    render_context.insert("article_content", &article_content);
    let username = web_util::get_username_from_session(&session).unwrap_or(String::from(""));
    render_context.insert("username", &username);
    let tmpl_name = web_util::get_tmpl_from_session(&session) + "/article.html";
    let body = tmpl.render(&tmpl_name, &render_context).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/article/admin/edit/{article_id}")]
async fn admin_edit_view(
    path_params: web::Path<(String,)>,
    session: Session,
    pool: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> impl Responder {
    let tmpl_name = web_util::get_tmpl_from_session(&session) + "/admin/article/edit.html";
    let mut ctx = tera::Context::new();
    let article_id = &path_params.0;
    let is_edit = "add" != article_id;
    ctx.insert("is_edit", &is_edit);

    if is_edit {
        //修改
        //检查当前用户是否是文章的所属者
        let username = web_util::get_username_from_session(&session).unwrap();
        match db_util::get_conn(&pool) {
            Some(conn) => {
                match articlerepo::find_article_by_id(&conn, article_id) {
                    Ok(article) if article.creater == username => {
                        ctx.insert("article", &article);
                        match articlerepo::find_article_content_by_id(&conn, article_id) {
                            Ok(article_content) => {
                                ctx.insert("article_content", &article_content.get_content())
                            }
                            _ => (),
                        };
                    }
                    _ => {
                        return HttpResponse::NotFound()
                            .content_type("text/html")
                            .body("文章不存在")
                    }
                };
            }
            None => return result::internal_server_error(String::from("服务器繁忙请稍后再试")),
        };
    };

    let body = tmpl.render(&tmpl_name, &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}
