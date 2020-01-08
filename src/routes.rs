extern crate log;
extern crate rocket;

use rocket::response::status::BadRequest;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use crate::service::ReportService;

use self::rocket::State;
use rocket::Rocket;

use rocket::Data;
use std::io::Read;

#[derive(Deserialize)]
pub struct GetReport {
    template_name: String,
    user_params: JsonValue,
}

#[get("/hello")]
pub fn hello() -> &'static str{
    "Hello, World!"
}
// #[post("/hello",format="plain",data="<data>"]
 #[post("/echo", format = "plain", data = "<name>")]
pub fn say_hello(name: Data) -> String {
    let mut stream = name.open();
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).expect("data read to stream");
    buffer
}

#[post("/generate", format = "application/json", data = "<req>")]
pub fn generate_report(service: State<ReportService>, req: Json<GetReport>)
                       -> Result<NamedFile, BadRequest<String>> {
    let params = req.0.user_params;
    let report = service.render(&req.0.template_name, params);

    report
        .map_err(|e| BadRequest(Some(format!("Failed to generate report: {:?}", e))))
        .and_then(|path| NamedFile::open(path).map_err(|e| BadRequest(Some(e.to_string()))))
}

pub fn mount_routes(service: ReportService) -> Rocket {
    rocket::ignite()
        .manage(service)
        .mount("/api/v1",routes![generate_report],
        )
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello])
    .mount("/", routes![say_hello])
}