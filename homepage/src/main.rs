#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::Redirect;
use handlebars::Handlebars;

use rocket_dyn_templates::{Template, handlebars, context};
//use self::handlebars::{Handlebars, JsonRender};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        text: "Helllloooooo",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

