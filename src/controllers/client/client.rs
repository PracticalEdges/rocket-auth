use diesel::prelude::*;
use rocket::post;
use rocket::serde::json::Json;
use crate::{
    models::{client::NewClient, tenant::Tenant},
    utils::connect_sql::establish_connection,
};
use diesel_async::RunQueryDsl;
use crate::schema::tenant::dsl::{tenant, id}; 
use crate::schema::client::dsl::client; 

#[post("/createClient", data = "<new_client>")]
pub async fn create_client(new_client: Json<NewClient<'_>>) -> Result<String, String> {
    let connection = &mut establish_connection().await;

    let tenant_exists: Result<Tenant, diesel::result::Error> = tenant
        .filter(id.eq(new_client.tenant_id))
        .first(connection)
        .await;

    match tenant_exists {
        Ok(_) => {
            diesel::insert_into(client)
                .values(&*new_client)
                .execute(connection)
                .await
                .expect("Error saving new client");
            return Ok("New Client Created".to_string());
        },
        Err(e) => return Err(format!("Error: Tenant does not exist: {}", e)),
    }
}