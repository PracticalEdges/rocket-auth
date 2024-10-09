// @generated automatically by Diesel CLI.

diesel::table! {
    access_token (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 36]
        client_id -> Char,
        #[max_length = 36]
        user_id -> Char,
        token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    authorization_code (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 36]
        client_id -> Char,
        #[max_length = 36]
        user_id -> Char,
        code -> Text,
        redirect_uri -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    client (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 36]
        tenant_id -> Char,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        client_secret -> Varchar,
        #[max_length = 255]
        redirect_uri -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    refresh_token (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 36]
        client_id -> Char,
        #[max_length = 36]
        user_id -> Char,
        token -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tenant (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 36]
        tenant_id -> Char,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(access_token -> client (client_id));
diesel::joinable!(access_token -> user (user_id));
diesel::joinable!(authorization_code -> client (client_id));
diesel::joinable!(authorization_code -> user (user_id));
diesel::joinable!(client -> tenant (tenant_id));
diesel::joinable!(refresh_token -> client (client_id));
diesel::joinable!(refresh_token -> user (user_id));
diesel::joinable!(user -> tenant (tenant_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_token,
    authorization_code,
    client,
    refresh_token,
    tenant,
    user,
);
