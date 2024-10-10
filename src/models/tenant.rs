use diesel::prelude::*;
use crate::schema::tenant;
use rocket::serde::Deserialize;

#[derive(Queryable, Identifiable, Debug)]
#[table_name="tenant"]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name="tenant"]
pub struct NewTenant<'a> {
    pub id: &'a str,
    pub name: &'a str,
}