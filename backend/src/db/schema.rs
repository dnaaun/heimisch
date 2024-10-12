// @generated automatically by Diesel CLI.

diesel::table! {
    temp_auths (csrf_token) {
        csrf_token -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (github_user_id) {
        github_user_id -> Int8,
        #[max_length = 255]
        github_username -> Nullable<Varchar>,
        #[max_length = 255]
        github_email -> Nullable<Varchar>,
        github_access_token -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    temp_auths,
    users,
);
