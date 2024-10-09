use crate::schema::tenant;
use diesel::prelude::*;
use rocket::serde::Deserialize;

#[derive(Queryable, Identifiable, Debug, Selectable)]
#[diesel(table_name = tenant)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = tenant)]
pub struct NewTenant<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
