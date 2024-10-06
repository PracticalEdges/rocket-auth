use diesel::prelude::*;
use diesel::sql_types::{Char, Text, Timestamp};
use crate::schema::{authorization_code, user, client};

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to[Tenant]]
#[table_name="authorization_code"]
pub struct AuthorizationCode {
    pub id: String,
    pub client_id: String,
    pub user_id: String,
    pub code: String,
    pub redirect_uri: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="authorization_code"]
pub struct NewAuthorizationCode<'a> {
    pub id: &'a str,
    pub client_id: &'a str,
    pub user_id: &'a str,
    pub code: &'a str,
    pub redirect_uri: &'a str,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}