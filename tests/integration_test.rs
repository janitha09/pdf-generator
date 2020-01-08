extern crate pdf_generator;
// extern crate rocket;

use pdf_generator::service::ReportService;
use pdf_generator::routes::mount_routes;

use rocket::local::Client;
use rocket::http::Status;
use pdf_generator::routes::rocket;
use rocket::http::ContentType;
use pdf_generator::templates::TemplateEngine;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::path::Path;
extern crate env_logger;

#[test]
fn hello_world() {
    let rocket = rocket(); // rocket.ignite()
    assert_eq!(rocket.routes().count(), 2);
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
#[test]
fn echo_name() {
    let rocket = rocket();
    assert_eq!(rocket.routes().count(),2 );
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.post("/echo").body("Janitha").header(ContentType::Plain).dispatch();
    assert_eq!(response.status(),Status::Ok );
    assert_eq!(response.body_string(), Some("Janitha".into()));
}
// #[test]
// #[ignore]
// fn api_v1_generate() {
//     let report_service = ReportService::new();
//     let rocket = mount_routes(report_service.unwrap());
//     let client = Client::new(rocket).expect("valid rocket instance");
//     let mut response = client.post("/api/v1/generate").header(ContentType::JSON).body("").dispatch();
//     // assert_eq!(response.status(), Status::Ok );
//     // assert_eq!(response.body_string(), Some("Janitha".into()));
// }
#[test]
fn render() {
    let _work_dir = "target/work_dir".to_string();
    let handlebars = TemplateEngine::init_template_engine().expect("handle bars created");

    let template_engine = TemplateEngine::new().expect("template engine created");
    let html = template_engine.render("","");
    println!("{:?}",html);

    
    // handlebars.render
    // println!("{:?}",handlebars );
    // let template_engine = TemplateEngine { handlebars };
    // let report_service = ReportService { template_engine, work_dir };
}

#[test]
fn handlebars_helloworld() {
    let mut handlebars = Handlebars::new();
    let source = "hello {{world}}";
    handlebars.register_template_string("t1",source).expect("template registered");
    let mut data = BTreeMap::new();

    data.insert("world".to_string(), "世界!".to_string());
    assert_eq!(handlebars.render("t1", &data).unwrap(), "hello 世界!");
}
#[test]

fn handlebars_from_file(){
    let mut handlebars = Handlebars::new();
    let path = Path::new("./templates"); //relative to where you run from cargo test runs at the root
    handlebars.register_templates_directory(".html", path).expect("templates reqistered");
    assert_eq!(handlebars.get_templates().len(),1);
    // for key in handlebars.get_templates().keys() {
    //     println!("{}", key);
    // }
    let the_file = r#"{
        "template_name": "book-order-report",
        "user_params": {
          "customer_name": "Frank Smith",
          "address": "Address: Frankfurt am Main, Mainzer str. 100",
          "ordered_books": [
            {
              "book_name": "Getting Things Done: The Art of Stress-Free Productivity. Authors: David Allen",
              "amount": 9.51
            },
            {
              "book_name": "Funky Business - Talent Makes Capital Dance. Authors: Ridderstråle, Nordström",
              "amount": 14.99
            },
            {
              "book_name": "The Rust Programming Language (Manga Guide). Authors: Klabnik, Nichols",
              "amount": 23.99
            }
          ],
          "total": 48.49
        }
      }"#;

    let req: serde_json::Value = serde_json::from_str(the_file).expect("JSON was not well-formatted");
    // println!("{:?}",req["user_params"]);
    assert_eq!(req["template_name"],"book-order-report");
    assert_eq!(req["user_params"]["customer_name"],"Frank Smith");
    // let params = req.user_params;
    let template_name = req["template_name"].as_str().expect("get string");
    // assert_eq!(template_name,"book-order-report");
    let html = handlebars.render(&template_name,&req["user_params"]).expect("html rendered");
    println!("{:?}",html);
    // assert_eq!(handlebars.get_templates.keys())
}