use crate::api::Status;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

pub fn serve(req: &rouille::Request) -> rouille::Response {
    match rouille::input::json_input::<Request>(req) {
        Ok(_) => {}
        _ => return rouille::Response::from(Status::BadRequest),
    };

    rouille::Response::json(&Response {
        message: String::from("Pokemon created!"),
    })
}