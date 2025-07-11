use std::path::Path;

use actix_files as fs;
use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{Either, Error, HttpResponse, Result, error, get, post, web};
//use actix_http::{body::BoxBody, Response};
//use http::StatusCode;
use async_std::fs::File;
use diesel::r2d2::{self, ConnectionManager};

use futures::{AsyncWriteExt, StreamExt};

use log::{error, info};

use common::{config_util, db_util, result};
use dao::models::filemod::*;
use dao::repos::filerepo;

use super::web_util;

pub type DbConnection = filerepo::DbConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;
type UploadReslut = Either<HttpResponse, Result<HttpResponse, Error>>;

#[post("/api/file/admin/upload")]
pub(crate) async fn upload(
    mut multipart: Multipart,
    session: Session,
    pool: web::Data<Pool>,
) -> UploadReslut {
    let username = match web_util::get_username_from_session(&session) {
        Some(usernaem) => usernaem,
        None => return Either::Left(result::forbidden_with_errmsg(String::from("请先登录"))),
    };

    let mut file_ids: Vec<String> = Vec::new();
    while let Some(Ok(field)) = multipart.next().await {
        let path = &get_file_save_path();
        match save_file(field, path, &username, &pool).await {
            Ok(file_id) => file_ids.push(file_id),
            Err(err) => return Either::Right(Err(err)),
        }
    }
    if file_ids.is_empty() {
        return Either::Left(result::forbidden_with_errmsg(String::from(
            "请选择上传文件",
        )));
    }
    Either::Left(success_with_file_ids(file_ids))
}

#[get("/api/file/{id}")]
pub(crate) async fn view_file(path_params: web::Path<(String,)>) -> Result<fs::NamedFile> {
    let path_params = path_params.into_inner();
    let file_id = &path_params.0;
    let save_path = get_file_save_path();
    let path = Path::new(&save_path);
    //todo 判断是否是私有文件
    Ok(fs::NamedFile::open(path.join(file_id))?)
}
fn success_with_file_ids(file_ids: Vec<String>) -> HttpResponse {
    HttpResponse::Ok().json(result::AjaxResult::success(Some(file_ids)))
}
fn get_file_save_path() -> String {
    let path = config_util::get_app_config()
        .get_string("tl.app.upload.path")
        .unwrap_or_else(|_| String::from("upload"));
    match std::fs::create_dir_all(&path) {
        Ok(_) => info!(" app upload path:{}", &path),
        Err(_) => error!("error create app upload path: {}", &path),
    }
    path
}

fn get_file_max_size_bytes() -> usize {
    let max_size_mb = config_util::get_app_config()
        .get_float("tl.app.upload.max_size")
        .unwrap_or(1.0);
    (max_size_mb * 1024.0 * 1024.0) as usize
}

/*lazy_static! {
    //static ref FILE_SAVE_PATH: String = get_file_save_path();
    //static ref FILE_MAX_SIZE: usize = get_file_max_size_bytes();
}*/

//保存文件
async fn save_file(
    mut field: Field,
    path: &str,
    username: &str,
    pool: &Pool,
) -> Result<String, Error> {
    let path = Path::new(path);
    let file_id = db_util::uuid();
    let file_path = path.join(&file_id);
    let mut file = File::create(file_path).await?;
    let mut length = 0;
    let content_disposition = field.content_disposition();
    let upload_file_name = content_disposition
        .and_then(|d| d.get_filename())
        .unwrap_or_default();
    let upload_file_name = upload_file_name.to_string();
    let upload_file_ext = upload_file_name.split(".").last().unwrap_or("").to_string();

    while let Some(bytes) = field.next().await {
        let bytes = bytes?;
        length += bytes.len();
        if length > get_file_max_size_bytes() {
            error!("err:{}", "上传的文件过大");
            return Err(error::ErrorInternalServerError("上传的文件过大"));
            //return Err(Error::from(Response::new(StatusCode::INTERNAL_SERVER_ERROR).set_body(BoxBody::new("上传的文件过大"))));
        }
        file.write_all(&bytes).await?;
    }
    let new_file_mod = NewFileMod {
        id: &file_id,
        name: Some(&upload_file_name),
        ext: Some(&upload_file_ext),
        is_private: 0,
        creater: username,
    };
    match db_util::get_conn(pool) {
        Some(mut conn) => match filerepo::add_file(&mut conn, &new_file_mod) {
            Ok(_) => (),
            Err(err) => {
                error!("err:{}", err);
                return Err(error::ErrorInternalServerError("文件存储失败"));
            }
        },
        None => {
            return Err(error::ErrorInternalServerError("文件存储失败"));
        }
    };

    Ok(file_id)
}
