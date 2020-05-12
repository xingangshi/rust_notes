#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

mod other {
    #[get("/world")]
    pub fn world() -> &'static str{
        "Hello world"
    }
}

#[get("/hello/<name>")]
pub fn hello(name: &RawStr) -> String{
    format!("Hello, {}!", name.as_str())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, First rocket website"
}

use other::world;

fn main() {
    rocket::ignite().mount("/hello", routes![index, hello, other::world]).launch();
}
