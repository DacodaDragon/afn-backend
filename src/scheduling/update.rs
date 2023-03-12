use chrono::{NaiveDateTime, Local};
use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::{*, schemars::*};
use rocket_okapi::openapi;


use crate::diesel::RunQueryDsl;
use crate::schema::panelschedule;
use crate::models::{NewScheduledPanel};
use crate::db::establish_connection;

/// Represents all data inside of an event
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub start: String,
    pub end: String,
    pub original_start: Option<String>,
    pub original_end: Option<String>
}


#[openapi(tag = "Event Schedule")]
#[post("/event", format="json", data="<event>")]
pub fn post_event(event: Json<Event>) -> Json<Event> {
    let mut connection = establish_connection();

    let start_date_time =  NaiveDateTime::parse_from_str(event.start.as_str(), "%Y-%m-%d %H:%M:%S").expect("Couldn't parse start datetime.");
    let end_date_time = NaiveDateTime::parse_from_str(event.start.as_str(), "%Y-%m-%d %H:%M:%S").expect("Couldn't parse start datetime.");

    let new_event = NewScheduledPanel {
        name: event.name.to_owned(),
        description: event.description.to_owned(),
        created_on: Local::now().naive_local(),
        starts_on: start_date_time,
        ends_on: end_date_time,
        starts_on_originally: start_date_time,
        ends_on_originally: end_date_time,
    };

    diesel::insert_into(panelschedule::table)
            .values(&new_event)
            .execute(&mut connection)
            .expect("Error saving new event schedule");

    event
}