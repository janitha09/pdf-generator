extern crate pdf_generator;
// extern crate rocket;

use pdf_generator::service::ReportService;
use pdf_generator::routes::mount_routes;

use rocket::local::Client;
use rocket::http::Status;
use pdf_generator::routes::rocket;

// #[test]
fn test_something(){
    assert_eq!(0,0);
}
// #[test]
fn test_request_to_root(){
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

// #[test]
fn test_request_generate(){
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");
    let response = client.get("/generate").dispatch();
    assert_eq!(response.status(), Status::Ok);
    // println!("{:#?}",rocket );

}

// #[test]
fn test_get_hello(){
    // is this a singleton?
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");
    let response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
}


#[test]
fn hello_world() {
    
    let rocket = rocket(); // rocket.ignite()
    assert_eq!(rocket.routes().count(), 1);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, World!".into()));
}
#[test]
fn count_mounted_routes() {
    let result = ReportService::new();
    let rocket = mount_routes(result.unwrap());
    assert_eq!(rocket.routes().count(),1);
}
