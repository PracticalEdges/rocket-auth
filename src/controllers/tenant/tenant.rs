use rocket::post;
use crate::{models::tenant::NewTenant, utils::connect_sql::establish_connection};
use rocket::serde::json::Json;
use crate::schema::tenant::dsl::tenant;
use diesel_async::RunQueryDsl;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use std::env;
use rocket::error;

#[derive(serde::Deserialize)]
pub struct NewTenantCreate<'a> {
    name: &'a str,
}

#[derive(serde::Serialize)]
pub struct CreateTenantResponse {
    action: String,
    tenant_key: String,
}

#[post("/createTenant", data = "<new_tenant_create>")]
pub async fn create_tenant(new_tenant_create: Json<NewTenantCreate<'_>>) -> Json<CreateTenantResponse> {

    let size: Result<String, String> = env::var("SIZE_LEN_LIMIT_STR").map_err(|e|{
        error!("Error: {}", e);
        "Size must be set".to_string()
    });

    let rand_hash: String = generate_random_hash_function(size.unwrap().parse().unwrap());

    let new_tenant: NewTenant<'_> = NewTenant {
        id: rand_hash.as_str(),
        name: new_tenant_create.name,
    };

    let _ = diesel::insert_into(tenant)
        .values(new_tenant)
        .execute(&mut establish_connection().await)
        .await
        .map_err(|e| {
            error!("Error saving new tenant: {}", e);
            "Error saving new tenant".to_string()
        });

    let key: String = env::var("ENCRYPTION_KEY").expect("Key must be set");

    let encrypted_text: String = encrypt(rand_hash.as_str(), key.as_str(), 16);

    Json(CreateTenantResponse {
        action: "Created Tenant successfully!".to_string(),
        tenant_key: encrypted_text,
    })
}