use diesel_async::{AsyncConnection, AsyncMysqlConnection};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match AsyncMysqlConnection::establish(&database_url).await {
        Ok(_) => println!("Connection established successfully."),
        Err(e) => eprintln!("Error establishing connection: {}", e),
    }
}