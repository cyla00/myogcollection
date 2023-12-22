CREATE TABLE [IF NOT EXISTS] users (
    id VARCHAR(50) PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(500) NOT NULL,
    createdAt DATETIME NULL,
    active BOOLEAN DEFAULT false NOT NULL,
)