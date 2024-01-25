CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(40) NOT NULL UNIQUE,
    username VARCHAR(200) NOT NULL UNIQUE,
    email VARCHAR(200) NOT NULL UNIQUE,
    password VARCHAR(200) NOT NULL,
    active BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_login TIMESTAMPTZ
);


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
    created_at TIMESTAMPTZ DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS comments (
    id VARCHAR(40) NOT NULL UNIQUE,
    owner_id VARCHAR(40) NOT NULL,
    pattern_id VARCHAR(40) NOT NULL,
    comment VARCHAR(500) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);


INSERT INTO users(id, username, email, password, active) 
VALUES (
    'bbf2aed1-368d-4d14-b64d-c9a18a4f50d8',
    'username',
    'ikhayam000@protonmail.com',
    '$argon2id$v=19$m=16,t=2,p=1$cGFzc3dvcmQ$8vDS3rsezOjrur01dF12EA', -- salt:'password' pwd:'password' (argon2)
    false
) ON CONFLICT (id) DO NOTHING;


INSERT INTO patterns (id, owner_id, title, pattern_description, gallery_paths, pattern_path, materials, tools, category)
VALUES (
    '305d07a7-06b8-43d2-937e-05002d969b28',
    'bbf2aed1-368d-4d14-b64d-c9a18a4f50d8',
    'plate carrier',
    'this is a pattern for a minimal platecarrier',
    '{"/gallerey/img1.jpeg", "/gallerey/img2.jpeg", "/gallerey/img3.jpeg"}',
    '/patterns/pattern.pdf',
    '{"cordura", "cotton", "velcro"}',
    '{"machine", "scissors", "thread"}',
    'somecategory'
) ON CONFLICT (id) DO NOTHING;


INSERT INTO comments (id, owner_id, pattern_id, comment)
VALUES (
    '17a89cfd-edab-419b-86e9-6dffb8821949',
    'bbf2aed1-368d-4d14-b64d-c9a18a4f50d8',
    '305d07a7-06b8-43d2-937e-05002d969b28',
    'very cool pattern thanks for sharing'
) ON CONFLICT (id) DO NOTHING;