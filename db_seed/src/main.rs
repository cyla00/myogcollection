use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;
use std::time::Duration;

use datatypes::{ User, Pattern, Comment };
use chrono::Local;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let psql_url: String = env::var("PSQL_URL").unwrap();

    let psql: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&psql_url)
        .await
        .expect("can't connect to psql database");

    let time = Local::now();

    let mut gallery_array: Vec<String> = Vec::new();
    gallery_array.push("/gallerey/img1.jpeg".to_string());
    gallery_array.push("/gallerey/img2.jpeg".to_string());
    gallery_array.push("/gallerey/img3.jpeg".to_string());

    let mut materials_array: Vec<String> = Vec::new();
    materials_array.push("cordura".to_string());
    materials_array.push("cotton".to_string());
    materials_array.push("velcro".to_string());

    let mut tools_array: Vec<String> = Vec::new();
    tools_array.push("machine".to_string());
    tools_array.push("scissors".to_string());
    tools_array.push("thread".to_string());

    let seed_user: User = User {
        id: "bbf2aed1-368d-4d14-b64d-c9a18a4f50d8".to_string(),
        username: "username".to_string(),
        email: "ikhayam000@protonmail.com".to_string(),
        password: "$argon2id$v=19$m=16,t=2,p=1$cGFzc3dvcmQ$8vDS3rsezOjrur01dF12EA".to_string(), // salt:"password" pwd:"password" (argon2)
        active: false,
        created_at: time,
        last_login: time,
    };

    let seed_pattern: Pattern = Pattern {
        id: "305d07a7-06b8-43d2-937e-05002d969b28".to_string(),
        owner_id: "bbf2aed1-368d-4d14-b64d-c9a18a4f50d8".to_string(),
        title: "plate carrier".to_string(),
        pattern_description: "this is a pattern for a minimal platecarrier".to_string(),
        gallery_paths: gallery_array,
        pattern_path: "/patterns/pattern.pdf".to_string(),
        materials: materials_array,
        tools: tools_array,
        category: "category1".to_string(),
        created_at: time,
    };

    let seed_comment: Comment = Comment {
        id: "17a89cfd-edab-419b-86e9-6dffb8821949".to_string(),
        owner_id: "bbf2aed1-368d-4d14-b64d-c9a18a4f50d8".to_string(),
        pattern_id: "305d07a7-06b8-43d2-937e-05002d969b28".to_string(),
        comment: "very cool pattern thanks for sharing".to_string(),
        created_at: time,
    };

    sqlx::query("
        INSERT INTO users (id, username, email, password, active, created_at, last_login)
        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO NOTHING;
    ")
    .bind(seed_user.id)
    .bind(seed_user.username)
    .bind(seed_user.email)
    .bind(seed_user.password)
    .bind(seed_user.active)
    .bind(seed_user.created_at)
    .bind(seed_user.last_login)
    .execute(&psql).await.expect("error user seed");

    sqlx::query("
        INSERT INTO patterns (id, owner_id, title, pattern_description, gallery_paths, pattern_path, materials, tools, category, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT (id) DO NOTHING;
    ")
    .bind(seed_pattern.id)
    .bind(seed_pattern.owner_id)
    .bind(seed_pattern.title)
    .bind(seed_pattern.pattern_description)
    .bind(seed_pattern.gallery_paths)
    .bind(seed_pattern.pattern_path)
    .bind(seed_pattern.materials)
    .bind(seed_pattern.tools)
    .bind(seed_pattern.category)
    .bind(seed_pattern.created_at)
    .execute(&psql).await.expect("error pattern seed");

    sqlx::query("
        INSERT INTO comments (id, owner_id, pattern_id, comment, created_at)
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING;
    ")
    .bind(seed_comment.id)
    .bind(seed_comment.owner_id)
    .bind(seed_comment.pattern_id)
    .bind(seed_comment.comment)
    .bind(seed_comment.created_at)
    .execute(&psql).await.expect("error comment seed");

    println!("db seed ok")
}