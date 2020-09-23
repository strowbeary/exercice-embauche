#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate serde;

mod helpers;
mod routes;

use routes::*;

use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;

use db_connector::SQLiteAdapter;

pub fn start_api(sqlite_adapter: SQLiteAdapter) {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080", "http://127.0.0.1:8080"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .unwrap();
    rocket::ignite()
        .manage(sqlite_adapter)
        .mount(
            "/",
            routes![
                    get_pharmacies
                ],
        )
        .attach(cors)
        .launch();
}