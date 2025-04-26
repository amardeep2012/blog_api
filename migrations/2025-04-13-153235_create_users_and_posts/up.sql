
-- CREATE TABLE users (
--     id SERIAL PRIMARY KEY,
--     username VARCHAR NOT NULL UNIQUE,
--     first_name VARCHAR NOT NULL,
--     last_name VARCHAR NOT NULL
-- );
SELECT 
    posts.id AS post_id, 
    posts.title AS post_title, 
    posts.body AS post_body,
    ARRAY_REMOVE(ARRAY_AGG(posts_tags.tag), NULL) AS tags,
    
    users.id AS user_id,
    users.username AS username,
    users.first_name AS first_name,
    users.last_name AS last_name

FROM posts
LEFT JOIN posts_tags ON posts.id = posts_tags.post_id
LEFT JOIN users ON posts.created_by = users.id
WHERE posts.title ILIKE $1
GROUP BY posts.id, users.id
ORDER BY posts.id DESC
LIMIT $2 OFFSET $3;

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL
);
