#[macro_use] extern crate rocket;

use chrono::prelude::*;
use std::fmt;
use rocket::Request;
use rocket::response::Redirect;
use handlebars::Handlebars;
use rocket_dyn_templates::{Template, handlebars, context};

#[get("/")]
fn index() -> Template {

    let now: DateTime<Utc> = Utc::now();
    Template::render("index", context! {
        text: "Helllloooooo",
        render_time: now.to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

