extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate warp;
extern crate pretty_env_logger;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use warp::Filter;
use std::net::SocketAddrV4;

pub fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

#[derive(Serialize)]
struct TestStruct {
    username: String
}


fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let addr: SocketAddrV4 = "127.0.0.1:3030".parse()
        .expect("Could not create IP.");

    let hello = warp::path("people")
        .and(warp::path::param::<String>())
        .map(|username| {
            warp::reply::json(&TestStruct {username})
        });

    let server = warp::serve(hello);
    server.run(addr);
}
