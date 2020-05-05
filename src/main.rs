#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;
extern crate dotenv;
extern crate pbkdf2;
extern crate base64;
extern crate rand;
extern crate hmac;
extern crate sha2;

mod app_state;
mod crypto;
mod lib_http;
mod models;
mod providers;
mod routes;
mod schema;

use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
use std::env;
use crate::app_state::AppState;
use crate::providers::DbConnection;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DbConnection::fairing())
        .manage(AppState::new())
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/dist/browser")))
        .mount("/auth", routes![routes::auth::token])
        .mount("/webdav", routes![routes::webdav::list])
        .launch();
}