// @generated automatically by Diesel CLI.

diesel::table! {
    panelschedule (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_on -> Timestamp,
        starts_on -> Timestamp,
        ends_on -> Timestamp,
        starts_on_originally -> Timestamp,
        ends_on_originally -> Timestamp,
    }
}
