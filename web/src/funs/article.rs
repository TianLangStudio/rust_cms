//use std::collections::HashMap;
use tera::{Result, Value, from_value, to_value};

use dao::repos::articlerepo;

use common::db_util;
use dao::models::articlemod::ARTICLE_STATUS_PUBLISHED;

///创建获取最新文章列表的函数
pub fn make_list_new_articles(pool: db_util::Pool) -> super::GlobalFn {
    make_list_by_page(pool, |conn, page_no, page_size, status: i32| {
        articlerepo::list_new_article_info(conn, page_no, page_size, status)
    })
}

pub fn make_list_recommend_articles(pool: db_util::Pool) -> super::GlobalFn {
    make_list_by_page(pool, |conn, page_no, page_size, _status| {
        articlerepo::list_recommend_article_info(conn, page_no, page_size)
    })
}

fn make_list_by_page<T>(pool: db_util::Pool, list_by_page: T) -> super::GlobalFn
where
    T: Fn(&mut articlerepo::DbConnection, i64, i64, i32) -> articlerepo::ListArticleResult
        + Send
        + Sync
        + 'static,
{
    Box::new(move |args| -> Result<Value> {
        let default_page_no = to_value(1).unwrap();
        let default_page_size = to_value(10).unwrap();
        let default_status = to_value(ARTICLE_STATUS_PUBLISHED).unwrap();

        let page_no = args.get("page_no").unwrap_or(&default_page_no);
        let page_size = args.get("page_size").unwrap_or(&default_page_size);
        let status = args.get("status").unwrap_or(&default_status);

        let page_no: i64 = from_value(page_no.clone()).unwrap();
        let page_size: i64 = from_value(page_size.clone()).unwrap();
        let status: i32 = from_value(status.clone()).unwrap();

        let mut conn = match db_util::get_conn(&pool) {
            Some(conn) => conn,
            None => return Err("oops".into()),
        };
        match list_by_page(&mut conn, page_no, page_size, status) {
            Ok(articles) => Ok(to_value(articles).unwrap()),
            _ => Err("oops".into()),
        }
    })
}
