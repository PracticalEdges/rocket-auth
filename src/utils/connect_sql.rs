use diesel_async::{AsyncConnection, AsyncMysqlConnection};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> AsyncMysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncMysqlConnection::establish(&database_url).await.expect("Error establishing connection")
}