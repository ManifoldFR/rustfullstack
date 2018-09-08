extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
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


fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let log = warp::log("info");

    let addr: SocketAddrV4 = "127.0.0.1:3030".parse()
        .expect("Could not create IP.");

    let greet = warp::path("people")
        .and(warp::path::param::<String>())
        .map(|username| {
            format!("{} says hello!", username)
        });

    let users = warp::path("users")
        .and(
            warp::get2()
            .map(|| {
                let conn = establish_connection();
                let res = models::User::get_list(&conn);
                warp::reply::json(&res)
            })
        ).or(
            warp::post2()
            .and(warp::body::json())
            .map(|new_user: models::NewUser| {
                info!("Adding user {:?}", new_user);
                let conn = establish_connection();
                let res = models::User::create(&conn, new_user);
                warp::reply::json(&res)
            })
        );

    let routes = warp::any()
        .and(greet)
        .or(users)
        .with(log)
        .with(warp::reply::with::header("Access-Control-Allow-Origin", "*"));

    let server = warp::serve(routes);
    server.run(addr);
}
