// @generated automatically by Diesel CLI.

diesel::table! {
    app_user (id) {
        id -> Int8,
        username -> Varchar,
        updated_at -> Nullable<Timestamp>,
    }
}
