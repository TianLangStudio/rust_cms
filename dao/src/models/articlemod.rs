use std::time::SystemTime;

use diesel::sql_types::Datetime;
use serde::{Deserialize, Serialize};

use crate::schema::{tb_article, tb_article_content};

pub const ARTICLE_STATUS_NEW: i32 = 0;
pub const ARTICLE_STATUS_UNDER_REVIEW: i32 = 1;
pub const ARTICLE_STATUS_PUBLISHED: i32 = 8;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[diesel(table_name = tb_article)]
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
/*
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[diesel(table_name = tb_draft_article)]
pub struct ArticleDraftModel {
    pub id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub intro: Option<String>,
    pub rcmd_weight: Option<i32>,
    pub url: Option<String>,
    pub status: Option<i32>,
    pub approver: Option<String>,
    pub creater: String,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}
impl ArticleDraftModel {
    pub fn to_publish_model(&self) -> ArticleModel {
        ArticleModel{
            id: self.id.clone(),
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
            intro: self.intro.clone(),
            rcmd_weight: self.rcmd_weight.clone(),
            url: self.url.clone(),
            status: self.status.clone(),
            creater: self.creater.clone(),
            create_at: self.create_at.clone(),
            update_at: self.update_at.clone()
        }
    }
}*/
/*impl From<ArticleDraftModel> for ArticleModel{
    fn from(draft: ArticleDraftModel) -> Self {
        ArticleModel {
            id: draft.id,
            title: draft.title,
            subtitle: draft.subtitle,
            intro: draft.intro,
            rcmd_weight: draft.rcmd_weight,
            url: draft.url,
            status: draft.status,
            creater: draft.creater,
            create_at: draft.create_at,
            update_at: draft.update_at
        }
    }
}*/
#[derive(Insertable)]
#[diesel(table_name = tb_article)]
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
#[diesel(table_name = tb_article)]
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
#[diesel(table_name = tb_article_content)]
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
#[diesel(table_name = tb_article_content)]
pub struct NewArticleContentModel<'a> {
    pub id: &'a str,
    pub status: i32,
    pub article_id: &'a str,
    pub content: &'a str,
    pub create_at: Option<chrono::NaiveDateTime>,
}

impl<'a> NewArticleContentModel<'a> {
    pub fn new(content_id: &'a str, article_id: &'a str, content: &'a str) -> Self {
        Self {
            id: content_id,
            status: ARTICLE_STATUS_NEW,
            article_id,
            content,
            create_at: Some(chrono::Utc::now().naive_local()),
        }
    }
}
