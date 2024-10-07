use diesel::prelude::*;
use crate::schema::user;
use crate::schema::tenant::dsl::tenant;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(tenant)]
#[table_name="user"]
pub struct User {
    pub id: String,
    pub tenant_id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub tenant_id: &'a str,
    pub user_name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}