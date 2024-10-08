use rocket::post;
use crate::{models::tenant::NewTenant, utils::connect_sql::establish_connection};
use rocket::serde::json::Json;
use crate::schema::tenant::dsl::tenant;
use diesel_async::RunQueryDsl;

#[post("/createTenant", data = "<new_tenant>")]
pub async fn create_tenant(new_tenant: Json<NewTenant<'_>>) -> String {
    let mut connection = establish_connection().await;
    diesel::insert_into(tenant)
        .values(new_tenant.into_inner())
        .execute(&mut connection)
        .await
        .expect("Error saving new tenant");
    format!("Tenant created!")
}