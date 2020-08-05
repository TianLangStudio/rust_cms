use std::time::SystemTime;

use diesel::sql_types::Datetime;
use serde::{Deserialize, Serialize};

use crate::schema::{tb_article, tb_article_content};

pub const ARTICLE_STATUS_NEW: i32 = 0;
pub const ARTICLE_STATUS_PUBLISHED: i32 = 8;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "tb_article"]
pub struct ArticleModel {
    pub id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub intro: Option<String>,
    pub rcmd_weight: Option<i32>,
    pub url: Option<String>,
    pub status: Option<i32>,
    pub creater: String,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tb_article"]
pub struct NewArticleModel<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub intro: &'a str,
    pub creater: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub subtitle: Option<String>,
    pub intro: String,
    pub content: String,
    pub rcmd_weight: Option<i32>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EditArticle {
    pub id: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub intro: Option<String>,
    pub rcmd_weight: Option<i32>,
    pub url: Option<String>,
    pub status: Option<i32>,
    pub content: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "tb_article"]
pub struct EditArticleModel {
    pub id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub rcmd_weight: Option<i32>,
    pub url: Option<String>,
    pub status: Option<i32>,
    pub intro: Option<String>,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(AsChangeset, Queryable)]
#[table_name = "tb_article_content"]
#[derive(Serialize, Deserialize)]
pub struct ArticleContentModel {
    pub id: String,
    pub status: Option<i32>,
    pub article_id: String,
    pub content: Option<String>,
    pub create_at: Option<chrono::NaiveDateTime>,
}

impl ArticleContentModel {
    pub fn get_content(&self) -> &str {
        match &self.content {
            Some(cnt) => &cnt,
            None => "",
        }
    }
}

#[derive(Insertable)]
#[table_name = "tb_article_content"]
pub struct NewArticleContentModel<'a> {
    pub id: &'a str,
    pub article_id: &'a str,
    pub content: &'a str,
    pub create_at: Option<chrono::NaiveDateTime>,
}

impl <'a> NewArticleContentModel<'a> {
    pub fn new(content_id:&'a str,  article_id: &'a str, content:&'a str) -> Self {
            Self {
                id: content_id,
                article_id,
                content,
                create_at: Some(chrono::Utc::now().naive_local()),
            }
    }
}
