use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

#[tokio::main]
async fn main(){
    dotenv().ok();
    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");
    let _pool= match PgPoolOptions::new()
                .max_connections(10)
                .connect(&db_url)
                .await
    {
        Ok(pool) => {
            println!("💯Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("/n🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
}
