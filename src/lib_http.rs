use rocket::http::{Status, ContentType};
use rocket::request::{Request};
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use std::fmt::Debug;
use serde::Serialize;

pub struct ApiResponse<T>
    where T: Serialize
{
    result: Option<T>,
    error: Option<ApiError>,
    status: Status,
}

impl<T> ApiResponse<T>
    where T: Serialize
{
    pub fn ok(value: T) -> Self {
        ApiResponse { result: Some(value), status: Status::Ok, error: None }
    }
    pub fn warn(value: T, status: Status) -> Self {
        ApiResponse { result: Some(value), status: status, error: None }
    }
    pub fn err(error: &'static str, error_description: &'static str, status: Status) -> Self {
        ApiResponse { result: None, status: status, error: Some(ApiError::new(error, error_description)) }
    }
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

impl<'r, T> Responder<'r> for ApiResponse<T>
    where T: Serialize
{
    fn respond_to(self, req: &Request) -> rocket::response::Result<'r> {
        let body = if self.is_error() {
            JsonValue(json!(self.error.unwrap()))
        } else {
            JsonValue(json!(self.result.unwrap()))
        };
        Response::build_from(body.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize, Debug)]
struct ApiError {
    pub error: &'static str,
    pub error_description: &'static str,
}

impl ApiError {
    pub fn new(error: &'static str, error_description: &'static str) -> ApiError {
        ApiError { error: error, error_description: error_description }
    }
}

//#[catch(404)]
//fn not_found(request: &Request<'_>) -> Html<String> {
//    let html = match request.format() {
//        Some(ref mt) if !mt.is_json() && !mt.is_plain() => {
//            format!("<p>'{}' requests are not supported.</p>", mt)
//        }
//        _ => format!("<p>Sorry, '{}' is an invalid path! Try \
//                 /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
//                 request.uri())
//    };
//
//    Html(html)
//}