CREATE TABLE posts_tags (
    post_id INTEGER NOT NULL REFERENCES posts(id),
    tag TEXT NOT NULL
);

