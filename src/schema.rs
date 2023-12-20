// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
