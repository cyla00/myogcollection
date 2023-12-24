CREATE TABLE IF NOT EXISTS patterns (
    id VARCHAR(100) PRIMARY KEY,
    owner_id VARCHAR(100) NOT NULL,
    pattern_path VARCHAR(100) NOT NULL,
    tags BOOLEAN DEFAULT false NOT NULL,
    created_at DATE NULL
)