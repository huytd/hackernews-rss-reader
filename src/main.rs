#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rss;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod parser;

use parser::*;
use std::io::Read;
use rocket_contrib::Template;
use rocket::response::content;

const RSS_URL: &str = "https://news.ycombinator.com/rss";

#[get("/")]
fn index() -> Template {
    let news = fetch_from(RSS_URL).ok().expect("Could not read RSS");
    Template::render("index", &news)
}

#[post("/preview", data = "<url>")]
fn preview(url: String) -> content::Html<String> {
    let mut resp = reqwest::get(&url).unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content);
    content::Html(content)
}
 
fn main() {
    rocket::ignite()
        .mount("/", routes![index, preview])
        .attach(Template::fairing())
        .launch();
}
