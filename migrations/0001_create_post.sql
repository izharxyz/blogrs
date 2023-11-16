-- Add migration script here
CREATE TABLE post (
    id  SERIAL PRIMARY KEY,
    title varchar(255) NOT NULL,
    author varchar(255) NOT NULL,
    content text NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);