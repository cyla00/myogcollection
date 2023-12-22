CREATE TABLE [IF NOT EXISTS] patterns (
    id VARCHAR(50) PRIMARY KEY,
    owner_id VARCHAR(100) NOT NULL,
    _path VARCHAR(100) NOT NULL,
    tags BOOLEAN DEFAULT false NOT NULL,
    createdAt DATETIME NULL,
)