use diesel::prelude::*;
use diesel::sql_types::{Char, Varchar, Timestamp};
use crate::schema::{user, tenant};

#[derive(Queryable, Identifiable, Associations)]
pub struct User {
    pub id: String,
    pub tenant_id: String,
    pub user_id: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub tenant_id: &'a str,
    pub user_id: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
}