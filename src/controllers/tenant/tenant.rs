use rocket::post;
use crate::{models::tenant::NewTenant, utils::connect_sql::establish_connection};
use rocket::serde::json::Json;
use crate::schema::tenant::dsl::tenant;
use diesel_async::RunQueryDsl;

#[derive(serde::Deserialize)]
pub struct NewTenantCreate<'a> {
    name: &'a str,
}

#[post("/createTenant", data = "<new_tenant_create>")]
pub async fn create_tenant(new_tenant_create: Json<NewTenantCreate<'_>>) -> String {
    // TODO: find a better algorithm for hashing
    let new_tenant = NewTenant {
        id: "1",
        name: new_tenant_create.name,
    };

    diesel::insert_into(tenant)
        .values(new_tenant)
        .execute(&mut establish_connection().await)
        .await
        .expect("Error saving new tenant");


    format!("Tenant created!")
}