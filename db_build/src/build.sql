CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(40) NOT NULL UNIQUE,
    username VARCHAR(200) NOT NULL UNIQUE,
    email VARCHAR(200) NOT NULL UNIQUE,
    password VARCHAR(200) NOT NULL,
    active BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_login TIMESTAMP
)

CREATE TABLE IF NOT EXISTS patterns (
    id VARCHAR(40) NOT NULL UNIQUE,
    owner_id VARCHAR(40) NOT NULL,
    title VARCHAR(100) NOT NULL,
    pattern_description VARCHAR(500) NOT NULL,
    gallery_paths text[] NOT NULL,
    pattern_path VARCHAR(200) NOT NULL,
    materials text[] NOT NULL,
    tools text[] NOT NULL,
    category VARCHAR(200) NOT NULL,
    created_at TIMESTAMP NOT NULL
)

CREATE TABLE IF NOT EXISTS comments (
    id VARCHAR(40) NOT NULL UNIQUE,
    owner_id VARCHAR(40) NOT NULL,
    pattern_id VARCHAR(40) NOT NULL,
    comment VARCHAR(500) NOT NULL,
    created_at TIMESTAMP NOT NULL
)