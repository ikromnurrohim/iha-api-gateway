// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        email -> Nullable<Text>,
        is_admin -> Bool,
        is_staff -> Bool,
        is_view -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
