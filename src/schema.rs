// @generated automatically by Diesel CLI.

diesel::table! {
    client_keys (id) {
        id -> Integer,
        key -> Text,
        is_active -> Integer,
    }
}
