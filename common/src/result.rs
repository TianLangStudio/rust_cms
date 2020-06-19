#![allow(unused)]
use serde::Serialize;
use actix_web::{ Responder, HttpResponse};

#[derive(Serialize)]
pub struct AjaxResult<T> {
    msg: String,
    data: Option<Vec<T>>,
}
 
const MSG_SUCCESS: &str = "success";

impl<T> AjaxResult<T> {
 
    pub fn success(data_opt: Option<Vec<T>>) -> Self{
         Self {
             msg: MSG_SUCCESS.to_string(),
             data: data_opt
         }
    }
 
    pub fn success_without_data() -> Self {
        Self::success(Option::None)
    }
    pub fn success_with_single(single: T) -> Self{
        Self {
            msg:  MSG_SUCCESS.to_string(),
            data: Option::Some(vec![single])
        }
    }
    pub fn fail(msg: String) -> Self {
        Self {
            msg,
            data: None
        }
     }

     pub fn get_msg(&self)  -> &str {
               &self.msg
     }
     pub fn get_data(&self) -> &Option<Vec<T>> {
              &self.data
     }
}

pub fn ok_without_data() -> impl Responder {
    HttpResponse::Ok().json(AjaxResult::<bool>::success_without_data())
}

pub fn forbidden_with_errmsg(msg: String)  -> impl Responder { 

    HttpResponse::Forbidden().json(AjaxResult::<bool>::fail(msg))
    
}