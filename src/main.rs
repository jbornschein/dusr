#![feature(plugin, proc_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
// extern crate serde_json;
// #[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use rocket::{Request};
use rocket::response::Redirect;
use rocket_contrib::Template;

mod nsupdate;

#[derive(Debug, FromForm)]
struct UpdateArgs {
    token: Option<String>,
    ip: Option<String>
}

// #[derive(Serialise)]
// struct IndexContext {
//     domain: String
// }


#[get("/")]
fn index() -> Template {
    // format!("index!")
    let mut context = HashMap::new();
    context.insert("name", "test.capsec.org");

    Template::render("index", &context)
}

#[get("/update/<name>?<args>")]
fn update(name: &str, args: UpdateArgs) -> String {
    match args.token {
        None => {
            format!("Missing token!")
        }
        Some(ref token) => {
            println!("Arguments:");
            println!("{:?}", args);
            format!("Name: {}", name)
        }
    }
}

fn main() {
    // match nsupdate::update_dns("hallo", "du") {
    //     Ok(_) => println!("update_dns: Ok!"),
    //     Err(why) => panic!("update_dns failed: {:?}", why)
    // }

    // Start webservice
    rocket::ignite()
        .mount("/", routes![index, update])
        .launch();
}
