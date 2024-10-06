use crate::schema::{client, refresh_token, user};
use diesel::prelude::*;
use diesel::sql_types::{Char, Text, Timestamp};

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Client)]
#[table_name = "refresh_token"]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "refresh_token"]
pub struct NewRefreshToken<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub token: &'a str,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}
