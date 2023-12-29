CREATE TABLE IF NOT EXISTS comments (
    id VARCHAR(100) PRIMARY KEY,
    owner_id VARCHAR(100) NOT NULL,
    pattern_id VARCHAR(100) NOT NULL,
    comment VARCHAR(500) NOT NULL,
    created_at DATE NOT NULL
)