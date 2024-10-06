mod utils;
use crate::utils::connect_sql::establish_connection;
use rocket::{get, routes, launch};

#[get("/")]
fn hello() -> String {
    format!("Server is running!")
}

#[launch]
async fn rocket() -> _ {
    establish_connection().await;
    rocket::build().mount("/", routes![hello])
}
