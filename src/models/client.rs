use diesel::prelude::*;
use diesel::sql_types::{Char, Text, Timestamp};
use crate::schema::{client, tenant};

#[derive(Queryable)]
#[belongs_to[Tenant]]
#[table_name="client"]
pub struct Client {
    pub id: String,
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="client"]
pub struct NewClient<'a> {
    pub id: &'a str,
    pub tenant_id: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
