#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

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

#[get("/hello_1?wave&<name>")]
fn hello_1(name: Option<String>) -> String {
    name.map(| name | format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}



#[get("/")]
fn index() -> &'static str {
    "Hello, First rocket website"
}

//use other::world;

fn main() {
    rocket::ignite().mount("/", routes![index, hello, hello_1, other::world]).launch();
}
