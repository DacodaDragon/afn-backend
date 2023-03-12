// @generated automatically by Diesel CLI.

diesel::table! {
    panelauthor (id) {
        id -> Int4,
        panelscheduleid -> Int4,
        name -> Varchar,
    }
}

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

diesel::joinable!(panelauthor -> panelschedule (id));

diesel::allow_tables_to_appear_in_same_query!(
    panelauthor,
    panelschedule,
);
