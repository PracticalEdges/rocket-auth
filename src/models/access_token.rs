use diesel::prelude::*;
use diesel::sql_types::{Char, Text, Timestamp};
use crate::schema::access_token;

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Client)]
#[table_name="access_token"]
pub struct AccessToken {
    pub id: String,
    pub client_id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="access_token"]
pub struct NewAccessToken<'a> {
    pub id: &'a str,
    pub client_id: &'a str,
    pub user_id: &'a str,
    pub token: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
