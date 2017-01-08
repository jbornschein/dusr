#![feature(plugin, proc_macro, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate maplit;

mod nsupdate;
mod xforwardedfor;

extern crate rocket_contrib;
extern crate rocket;
// extern crate serde_json;
// #[macro_use] extern crate serde_derive;

// use std::collections::HashMap;
use std::net::IpAddr;

// use rocket::http::uri::URI;
// use rocket::response::Redirect;
use rocket_contrib::Template;

// use nsupdate::Updater;
use xforwardedfor::XForwardedFor;

/////////////////////////////////////////////////////////////////////////////

#[derive(Debug, FromForm)]
struct UpdateArgs {
    token: Option<String>,
    ip: Option<IpAddr>
}

#[derive(Debug, FromForm)]
struct AdminArgs {
    token: Option<String>,
}

#[get("/")]
fn index() -> Template {
    let context = hashmap!{
        "name" => "test.capsec.org"
    };
    Template::render("index", &context)
}

#[get("/admin?<args>")]
fn admin(args: UpdateArgs) -> Template {
    let context = hashmap!{
        "name" => "test",
    };
    Template::render("admin", &context)
}

#[get("/update/<name>?<args>")]
fn update(name: &str, args: UpdateArgs, xforwarded: XForwardedFor) -> Template {
    if let Some(ref token) = args.token {
        println!("Arguments: {:?}", args);

        // println!("Request: {:?}", req);
        // let  args.ip.parse()

        let context = hashmap!{
            "name" => name,
        };
        Template::render("update", &context)
    } else {
        let context = hashmap!{
            "name" => name,
        };
        Template::render("denied", &context)
    }
}

#[error(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("<p>Sorry, but '{}' is not a valid path!</p>", req.uri())
}

fn main() {
    // Create DNS update service


    // Start webservice
    rocket::ignite()
        .mount("/", routes![index, update])
        .catch(errors![not_found])
        .launch();
}
