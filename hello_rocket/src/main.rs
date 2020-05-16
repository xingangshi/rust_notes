#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

use std::io::Cursor;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::{Header, ContentType, Method};

mod other {
    #[get("/world")]
    pub fn world() -> &'static str{
        "Hello world"
    }
}

// 动态路径
#[get("/hello/<name>/<age>/<cool>")]
pub fn hello(name: &RawStr, age: u8, cool: bool) -> String{
    if cool {
        format!("You are a cool {} year old, {} !", age, name)
    }
    else {
        format!("{}, we need talk about your name. ", name.as_str())
    }
}

#[get("/hello_1?<wave>&<name>")]
fn hello_1(wave: String, name: Option<String>) -> String {
    name.map(| name | format!("name :Hi, {}", name))
        .unwrap_or_else(|| "Hello!".into())
}


#[get("/")]
fn index() -> &'static str {
    "Hello, First rocket website"
}

//use other::world;


struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options ||
           response.content_type() == Some(ContentType::JSON) ||
           response.content_type() == Some(ContentType::Plain) {

            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PATCH, PUT, DELETE, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Origin, X-Auth-Token"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}


fn main() {
    rocket::ignite().attach(CORS()).mount("/", routes![index, hello, hello_1, other::world]).launch();

}
