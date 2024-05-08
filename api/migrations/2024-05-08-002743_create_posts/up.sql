CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO posts (title, body, published) VALUES ('Hello, T5!', 'This is an example post in the DB.', TRUE);
