// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        title -> Varchar,
        is_completed -> Bool,
    }
}
