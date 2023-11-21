CREATE TABLE IF NOT EXISTS post (
    id SERIAL PRIMARY KEY, 
    title VARCHAR(255) NOT NULL UNIQUE, 
    slug VARCHAR(255) NOT NULL UNIQUE, 
    author_id INT NOT NULL,
    excerpt TEXT NOT NULL, 
    content TEXT NOT NULL, 
    category_id INT, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(), 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_author FOREIGN KEY(author_id) REFERENCES author(id) ON DELETE CASCADE,
    CONSTRAINT fk_category FOREIGN KEY(category_id) REFERENCES category(id) ON DELETE SET NULL
);