// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Text,
        create_time -> Timestamp,
        update_time -> Timestamp,
        status -> Text,
        title -> Text,
        message -> Text,
    }
}
