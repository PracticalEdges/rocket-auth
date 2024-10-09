use rocket::post;
use crate::{models::client::NewClient, utils::connect_sql::establish_connection};
use rocket::serde::json::Json;
// use crate::schema::client::dsl::client;
// use diesel_async::RunQueryDsl;

#[post("/createClient", data = "<new_client>")]
pub async fn create_client(new_client: Json<NewClient<'_>>) -> String {
    // let mut connection = establish_connection().await;
    println!("{:?}", new_client);
    format!("New Client: {:?}", new_client)
}