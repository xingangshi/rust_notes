#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use std::io::BufRead;
use std::str::from_utf8;
use std::io::Cursor;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::{Header, ContentType, Method, Status};
use rocket::response::status::Custom;
use rocket::Data;
use multipart::server::Multipart;

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


#[post("/hello_1", data = "<data>")]
fn hello_1(cont_type: &ContentType, data: Data) -> Result<Custom<String>, Custom<String>> {
    let (_, boundary) = cont_type.params()
                                 .find(|&(k, _)| k == "boundary")
                                 .ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        )
    )?;

    let mut d = std::vec::Vec::new();
    data.stream_to(&mut d).expect("Unable to read");
    let mut mp = Multipart::with_body(Cursor::new(d), boundary);

    let mut name = String::new();
    let mut age:i32 = 0;

    mp.foreach_entry(|mut entry| {
        if *entry.headers.name == *"name" {
            let file_name_vec = entry.data.fill_buf().unwrap().to_owned();
            name = from_utf8(&file_name_vec).unwrap().to_string()
        } else if *entry.headers.name == *"age" {
            let age_temp = entry.data.fill_buf().unwrap().to_owned();
            age = from_utf8(&age_temp).unwrap().to_string().parse().unwrap();
        }
    }).expect("Unable to iterate");

    age = age + 1;
    print!("Hello, {}, your age {}", name, age);
    let result = format!("{{\"name\":\"{}\",\"age\":\"{}\"}}", name, age);
    return Ok(
        Custom(Status::Ok, result)
    );

    // name.map(| name | format!("name :Hi, {}", name))
    //     .unwrap_or_else(|| "Hello!".into())
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
            response.set_header(Header::new("Access-Control-Allow-Headers", 
                                            "Content-Type, Authorization, Access-Control-Allow-Methods, Access-Control-Request-Headers, Origin, X-Auth-Token"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}


fn main() {
    rocket::ignite().attach(CORS()).mount("/",
      routes![index,
        hello,
        hello_1,
        other::world]
    ).launch();
}
