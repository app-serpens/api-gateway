use std::env;

use crate::app::api::auth::login::login;
use actix_web::{web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use dotenv::dotenv;

extern crate actix;
extern crate actix_web;
extern crate r2d2;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate config;

pub mod app;
// pub mod schema;
pub mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");

    let host: String = env::var("API_HOST").expect("API host not set");
    let port: u16 = env::var("API_PORT")
        .expect("API port not set")
        .parse()
        .unwrap();

    let pool = Data::new(crate::app::db::get_pool());
    println!("Listening to requests at {}:{}...", host, port);
    // let db_address = SyncArbiter::start(5, move || app::db::DbActor(pool.clone()));

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(auth::token::validator);
        App::new()
            .app_data(pool.clone())
            .configure(app::init::initialize)
            .service(login)
    })
    .bind((host, port))?
    .run()
    .await
}
