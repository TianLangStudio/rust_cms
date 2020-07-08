use crate::models::articlemod::*;
use crate::repos;
use crate::schema::{tb_article, tb_article_content};
use diesel::prelude::*;
use diesel::result::Error;
use common::{db_util};
use log::{info, warn};

pub type DbConnection = db_util::DbConnection;

pub  fn add_article(conn:  &DbConnection, new_article: &NewArticle, username: &str) -> Result<usize,  Error> {
       
        conn.transaction::<_, Error, _>(|| {
             let id: String = db_util::uuid();
            let content = &new_article.content;
            let subtitle = match &new_article.subtitle {
                Some(subtitle) => &subtitle,
                None => ""
            };
            let new_article_model = NewArticleModel {
                id: &id,
                title: &new_article.title,
                subtitle: subtitle,
                intro: &new_article.intro,
                creater: username
            };
             let new_article_content = NewArticleContentModel {
                article_id: &new_article_model.id,
                content:  &content,
            };
            diesel::insert_into(tb_article_content::table).values(&new_article_content).execute(conn)?;
            diesel::insert_into(tb_article::table).values(&new_article_model).execute(conn)
        })
}

pub fn edit_article_info(conn: &DbConnection,  edit_article: &EditArticle)  -> Result<usize,  Error>  {
    diesel::update(tb_article::table).set(edit_article).execute(conn)
}


pub fn list_new_article(conn: &DbConnection, page_no: i64,  page_size: i64) -> Result<Vec<ArticleModel>, Error> {
    use self::tb_article::dsl::*;
    let (limit, offset) = db_util::page2limit_offset(page_no, page_size);

    tb_article.limit(limit).offset(offset).load::<ArticleModel>(conn)
}

pub fn find_article_by_id(conn: &DbConnection, id: &str) -> Result<ArticleModel, Error> {
    use self::tb_article::dsl::*;
    tb_article.find(id).order(id.desc()).first(conn)
}

pub fn find_article_content_by_id(conn: &DbConnection, find_article_id: &str) -> Result<ArticleContentModel, Error> {
    use self::tb_article_content::dsl::*;
    tb_article_content.filter(article_id.eq(find_article_id)).order(id.desc()).first(conn)
}



