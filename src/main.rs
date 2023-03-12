#[macro_use]
extern crate diesel;

mod scheduling;
mod schema;
mod models;
mod db;

use rocket::*;
use rocket_okapi::{openapi_get_routes};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use dotenv::dotenv;
use std::env;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let _rocket = rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                scheduling::get::get_events,
                scheduling::get::get_event,
                scheduling::update::post_event,
                scheduling::delete::delete_event,
                scheduling::create::create_event,
            ],
        )
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount("/v1", routes![
            scheduling::get::get_events,
            scheduling::get::get_event,
            scheduling::update::post_event,
            scheduling::create::create_event,
            scheduling::delete::delete_event,
        ])
        .launch()
        .await?;

    Ok(())
}
