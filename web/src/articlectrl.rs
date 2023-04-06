use actix_session::Session;
use actix_web::{error, get, post, web, Error, Responder, HttpResponse};
use actix_web::web::Redirect;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use tera::{self, Tera};

use log::{error, info};

use common::{db_util, config_util, result};

use super::web_util;
use dao::{models::articlemod::*, repos::articlerepo};


pub type DbConnection = articlerepo::DbConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[derive(Deserialize, Serialize)]
pub struct PublishParams {
    article_id:  String,
    content_id: String,
}

#[post("/api/article/admin/save")]
pub(crate) async fn admin_save_article(
    pool: web::Data<Pool>,
    session: Session,
    edit_article: web::Json<EditArticle>,
) -> impl Responder {
    let mut conn = match db_util::get_conn(&pool) {
        Some(conn) => conn,
        None => return result::server_busy_error(),
    };
    let username = web_util::get_username_from_session(&session).unwrap();
    //add new article
    if edit_article.id.as_ref().is_none() || edit_article.id.as_ref().unwrap().len() < 2 {
        let add_article_result =
            articlerepo::add_article(&mut pool.get().unwrap(), edit_article.0, &username);

        match add_article_result {
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

    } else {//edit article
        match articlerepo::edit_article(&mut conn, edit_article.0) {
            Ok((article_id, content_id_opt)) => {
                let ids = PublishParams {
                    article_id,
                    content_id: content_id_opt.unwrap_or("".to_string())
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
    let mut conn = match db_util::get_conn(&pool) {
        Some(conn) => conn,
        None => return Ok(result::server_busy_error()),
    };
    let page = page.into_inner();
    match articlerepo::list_new_article_info(&mut conn, page.0, page.1, ARTICLE_STATUS_PUBLISHED) {
        Ok(articles) => Ok(HttpResponse::Ok().json(result::AjaxResult::success(Some(articles)))),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}

#[post("/api/article/admin/publish")]
pub(crate) async fn admin_publish_article(
    pool: web::Data<Pool>,
    params: web::Json<PublishParams>,
) -> Result<HttpResponse, Error> {
    let article_id = &params.article_id;
    let content_id = &params.content_id;
    let mut conn = web_util::get_conn_or_busy_error(&pool)?;

    //we should set the status of article as under review if need approval before publish
    if config_util::need_approval() {
       articlerepo::submit_review(article_id, content_id, &mut conn)
           .map_err(|e|error::ErrorInternalServerError(e))?;
    } else {
        articlerepo::publish_article(article_id, content_id, &mut conn)
            .map_err(|e|error::ErrorInternalServerError(e))?;
    }

    Ok(result::ok_without_data())
}

#[post("/api/article/admin/approve/{is_approved}")]
async fn admin_approve( pool: web::Data<Pool>,
                        params: web::Json<PublishParams>,
                        path_params: web::Path<(bool,)>,
                        session: Session,
) -> Result<HttpResponse, Error> {
    let username = web_util::get_username_from_session(&session)
        .ok_or(error::ErrorNetworkAuthenticationRequired("login first"))?;
    if !config_util::is_approver(&username) {
        return Err(error::ErrorPreconditionFailed("not approver"));
    };

    let article_id = &params.article_id;
    let content_id = &params.content_id;

    let (is_approve, ) = path_params.into_inner();
    let mut conn = web_util::get_conn_or_busy_error(&pool)?;
    if is_approve {
        articlerepo::publish_article(article_id, content_id, &mut conn)
            .map_err(|e|error::ErrorInternalServerError(e))?;
    }else {
        articlerepo::reject_review(article_id, content_id, &mut conn)
            .map_err(|e|error::ErrorInternalServerError(e))?;

    }
    Ok(result::ok_without_data())
}
fn view_articles_by_status(
    article_status: i32,
    query_param: web::Query<web_util::Page>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error>  {
    let query_param = query_param.0;
    let page_no = query_param.page_no.unwrap_or(1);
    let page_size = query_param.page_size.unwrap_or(7);
    info!("page_no:{}, page_size:{}", page_no, page_size);
    let mut render_context = web_util::new_render_context(&session);
    render_context.insert("page_no", &page_no);
    render_context.insert("page_size", &page_size);
    let username = web_util::get_username_from_session(&session).unwrap_or(String::from(""));
    render_context_insert(&mut render_context, &article_status, &username);
    Ok(web_util::render_html(
        &session,
        &render_context,
        &tmpl,
        "articles",
    ))
}
fn render_context_insert(render_context: &mut tera::Context, article_status: &i32, username: &str) {
    render_context.insert("username", username);
    let is_approver = config_util::is_approver(username);
    render_context.insert("isApprover", &is_approver);
    let is_under_review = article_status == &ARTICLE_STATUS_UNDER_REVIEW;
    render_context.insert("status", &article_status);
    render_context.insert("isUnderReview", &is_under_review);
    render_context.insert("ARTICLE_STATUS_UNDER_REVIEW", &ARTICLE_STATUS_UNDER_REVIEW);
    render_context.insert("ARTICLE_STATUS_PUBLISHED", &ARTICLE_STATUS_PUBLISHED);
    render_context.insert("ARTICLE_STATUS_NEW", &ARTICLE_STATUS_NEW);
}
#[get("/view/articles")]
pub(crate) async fn view_articles(
    query_param: web::Query<web_util::Page>,
    session: Session,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    view_articles_by_status(ARTICLE_STATUS_PUBLISHED, query_param, session, tmpl)
}
#[get("/view/article/{article_id}")]
pub(crate) async fn view_article_by_id(
    path_params: web::Path<(String,)>,
) -> impl Responder {
    let (article_id, ) = path_params.into_inner();
    let status = ARTICLE_STATUS_PUBLISHED;
    let url = format!("/view/article/{article_id}/{status}");
    Redirect::to(url).permanent()

}

#[get("/view/article/{article_id}/{status}")]
pub(crate) async fn view_article_by_id_and_status(
    path_params: web::Path<(String, Option<i32>)>,
    session: Session,
    pool: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let mut render_context = tera::Context::new();
    let mut conn = web_util::get_conn_or_busy_error(&pool)?;

    let path_params = path_params.into_inner();

    let article_id = &path_params.0;
    let article_status_opt = &path_params.1;
    let article_status = article_status_opt.unwrap_or(ARTICLE_STATUS_PUBLISHED);

    let article_info = articlerepo::find_article_by_id(&mut conn, &article_id);
    let article_info = match article_info {
        Ok(article_info) => article_info,
        _ => {
            return Ok(result::server_busy_error())
        }
    };

    let article_content = articlerepo::find_article_content_by_id_and_status(&mut conn,
                                                                             &article_id,
                                                                             &article_status_opt);
    let article_content = match article_content {
        Ok(article_content) => article_content,
        _ => {
            return Ok(result::internal_server_error(String::from(
                "服务器繁忙请稍后再试",
            )))
        }
    };


    render_context.insert("article_info", &article_info);
    render_context.insert("article_content", &article_content);
    let username = web_util::get_username_from_session(&session).unwrap_or(String::from(""));

    render_context_insert(&mut render_context, &article_status, &username);
    let tmpl_name = web_util::get_tmpl_from_session(&session) + "/article.html";
    let body = tmpl.render(&tmpl_name, &render_context).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/article/admin/edit/{article_id}")]
pub(crate) async fn admin_edit_view(
    path_params: web::Path<(String,)>,
    session: Session,
    pool: web::Data<Pool>,
    tmpl: web::Data<Tera>,
) -> Result<impl Responder, Error>{
    let path_params = path_params.into_inner();
    let tmpl_name = web_util::get_tmpl_from_session(&session) + "/admin/article/edit.html";
    let mut ctx = tera::Context::new();
    let article_id = &path_params.0;
    let is_edit = "add" != article_id;
    ctx.insert("is_edit", &is_edit);

    if is_edit {
        //修改
        //检查当前用户是否是文章的所属者
        let username = web_util::get_username_from_session(&session).unwrap();
        let mut conn = web_util::get_conn_or_busy_error(&pool)?;
        match articlerepo::find_article_by_id(&mut conn, article_id) {
            Ok(article) if article.creater == username => {
                ctx.insert("article", &article);
                let article_status = Some(ARTICLE_STATUS_PUBLISHED);
                match articlerepo::find_article_content_by_id_and_status(&mut conn, article_id, &article_status) {
                    Ok(article_content) => {
                        ctx.insert("article_content", &article_content.get_content())
                    }
                    _ => (),
                };
            }
            _ => {
                return Ok(HttpResponse::NotFound()
                    .content_type("text/html")
                    .body("文章不存在"))
            }
        };
    };

    let body = tmpl.render(&tmpl_name, &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/article/admin/under_review")]
async fn admin_under_review( query_param: web::Query<web_util::Page>,
                             session: Session,
                             tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    view_articles_by_status(ARTICLE_STATUS_UNDER_REVIEW, query_param, session, tmpl)
}


