use chrono::{NaiveDateTime, Local};
use rocket::{put, serde::json::Json};
use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::{*, schemars::*};
use rocket_okapi::openapi;
use diesel::*;

use crate::models::{NewScheduledPanel};
use crate::db::establish_connection;
use crate::schema::panelschedule::{self, id};


/// Represents a planned event
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventId {
    pub id: i32
}

/// Represents a planned event
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub start: String,
    pub end: String
}

#[openapi(tag = "Event Schedule")]
#[put("/event", format="json", data="<event>")]
pub fn create_event(event: Json<NewEvent>) -> Json<EventId> {
    let mut connection = establish_connection(); 

    let start_date_time =  NaiveDateTime::parse_from_str(event.start.as_str(), "%Y-%m-%d %H:%M:%S").expect("Couldn't parse start datetime.");
    let end_date_time = NaiveDateTime::parse_from_str(event.end.as_str(), "%Y-%m-%d %H:%M:%S").expect("Couldn't parse start datetime.");

    let new_event = NewScheduledPanel {
        name: event.name.to_owned(),
        description: event.description.to_owned(),
        created_on: Local::now().naive_local(),
        starts_on: start_date_time,
        ends_on: end_date_time,
        starts_on_originally: start_date_time,
        ends_on_originally: end_date_time,
    };

    let created_event : i32 = diesel::insert_into(panelschedule::table)
            .values(&new_event)
            .returning(id)
            .get_result(&mut connection)
            .expect("Error saving new event schedule");

    Json(EventId{
        id: created_event.to_owned(),
    })
}