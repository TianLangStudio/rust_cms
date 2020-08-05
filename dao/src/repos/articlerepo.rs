use std::time::SystemTime;

use crate::models::articlemod::*;
use crate::repos;
use crate::schema::{tb_article, tb_article_content};
use common::db_util;
use diesel::prelude::*;
use diesel::result::Error;
use log::{info, warn};

pub type DbConnection = db_util::DbConnection;
pub type ListAriticleResult = Result<Vec<ArticleModel>, Error>;

pub fn add_article(
    conn: &DbConnection,
    new_article: EditArticle,
    username: &str,
) -> Result<String, Error> {
    conn.transaction::<_, Error, _>(|| {
        let id: String = db_util::uuid();
        let content = &new_article.content;
        let subtitle = match &new_article.subtitle {
            Some(subtitle) => &subtitle,
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
                article_id: &new_article_model.id,
                content: &content.as_ref().unwrap(),
                create_at: Some(chrono::Utc::now().naive_local()),
            };
            save_article_content(conn, &new_article_content)?;
        }
        Ok(id)
    })
}

pub fn edit_article_info(conn: &DbConnection, edit_article: &EditArticle) -> Result<usize, Error> {
    let id = edit_article.id.as_ref().unwrap();
    let edit_article_model = EditArticleModel {
        id: id.clone(),
        title: edit_article.title.clone(),
        subtitle: edit_article.subtitle.clone(),
        intro: edit_article.intro.clone(),
        rcmd_weight: edit_article.rcmd_weight.clone(),
        url: edit_article.url.clone(),
        status: edit_article.status.clone(),
        update_at: chrono::Utc::now().naive_local(),
    };

    diesel::update(tb_article::table)
        .set(edit_article_model)
        .execute(conn)
}

pub fn publish_article(
    article_id: &str,
    content_id: &str,
    conn: &DbConnection,
) -> Result<usize, Error> {
    publish_article_content(article_id, content_id, conn)?;
    publish_article_info(article_id, conn)
}

fn publish_article_content(
    atl_id: &str,
    content_id: &str,
    conn: &DbConnection,
) -> Result<usize, Error> {
    use self::tb_article_content::dsl::*;
    diesel::update(tb_article_content)
        .filter(article_id.eq(atl_id))
        .set(status.eq(ARTICLE_STATUS_NEW))
        .execute(conn);
    diesel::update(tb_article_content)
        .filter(id.eq(content_id))
        .set(status.eq(ARTICLE_STATUS_PUBLISHED))
        .execute(conn)
}

fn publish_article_info(article_id: &str, conn: &DbConnection) -> Result<usize, Error> {
    use self::tb_article::dsl::*;
    diesel::update(tb_article)
        .filter(id.eq(article_id))
        .set(status.eq(ARTICLE_STATUS_PUBLISHED))
        .execute(conn)
}
/**
 * 文章内容更新并不更新原来的记录而是新增记录，这样后期可以支持回滚，多版本
 * **/
pub fn save_article_content(
    conn: &DbConnection,
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
    conn: &DbConnection,
    saved: i64,
    article_id_find: &str,
) -> Result<usize, Error> {
    use self::tb_article_content::dsl::*;
    let mut filter = tb_article_content
        .filter(article_id.eq(article_id_find))
        .order(create_at.desc())
        .limit(10)
        .offset(saved)
        .load::<ArticleContentModel>(conn);

    match filter {
        Ok(article_contents) if article_contents.len() > 0 => {
            let removed: Vec<String> = article_contents
                .iter()
                .map(|article| article.id.clone())
                .collect();
            diesel::delete(tb_article_content.filter(id.eq_any(removed))).execute(conn)
        }
        _ => Ok(0),
    }
}
pub fn list_new_article(conn: &DbConnection, page_no: i64, page_size: i64) -> ListAriticleResult {
    use self::tb_article::dsl::*;
    let (limit, offset) = db_util::page2limit_offset(page_no, page_size);
    info!("limit:{}, offset:{}", &limit, &offset);

    tb_article
        .filter(status.eq(ARTICLE_STATUS_PUBLISHED))
        .order(update_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<ArticleModel>(conn)
}

pub fn list_recommend_article(
    conn: &DbConnection,
    page_no: i64,
    page_size: i64,
) -> ListAriticleResult {
    use self::tb_article::dsl::*;
    let (limit, offset) = db_util::page2limit_offset(page_no, page_size);
    tb_article
        .filter(status.eq(ARTICLE_STATUS_PUBLISHED))
        .order(rcmd_weight.desc())
        .limit(limit)
        .offset(offset)
        .load::<ArticleModel>(conn)
}

pub fn find_article_by_id(conn: &DbConnection, id: &str) -> Result<ArticleModel, Error> {
    use self::tb_article::dsl::*;
    tb_article.find(id).order(create_at.desc()).first(conn)
}

pub fn find_article_content_by_id(
    conn: &DbConnection,
    find_article_id: &str,
) -> Result<ArticleContentModel, Error> {
    use self::tb_article_content::dsl::*;
    tb_article_content
        .filter(article_id.eq(find_article_id))
        .filter(status.eq(ARTICLE_STATUS_PUBLISHED))
        .order(create_at.desc())
        .first(conn)
}
