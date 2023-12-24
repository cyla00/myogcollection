CREATE TABLE IF NOT EXISTS comments (
    id VARCHAR(100) PRIMARY KEY,
    owner_id VARCHAR(100) NOT NULL,
    pattern_id VARCHAR(100) NOT NULL,
    rating FLOAT DEFAULT false NOT NULL,
    created_at DATE NULL
)