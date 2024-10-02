mod utils;
use crate::utils::connect_sql::establish_connection;
use rocket::{get, routes, launch};

#[get("/")]
fn hello() -> String {
    format!("Server is running!")
}

#[launch]
fn rocket() -> _ {
    establish_connection();
    rocket::build().mount("/", routes![hello])
}
