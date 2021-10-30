use crate::api::Status;
use serde::{Deserialize, Serialize};
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

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

pub fn serve(_repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    match rouille::input::json_input::<Request>(req) {
        Ok(_) => {}
        _ => return rouille::Response::from(Status::BadRequest),
    };

    rouille::Response::json(&Response {
        message: String::from("Pokemon created!"),
    })
}