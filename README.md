Steps to run project:
1. Dwonload the Docker.
2. Run the command docker-compose up -d. It will run the postgres container.
3. Go to src folder and run command cargo run 
4. Test all the api

cargo install diesel_cli --no-default-features --features postgres

diesel setup
diesel migration generate create_users_and_posts
diesel migration run
diesel print-schema > src/schema.rs
diesel print-schema > src/schema.rs


#[get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    conn: DbConn,
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
) -> Json<PaginatedResponse<PostWithTags>> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let search = search.unwrap_or_default();
    let offset = (page - 1) * limit;

    let results = conn
        .run(move |c| {
            use crate::schema::posts::dsl::*;

            let total_docs: i64 = posts
                .filter(title.ilike(format!("%{}%", search)))
                .count()
                .get_result(c)
                .unwrap();

            let query = diesel::sql_query(
                r#"
                SELECT 
                    posts.id as db_id, 
                    posts.title as post_title, 
                    posts.body as post_body, 
                    ARRAY_REMOVE(ARRAY_AGG(posts_tags.tag), NULL) as tags
                FROM posts
                LEFT JOIN posts_tags ON posts.id = posts_tags.post_id
                WHERE posts.title ILIKE $1
                GROUP BY posts.id
                ORDER BY posts.id DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind::<diesel::sql_types::Text, _>(format!("%{}%", search))
            .bind::<diesel::sql_types::Integer, _>(limit as i32)
            .bind::<diesel::sql_types::Integer, _>(offset as i32);

            #[derive(QueryableByName)]
            struct PostWithTagsRow {
                #[diesel(sql_type = diesel::sql_types::Integer)]
                pub db_id: i32,

                #[diesel(sql_type = diesel::sql_types::Text)]
                pub post_title: String,

                #[diesel(sql_type = diesel::sql_types::Text)]
                pub post_body: String,

                #[diesel(sql_type = diesel::sql_types::Array<diesel::sql_types::Text>)]
                pub tags: Vec<String>,
            }

            let raw_rows: Vec<PostWithTagsRow> = query.get_results(c).unwrap();

            let posts_with_tags: Vec<PostWithTags> = raw_rows
                .into_iter()
                .map(|row| PostWithTags {
                    id: row.db_id,
                    title: row.post_title,
                    body: row.post_body,
                    tags: row.tags,
                })
                .collect();

            let total_pages = ((total_docs as f64) / (limit as f64)).ceil() as u32;

            let meta = PaginationMeta {
                current_page: page,
                per_page: limit,
                from: offset + 1,
                to: offset + posts_with_tags.len() as i64,
                total_pages: total_pages as i64,
                total_docs: total_docs as i64,
            };

            PaginatedResponse {
                records: posts_with_tags,
                meta,
            }
        })
        .await;

    Json(results)
}

 curl -X POST http://localhost:8000/posts \     
  -H "Content-Type: application/json" \
  -d '{
        "title": "My second post",
        "body": "This is a post about Rust + Rocket + Diesel and their usages!",
        "tags": ["rust", "webdev", "rocket", "backen"]
      }'
curl -X POST http://localhost:8000/users \
     -H "Content-Type: application/json" \
     -d '{
           "username": "alice123",
           "first_name": "Alice",
           "last_name": "Wonderland"
         }'

curl "http://localhost:8000/posts?page=1&limit=10&search=rust"
