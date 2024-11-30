// @generated automatically by Diesel CLI.

diesel::table! {
    installations (id) {
        id -> Int8,
        created_at -> Timestamp,
        github_user_id -> Int8,
    }
}

diesel::table! {
    login_users (github_user_id) {
        github_user_id -> Int8,
        github_username -> Text,
        github_email -> Nullable<Text>,
        github_access_token -> Text,
        last_last_in_touch_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        data -> Jsonb,
        expiry_date -> Timestamp,
    }
}

diesel::table! {
    temp_auths (csrf_token) {
        csrf_token -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    webhooks (id) {
        webhook_content -> Jsonb,
        installation_id -> Int8,
        created_at -> Timestamp,
        id -> Int8,
    }
}

diesel::joinable!(webhooks -> installations (installation_id));

diesel::allow_tables_to_appear_in_same_query!(
    installations,
    login_users,
    sessions,
    temp_auths,
    webhooks,
);
