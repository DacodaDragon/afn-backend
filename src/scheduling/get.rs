use rocket::{get, serde::json::Json};
use rocket_okapi::{*, okapi::schemars};
use diesel::*;

use crate::serde::*;
use crate::models::{ScheduledPanel};
use crate::db::establish_connection;

/// Represents a planned event
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// The id of the event
    pub id: String,
    /// The name of the event
    pub name: String,
    /// The description of the event
    pub description: Option<String>,
    /// The actual start of the event, format: YYYY-MM-DD HH:MM:SS
    pub start: String,
    /// The actual end of the event, format: YYYY-MM-DD HH:MM:SS
    pub end: String,
    /// The expected start of the event, format: YYYY-MM-DD HH:MM:SS
    pub original_start: Option<String>,
    /// The expected end of the event, format: YYYY-MM-DD HH:MM:SS
    pub original_end: Option<String>,
}

#[openapi(tag = "Event Schedule")]
#[get("/events", format="json")]
pub fn get_events() -> Json<Vec<Event>> {
    let mut connection = establish_connection();

    use crate::schema::panelschedule::dsl::*;

    let events = panelschedule.load::<ScheduledPanel>(&mut connection).expect("scream");
    
    let mut json_events = Vec::new();
    for item in events.iter() {
        json_events.push(Event{
            id: item.id.to_owned().to_string(),
            name: item.name.to_owned(),
            description: item.description.to_owned(),
            start: item.starts_on.to_string(),
            end: item.ends_on.to_string(),
            original_start: Some(item.starts_on_originally.to_string()),
            original_end: Some(item.ends_on_originally.to_string()),

        });
    };

    Json(json_events)
}
#[openapi(tag = "Event Schedule")]
#[get("/event", format="json")]
pub fn get_event() -> Json<Event> {
    Json(Event{
        id: "".to_string(),
        name: "".to_string(),
        description: Some("".to_string()),
        start: "".to_string(),
        end: "".to_string(),
        original_start: Some("".to_string()),
        original_end: Some("".to_string())
    })
}