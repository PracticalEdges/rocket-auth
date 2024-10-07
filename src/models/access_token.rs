use diesel::prelude::*;
use crate::schema::access_token;
use crate::schema::client::dsl::client;
use crate::schema::user::dsl::user;

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(user)]
#[belongs_to(client)]
#[table_name="access_token"]
pub struct AccessToken {
    pub id: String,
    pub client_id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="access_token"]
pub struct NewAccessToken<'a> {
    pub id: &'a str,
    pub client_id: &'a str,
    pub user_id: &'a str,
    pub token: &'a str,
}
