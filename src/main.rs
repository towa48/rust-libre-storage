#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate dotenv;

//pub mod models;
mod schema;
mod auth;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();

    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/dist/browser")))
        .mount("/auth", routes![auth::token])
        .launch();
}