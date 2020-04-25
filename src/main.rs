#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate chrono;
extern crate rocket_contrib;
extern crate dotenv;
extern crate pbkdf2;
extern crate base64;
extern crate rand;
extern crate hmac;
extern crate sha2;

mod schema;
mod routes;
pub mod lib_http;
pub mod crypto;
pub mod providers;

use rocket_contrib::serve::StaticFiles;
use std::env;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/dist/browser")))
        .mount("/auth", routes![routes::auth::token])
        .launch();
}