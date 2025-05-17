use std::time::SystemTime;

use crate::models::articlemod::*;
use crate::repos;
use crate::schema::tb_article_content::article_id;
use crate::schema::{tb_article, tb_article_content};
use common::db_util;
use diesel::prelude::*;
use diesel::result::Error;
use log::{info, warn};

pub type DbConnection = db_util::DbConnection;
pub type ListArticleResult = Result<Vec<ArticleModel>, Error>;

pub fn add_article(
    conn: &mut DbConnection,
    new_article: EditArticle,
    username: &str,
) -> Result<String, Error> {
    conn.transaction::<_, Error, _>(|conn| {
        let id: String = db_util::uuid();
        let content = &new_article.content;
        let subtitle = match &new_article.subtitle {
            Some(subtitle) => subtitle,
            None => "",
        };
        let new_article_model = ArticleModel {
            id: id.clone(),
            title: new_article.title,
            subtitle: new_article.subtitle,
            intro: new_article.intro,
            rcmd_weight: new_article.rcmd_weight,
            url: new_article.url,
            status: Some(ARTICLE_STATUS_NEW),
            creater: String::from(username),
            create_at: chrono::Utc::now().naive_local(),
            update_at: chrono::Utc::now().naive_local(),
        };
        diesel::insert_into(tb_article::table)
            .values(&new_article_model)
            .execute(conn);
        if content.is_some() {
            let new_article_content = NewArticleContentModel {
                id: &new_article_model.id,
                status: ARTICLE_STATUS_NEW,
                article_id: &new_article_model.id,
                content: content.as_ref().unwrap(),
                create_at: Some(chrono::Utc::now().naive_local()),
            };
            save_article_content(conn, &new_article_content)?;
        }
        Ok(id)
    })
}
//return (article_id, content_id)
pub fn edit_article(
    conn: &mut DbConnection,
    edit_article: EditArticle,
) -> Result<(String, Option<String>), Error> {
    conn.transaction(|conn| {
        let mut content_id_opt: Option<String> = None;
        if let Some(content) = &edit_article.content {
            let content_id = db_util::uuid();
            let new_article_content = NewArticleContentModel::new(
                &content_id,
                edit_article.id.as_ref().unwrap(),
                content,
            );
            save_article_content(conn, &new_article_content);
            remove_article_content(conn, 6, edit_article.id.as_ref().unwrap());
            content_id_opt = Some(content_id);
        }
        edit_article_info(conn, &edit_article)?;
        Ok((edit_article.id.unwrap(), content_id_opt))
    })
}
pub fn edit_article_info(
    conn: &mut DbConnection,
    edit_article: &EditArticle,
) -> Result<usize, Error> {
    let id = edit_article.id.as_ref().unwrap();
    let edit_article_model = EditArticleModel {
        id: id.clone(),
        title: edit_article.title.clone(),
        subtitle: edit_article.subtitle.clone(),
        intro: edit_article.intro.clone(),
        rcmd_weight: edit_article.rcmd_weight,
        url: edit_article.url.clone(),
        //status: edit_article.status.clone(),
        status: Some(ARTICLE_STATUS_NEW), //we should set article's status as new while editing
        update_at: chrono::Utc::now().naive_local(),
    };

    diesel::update(tb_article::table)
        .filter(tb_article::dsl::id.eq(id))
        .set(edit_article_model)
        .execute(conn)
}

pub fn publish_article(
    atl_id: &str,
    content_id: &str,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    conn.transaction(|conn| {
        publish_article_content(content_id, content_id, conn)?;
        publish_article_info(atl_id, conn)
    })
}

pub fn submit_review(
    atl_id: &str,
    content_id: &str,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    conn.transaction(|conn| {
        change_article_info_status(atl_id, ARTICLE_STATUS_UNDER_REVIEW, conn)?;
        change_article_content_status(content_id, ARTICLE_STATUS_UNDER_REVIEW, conn)
    })
}

