CREATE TABLE IF NOT EXISTS patterns (
    id VARCHAR(100) PRIMARY KEY,
    owner_id VARCHAR(100) NOT NULL,
    title VARCHAR(100) NOT NULL,
    pattern_description VARCHAR(500) NOT NULL,
    gallery_paths VARCHAR(100)[] NOT NULL,
    pattern_path VARCHAR(100) NOT NULL,
    materials VARCHAR(100)[] NOT NULL,
    tools VARCHAR(100)[] NOT NULL,
    category VARCHAR(100) NOT NULL,
    created_at DATE NOT NULL
)