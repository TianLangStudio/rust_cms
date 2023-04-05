use std::time::SystemTime;

use crate::models::articlemod::*;
use crate::repos;
use crate::schema::{tb_article, tb_article_content, tb_draft_article};
use common::{config_util, db_util};
use diesel::prelude::*;
use diesel::result::Error;
use log::{info, warn};
//use crate::schema::tb_article::dsl::tb_article;


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
            Some(subtitle) => &subtitle,
            None => "",
        };

        let draft_article_model = ArticleDraftModel {
            id: id.clone(),
            title: new_article.title,
            subtitle: new_article.subtitle,
            intro: new_article.intro,
            rcmd_weight: new_article.rcmd_weight,
            url: new_article.url,
            status: Some(ARTICLE_STATUS_NEW),
            approver: None,
            creater: String::from(username),
            create_at: chrono::Utc::now().naive_local(),
            update_at: chrono::Utc::now().naive_local(),
        };
        match config_util::need_approval() {
            false => {
                let publish_article: ArticleModel= draft_article_model.into();
                diesel::insert_into(tb_article::table)
                    .values(&publish_article)
                    .execute(conn);

            }, //does not need approval insert into tb_article
            true => {
                diesel::insert_into(tb_draft_article::table)
                    .values(&draft_article_model)
                    .execute(conn);
            },//need approval insert into tb_draft_article first
        };
        if content.is_some() {
            let new_article_content = NewArticleContentModel {
                id: id.as_str(),
                article_id: id.as_str(),
                content: content.as_ref().unwrap(),
                create_at: Some(chrono::Utc::now().naive_local()),
            };
            save_article_content(conn, &new_article_content)?;
        }
        Ok(id)
    })
}

pub fn edit_article_info(conn: &mut DbConnection, edit_article: &EditArticle) -> Result<usize, Error> {
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
        .filter(tb_article::dsl::id.eq(id))
        .set(edit_article_model)
        .execute(conn)
}

pub fn publish_article(
    article_id: &str,
    content_id: &str,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    publish_article_content(article_id, content_id, conn)?;
    publish_article_info(article_id, conn)
}

fn publish_article_content(
    atl_id: &str,
    content_id: &str,
    conn: &mut DbConnection,
) -> Result<usize, Error> {
    use self::tb_article_content::dsl;
    diesel::update(dsl::tb_article_content)
        .filter(dsl::article_id.eq(atl_id))
        .set(dsl::status.eq(ARTICLE_STATUS_NEW))
        .execute(conn);
    diesel::update(dsl::tb_article_content)
        .filter(dsl::id.eq(content_id))
        .set(dsl::status.eq(ARTICLE_STATUS_PUBLISHED))
        .execute(conn)
}

fn publish_article_info(article_id: &str, conn: &mut DbConnection) -> Result<usize, Error> {
    use self::tb_article::dsl;
    diesel::update(dsl::tb_article)
        .filter(dsl::id.eq(article_id))
        .set(dsl::status.eq(ARTICLE_STATUS_PUBLISHED))
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
        Ok(article_contents) if article_contents.len() > 0 => {
            let removed: Vec<String> = article_contents
                .iter()
                .map(|article| article.id.clone())
                .collect();
            diesel::delete(dsl::tb_article_content.filter(dsl::id.eq_any(removed))).execute(conn)
        }
        _ => Ok(0),
    }
}
pub fn list_new_article(conn: &mut DbConnection, page_no: i64, page_size: i64) -> ListArticleResult {
    use self::tb_article::dsl;
    let (limit, offset) = db_util::page2limit_offset(page_no, page_size);
    info!("limit:{}, offset:{}", &limit, &offset);

    dsl::tb_article
        .filter(dsl::status.eq(ARTICLE_STATUS_PUBLISHED))
        .order(dsl::update_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<ArticleModel>(conn)
}

pub fn list_recommend_article(
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

pub fn find_article_content_by_id(
    conn: &mut DbConnection,
    find_article_id: &str,
) -> Result<ArticleContentModel, Error> {
    use self::tb_article_content::dsl;
    dsl::tb_article_content
        .filter(dsl::article_id.eq(find_article_id))
        .filter(dsl::status.eq(ARTICLE_STATUS_PUBLISHED))
        .order(dsl::create_at.desc())
        .first(conn)
}
