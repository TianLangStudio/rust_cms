
use std::path::Path;

use actix_web::{post,  Responder, web, HttpResponse,  error, Error};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_session::Session;
use async_std::fs::File;
use lazy_static::lazy_static;
use futures::{AsyncWriteExt, StreamExt};

use log::{error};

use common::{config_util, db_util, result};
use super::web_util;


#[post("/api/file/admin/upload")]
async fn upload(
    mut multipart: Multipart,
    session: Session
) -> impl Responder {
    let mut file_ids :Vec<String> = Vec::new();
    while let Some(Ok(field)) = multipart.next().await {
        let path = get_file_save_path();
        if let Ok(file_id)  = save_file(field,  &path).await {
             file_ids.push(file_id)
        } 
    }
   HttpResponse::Ok().json(
       result::AjaxResult::success(Some(file_ids))
    )
}

fn get_file_save_path()  -> String  {
    let path = match config_util::APP_CONFIG.get_str("tl.app.upload.path")  {
          Ok(path) => path,
          Err(_) => String::from("upload")
    };
    std::fs::create_dir_all(&path);
    path
}

fn get_file_max_size_bytes() -> usize {
    let max_size_mb = match config_util::APP_CONFIG.get_float("tl.app.upload.max_size")  {
        Ok(size_mb) => size_mb,
        Err(_) => 1.0
    };
    (max_size_mb * 1024.0 * 1024.0) as usize
}

//保存文件
async fn save_file(mut field: Field, path: &str) -> Result<String,  Error> {
    let path = Path::new(path);
    let file_id = db_util::uuid();
    let file_path = path.join(&file_id);
    let mut file = File::create(file_path).await?;
    let mut length = 0;
    
    while let Some(bytes) = field.next().await {
        let bytes = bytes?;
        length += bytes.len();
        if length > get_file_max_size_bytes() {
            return Err(Error::from(
                HttpResponse::PayloadTooLarge().body("上传的文件过大")
            ));
        }
        file.write_all(&bytes).await?;
    }
    Ok(file_id)
}

