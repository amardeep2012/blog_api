// src/models.rs
use crate::schema::{users, posts};
use serde::{Deserialize, Serialize};
use rocket::form::FromForm;
use crate::schema::posts_tags;

#[derive(Queryable, Serialize,Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}


#[derive(Debug, FromForm)]
pub struct PostQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    pub meta: PaginationMeta,
}


#[derive(Debug, Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub from: i64,
    pub to: i64,
    pub total_pages: i64,
    pub total_docs: i64,
}
#[derive(Queryable, Identifiable,Serialize)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub created_by: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub created_by: i32,
}
#[derive(Insertable)]
#[diesel(table_name = posts_tags)]
pub struct NewPostTag {
    pub post_id: i32,
    pub tag: String,
}


#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = posts_tags)]
#[diesel(primary_key(post_id, tag))]
pub struct PostTag {
    pub post_id: i32,
    pub tag: String,
}

use diesel::prelude::*;
// use diesel::sql_types::Text;
// use diesel::sql_types::Nullable;
// use diesel::sql_types::Integer;
// use diesel::pg::sql_types::Array;
// #[derive(Debug, QueryableByName)]
// pub struct PostWithTagsRow {
//     #[sql_type = "Integer"]
//     pub id: i32,

//     #[sql_type = "Text"]
//     pub title: String,

//     #[sql_type = "Nullable<Text>"]
//     pub body: Option<String>,

//     #[sql_type = "Nullable<Integer>"]
//     pub created_by: Option<i32>,

//     #[sql_type = "Nullable<Text>"]
//     pub username: Option<String>,

//     #[sql_type = "Nullable<Text>"]
//     pub first_name: Option<String>,

//     #[sql_type = "Nullable<Text>"]
//     pub last_name: Option<String>,

//     #[sql_type = "Nullable<Array<Text>>"]
//     pub tag: Option<Vec<String>>,
// }

#[derive(Debug, Serialize)]
pub struct PostWithTagsRow {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_by: Option<i32>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tag: Vec<String>,
}

#[derive(Serialize)]
pub struct PostWithTags {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct CreatedBy {
    pub user_id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
