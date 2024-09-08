// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Text,
        is_admin -> Bool,
        is_staff -> Bool,
        is_view -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
