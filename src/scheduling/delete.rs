use rocket::{delete, serde::json::*};
use rocket_okapi::openapi;
use crate::db::establish_connection;

use diesel::*;

use crate::schema::panelschedule::{self, id};

#[openapi(tag = "Event Schedule")]
#[delete("/event", format="json", data="<identifier>")]
pub fn delete_event(identifier: Json<i32>) {
    let mut connection = establish_connection();
    diesel::delete(panelschedule::table.filter(id.eq(identifier.0))).execute(&mut connection).expect("Couldn't delete event");
} 