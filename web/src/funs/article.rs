//use std::collections::HashMap;
use tera::{Result, to_value, from_value,  Value};

use dao::repos::articlerepo;
use common::db_util;

pub fn make_list_new_articles(pool:  db_util::Pool) -> super::GlobalFn {
    Box::new(move |args| -> Result<Value> {
       let default_page_no = to_value(0).unwrap();
       let default_page_size = to_value(10).unwrap();

       let page_no   = args.get("page_no").unwrap_or( &default_page_no);
       let page_size = args.get("page_size").unwrap_or(&default_page_size);
       
       let page_no:i64  = from_value(page_no.clone()).unwrap();
       let page_size: i64 = from_value(page_size.clone()).unwrap();

       let  conn = match db_util::get_conn(&pool)  {
            Some(conn) => conn,
            None => return Err("oops".into())
       };
       match articlerepo::list_new_article(&conn,  page_no,  page_size) {
           Ok(articles) => Ok(
               to_value(articles).unwrap()
            ),
           _ => Err("oops".into())
       }
    })
}

