use diesel::prelude::*;
use crate::schema::client;
use crate::schema::tenant::dsl::tenant;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(tenant)]
#[table_name="client"]
pub struct Client {
    pub id: String,
    pub tenant_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="client"]
pub struct NewClient<'a> {
    pub id: &'a str,
    pub tenant_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
}
