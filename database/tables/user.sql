CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(100) PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    user_password VARCHAR(500) NOT NULL,
    created_at DATE NOT NULL,
    active BOOLEAN DEFAULT false NOT NULL
)