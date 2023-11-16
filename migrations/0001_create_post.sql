-- Add migration script here
CREATE TABLE post
    IF NOT EXISTS notes (
        id  SERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL UNIQUE,
        content TEXT NOT NULL,
        category VARCHAR(100),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
);