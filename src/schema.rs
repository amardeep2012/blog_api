// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        created_by -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    posts_tags (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        tag -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

diesel::joinable!(posts -> users (created_by));
diesel::joinable!(posts_tags -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    posts_tags,
    users,
);
