#![feature(plugin, proc_macro, custom_derive)]
#![plugin(rocket_codegen)]


extern crate rocket;

#[derive(Debug, FromForm)]
struct UpdateArgs {
    token: Option<String>,
    ip: Option<String>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
    rocket::ignite()
        .mount("/", routes![index, update])
        .launch();
}
