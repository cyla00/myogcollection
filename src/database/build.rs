use std::env;

fn main(){
    dotenv().ok();
    let psql_url = env::var("PSQL_URL").unwrap();

    let psql = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&psql_url)
        .await
        .expect("can't connect to psql database");

    
}