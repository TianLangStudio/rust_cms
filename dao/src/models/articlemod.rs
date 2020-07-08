use serde::{Deserialize, Serialize};
use crate::schema::{tb_article, tb_article_content};

#[derive(Serialize, Deserialize)]
#[derive(AsChangeset)]
#[derive(Insertable)]
#[derive(Queryable)]
#[table_name = "tb_article"]
pub struct ArticleModel {
    pub id : String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub intro: Option<String>,
    pub creater: String,
}

#[derive(Insertable)]
#[table_name = "tb_article"]
pub struct NewArticleModel<'a> {
    pub id: &'a str,
    pub title : &'a str,
    pub subtitle: &'a str,
    pub intro: &'a str,
    pub creater: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct NewArticle {
    pub title : String,
    pub subtitle: Option<String>,
    pub intro: String,
    pub content: String
}

#[derive(Serialize, Deserialize)]
#[derive(AsChangeset)]
#[table_name="tb_article"]
pub struct EditArticle {
       pub id: String,
       pub title: Option<String>,
       pub subtitle: Option<String>,
       pub intro: Option<String>,
}

#[derive(AsChangeset)]
#[derive(Queryable)]
#[table_name = "tb_article_content"]
pub struct ArticleContentModel {
    id: i64,
    article_id: String,
    content: Option<String>
}

impl ArticleContentModel {
    pub fn get_content(&self) -> &str {
            match &self.content {
                Some(cnt) =>  &cnt,
                None => ""
            }
    }
}


#[derive(Insertable)]
#[table_name = "tb_article_content"]
pub struct  NewArticleContentModel<'a>{
    pub article_id: &'a str,
    pub content:  &'a str,
} 