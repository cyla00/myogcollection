use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;
use std::time::Duration;

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

    sqlx::query("DROP TABLE IF EXISTS users, patterns, comments").execute(&psql).await.expect("error destoying build");

    println!("db destroy ok")
}
 