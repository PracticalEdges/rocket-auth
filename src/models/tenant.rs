use diesel::prelude::*;
use diesel::sql_types::{Char, Text, Timestamp};
use crate::schema::tenant;

#[derive(Queryable, Indentifiable, Associations)]
#[table_name="tenant"]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tenant"]
pub struct NewTenant<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub created_at: chrono::NaiveDateTime,
}