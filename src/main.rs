#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Server is running!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
