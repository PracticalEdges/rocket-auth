mod utils;
mod models;
mod schema;
mod controllers;

use crate::utils::connect_sql::establish_connection;
use crate::controllers::tenant::tenant::create_tenant;
use crate::controllers::client::client::create_client;
use rocket::{get, routes, launch};

#[get("/")]
fn hello() -> String {
    format!("Server is running!")
}

#[launch]
async fn rocket() -> _ {
    establish_connection().await;
    rocket::build().mount("/", routes![hello, create_tenant, create_client])
}
