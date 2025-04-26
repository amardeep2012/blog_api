use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::get;
use diesel::prelude::*;
use serde::Deserialize;
use crate::DbConn;

use crate::models::{NewUser, User, NewPost, Post, NewPostTag, PostWithTagsRow, NewPostRequest};
use crate::schema::{users, posts, posts_tags};
use std::collections::HashMap;

#[post("/users", data = "<new_user>")]
pub async fn create_user(conn: DbConn, new_user: Json<NewUser>) -> Json<User> {
    let new_user = new_user.into_inner();

    let inserted = conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(c)
            .unwrap()
    }).await;

    Json(inserted)
}



#[post("/posts", format = "json", data = "<data>")]
pub async fn create_post(conn: DbConn, data: Json<NewPostRequest>) -> Json<Post> {
    use crate::schema::posts::dsl::*;
    use crate::schema::posts_tags::dsl::*;
    use diesel::prelude::*;

    // Clone necessary values before the closure to avoid lifetime issues
    let post_title = data.title.clone();
    let post_body = data.body.clone();
    let post_tags = data.tags.clone();

    let result = conn.run(move |c| {
        // Use the renamed variables to avoid confusion with schema
        let new_post = NewPost {
            title: &post_title,
            body: &post_body,
            created_by: 1,  // Replace with actual user ID
        };

        let inserted_post: Post = diesel::insert_into(posts)
            .values(&new_post)
            .get_result(c)?;

        // Rename `post_id` to avoid conflict with schema
        let new_post_id = inserted_post.id;  // Renamed variable

        let tag_data: Vec<NewPostTag> = post_tags
            .into_iter()
            .map(|tag_str| NewPostTag {
                post_id: new_post_id,  // Use renamed variable
                tag: tag_str,
            })
            .collect();

        diesel::insert_into(posts_tags)
            .values(&tag_data)
            .execute(c)?;

        Ok::<_, diesel::result::Error>(inserted_post)
    }).await;

    match result {
        Ok(post) => Json(post),
        Err(_) => panic!("Failed to create post"), // Replace with proper error handling later
    }
}


pub async fn get_posts_paginated(
    conn: &DbConn,
    page: Option<usize>,
    limit: Option<usize>,
    search: Option<String>,
) -> Result<Vec<PostWithTagsRow>, diesel::result::Error> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    let search_term = search.unwrap_or_default();
    let like_pattern = format!("%{}%", search_term);

    // Step 1: Query posts with users (without tags)
    let posts_with_users = conn.run(move |c| {
        posts::table
            .left_join(users::table.on(users::id.eq(posts::created_by)))
            .filter(
                posts::title.ilike(&like_pattern)
                    .or(posts::body.ilike(&like_pattern))
            )
            .select((
                posts::id,
                posts::title,
                posts::body,
                users::id.nullable(),
                users::username.nullable(),
                users::first_name.nullable(),
                users::last_name.nullable(),
            ))
            .order(posts::id.desc())
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<(i32, String, String, Option<i32>, Option<String>, Option<String>, Option<String>)>(c)
    }).await?;

    let post_ids: Vec<i32> = posts_with_users.iter().map(|r| r.0).collect();

    // Step 2: Query tags per post_id
    let tags_map = conn.run(move |c| {
        posts_tags::table
            .filter(posts_tags::post_id.eq_any(&post_ids))
            .select((posts_tags::post_id, posts_tags::tag))
            .load::<(Option<i32>, String)>(c)
            .map(|rows| {
                let mut map: HashMap<i32, Vec<String>> = HashMap::new();
                for (maybe_post_id, tag) in rows {
                    if let Some(post_id) = maybe_post_id {
                        map.entry(post_id).or_insert_with(Vec::new).push(tag);
                    }
                }
                map
            })
            
    }).await?;

    // Step 3: Combine into PostWithTagsRow
    let result: Vec<PostWithTagsRow> = posts_with_users.into_iter().map(|(id, title, body, user_id, username, first_name, last_name)| {
        PostWithTagsRow {
            id,
            title,
            body,
            created_by: user_id,
            username,
            first_name,
            last_name,
            tag: tags_map.get(&id).cloned().unwrap_or_default(),
        }
    }).collect();

    Ok(result)
}
#[get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    conn: DbConn,
    page: Option<usize>,
    limit: Option<usize>,
    search: Option<String>,
) -> Result<Json<Vec<PostWithTagsRow>>, Status> {
    match get_posts_paginated(&conn, page, limit, search).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}



