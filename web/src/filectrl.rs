
use std::path::Path;

use actix_web::{post, get, Either, Responder, web, HttpResponse,  error, Error, Result};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_session::Session;
use actix_files as fs;
use async_std::fs::File;
use diesel::r2d2::{self, ConnectionManager};

use lazy_static::lazy_static;
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
async fn upload(
    mut multipart: Multipart,
    session: Session,
    pool: web::Data<Pool>,
) ->  UploadReslut {
    let username = match web_util::get_username_from_session(&session) {
        Some(usernaem) => usernaem,
        None =>   return Either::A(
            result::forbidden_with_errmsg(String::from("请先登录"))
        )
    };

    let mut file_ids :Vec<String> = Vec::new();
    while let Some(Ok(field)) = multipart.next().await {
        let path = &*FILE_SAVE_PATH;
        match save_file(field,  path, &username, &pool).await {
            Ok(file_id) => {
                file_ids.push(file_id)
            },
            Err(err) => return Either::B(Err(err))
        }  
    }
    if file_ids.len() == 0  {
        return Either::A(
            result::forbidden_with_errmsg(String::from("请选择上传文件"))
        )
    }
    Either::A(success_with_file_ids(file_ids))
}

#[get("/api/file/{id}")]
async fn view_file(path_params: web::Path<(String,)>) -> Result<fs::NamedFile>{
    let file_id = &path_params.0;
    let path = Path::new(&*FILE_SAVE_PATH);
    //todo 判断是否是私有文件
    Ok(fs::NamedFile::open(path.join(file_id))?)

}
fn success_with_file_ids(file_ids: Vec<String>)  -> HttpResponse{
    HttpResponse::Ok().json(
        result::AjaxResult::success(Some(file_ids))
     ) 
}
fn get_file_save_path()  -> String  {
    let path = match config_util::APP_CONFIG.get_str("tl.app.upload.path")  {
          Ok(path) => path,
          Err(_) => String::from("upload")
    };
    match std::fs::create_dir_all(&path) {
        Ok(_) => info!(" app upload path:{}",  &path),
        Err(e) => error!("error create app uplod path: {}", &path)
    }
    path
}

fn get_file_max_size_bytes() -> usize {
    let max_size_mb = match config_util::APP_CONFIG.get_float("tl.app.upload.max_size")  {
        Ok(size_mb) => size_mb,
        Err(_) => 1.0
    };
    (max_size_mb * 1024.0 * 1024.0) as usize
}

lazy_static! {
    static ref FILE_SAVE_PATH: String = get_file_save_path();
    static ref FILE_MAX_SIZE: usize = get_file_max_size_bytes();
}

//保存文件
async fn save_file(mut field: Field, path: &str, username: &str,  pool: &Pool) -> Result<String,  Error> {
    let path = Path::new(path);
    let file_id = db_util::uuid();
    let file_path = path.join(&file_id);
    let mut file = File::create(file_path).await?;
    let mut length = 0;
    let content_disposition = field.content_disposition();
    let mut upload_file_name = String::from("");
    let mut upload_file_ext =  String::from("");

    if let Some(cd) = content_disposition  {
        upload_file_name = cd.get_filename().unwrap_or("").to_string();
        upload_file_ext = upload_file_name.split(".").last().unwrap_or("").to_string();
    }
    
    while let Some(bytes) = field.next().await {
        let bytes = bytes?;
        length += bytes.len();
        if length > *FILE_MAX_SIZE {
            return Err(Error::from(
                HttpResponse::PayloadTooLarge().body("上传的文件过大")
            ));
        }
        file.write_all(&bytes).await?;
    }
    let new_file_mod = NewFileMod {
        id: &file_id,
        name:  Some(&upload_file_name),
        ext: Some(&upload_file_ext),
        is_private: 0,
        creater: username
    };
    match db_util::get_conn(pool) {
        Some(conn) => match filerepo::add_file(&conn, &new_file_mod) {
               Ok(_) => (),
               Err(err) => {
                   error!("err:{}", err);
                   return Err(Error::from(
                    result::internal_server_error(String::from("文件存储失败"))
                    ))
               }
        },
        None =>   return Err(Error::from(
            result::internal_server_error(String::from("文件存储失败"))
        ))
    };
    
    Ok(file_id)
}