pub fn reject_review(
    atl_id: &str,
    content_id: &str,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    conn.transaction(|conn| {
        change_article_info_status(atl_id, ARTICLE_STATUS_NEW, conn)?;
        change_article_content_status(content_id, ARTICLE_STATUS_NEW, conn)
    })
}

fn publish_article_content(
    atl_id: &str,
    content_id: &str,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    change_article_content_status(content_id, ARTICLE_STATUS_PUBLISHED, conn)
}
fn change_article_content_status(
    content_id: &str,
    status: i32,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    use self::tb_article_content::dsl;

    diesel::update(dsl::tb_article_content)
        .filter(dsl::id.eq(content_id))
        .set(dsl::status.eq(status))
        .execute(conn)
}
fn publish_article_info(atl_id: &str, conn: &mut DbConnection) -> Result<usize, Error> {
    change_article_info_status(atl_id, ARTICLE_STATUS_PUBLISHED, conn)
}

/**
change the status of article
***/
fn change_article_info_status(
    atl_id: &str,
    status: i32,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    use self::tb_article::dsl;
    diesel::update(dsl::tb_article)
        .filter(dsl::id.eq(atl_id))
        .set(dsl::status.eq(status))
        .execute(conn)
}
/**
 * 文章内容更新并不更新原来的记录而是新增记录，这样后期可以支持回滚，多版本
 * **/
pub fn save_article_content(
    conn: &mut DbConnection,
    new_article_content: &NewArticleContentModel<'_>,
) -> Result<usize, Error> {
    diesel::insert_into(tb_article_content::table)
        .values(new_article_content)
        .execute(conn)
}

/**
 *
 * 保留最近的saved条记录，其它 的删除
 * **/
pub fn remove_article_content(
    conn: &mut DbConnection,
    saved: i64,
    article_id_find: &str,
) -> Result<usize, Error> {
    use self::tb_article_content::dsl;
    let mut filter = dsl::tb_article_content
        .filter(dsl::article_id.eq(article_id_find))
        .order(dsl::create_at.desc())
        .limit(10)
        .offset(saved)
        .load::<ArticleContentModel>(conn);

    match filter {
        Ok(article_contents) if !article_contents.is_empty() => {
            let removed: Vec<String> = article_contents
                .iter()
                .map(|article| article.id.clone())
                .collect();
            diesel::delete(dsl::tb_article_content.filter(dsl::id.eq_any(removed))).execute(conn)
        }
        _ => Ok(0),
    }
}
pub fn list_new_article_info(
    conn: &mut DbConnection,
    page_no: i64,
    page_size: i64,
    status: i32,
) -> ListArticleResult {
    use self::tb_article::dsl;
    let (limit, offset) = db_util::page2limit_offset(page_no, page_size);
    info!("limit:{}, offset:{}", &limit, &offset);

    dsl::tb_article
        .filter(dsl::status.eq(status))
        .order(dsl::update_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<ArticleModel>(conn)
}

pub fn list_recommend_article_info(
    conn: &mut DbConnection,
    page_no: i64,
    page_size: i64,
) -> ListArticleResult {
    use self::tb_article::dsl;
    let (limit, offset) = db_util::page2limit_offset(page_no, page_size);
    dsl::tb_article
        .filter(dsl::status.eq(ARTICLE_STATUS_PUBLISHED))
        .order(dsl::rcmd_weight.desc())
        .limit(limit)
        .offset(offset)
        .load::<ArticleModel>(conn)
}

pub fn find_article_by_id(conn: &mut DbConnection, id: &str) -> Result<ArticleModel, Error> {
    use self::tb_article::dsl;
    dsl::tb_article.find(id).first(conn)
}

pub fn find_article_content_by_id_and_status(
    conn: &mut DbConnection,
    find_article_id: &str,
    article_status: &Option<i32>,
) -> Result<ArticleContentModel, Error> {
    use self::tb_article_content::dsl;
    dsl::tb_article_content
        .filter(dsl::article_id.eq(find_article_id))
        .filter(dsl::status.eq(article_status.unwrap_or(ARTICLE_STATUS_PUBLISHED)))
        .order(dsl::create_at.desc())
        .first(conn)
}
