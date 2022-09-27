#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::Redirect;
use handlebars::Handlebars;

use rocket_dyn_templates::{Template, handlebars, context};
//use self::handlebars::{Handlebars, JsonRender};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

