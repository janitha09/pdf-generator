#![feature(proc_macro_hygiene, decl_macro)]
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
// extern crate pdf_generator;

use dotenv::dotenv;
use env_logger::{Builder, Target};
use std::env;
use pdf_generator::service::ReportService;
use pdf_generator::routes::mount_routes;
// use pdf_generator::routes::rocket;



// #[macro_use] extern crate rocket;
// use rocket::Rocket;

// #[get("/hello")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

// fn main() {
//     rocket::ignite().mount("/", routes![hello]).launch();
// }

fn main() {
    dotenv().ok();
    init_logger();
    info!("Starting pdf-generator...");

    match ReportService::new() {
        Ok(s) => {
            let error = mount_routes(s).launch();
            drop(error);
        }
        Err(e) => {
            error!("Failed to start pdf-generator service, error: {:?}", e);
            panic!(e)
        }
    }
}

fn init_logger() {
    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    env::var("RUST_LOG").iter().for_each(|s| { builder.parse(s.as_str()); });
    builder.init();
}
// #[cfg(test)]
// mod test {
//     // use super::rocket;
//     use rocket::local::Client;
//     use rocket::http::Status;
//     use pdf_generator::routes::rocket;

//     #[test]
//     fn hello_world() {
        
//         let rocket = rocket(); // rocket.ignite()
//         assert_eq!(rocket.routes().count(), 1);
//         let client = Client::new(rocket).expect("valid rocket instance");
//         let mut response = client.get("/hello").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.body_string(), Some("Hello, World!".into()));
//     }
// }