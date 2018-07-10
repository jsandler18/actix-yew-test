extern crate actix_web;
extern crate actix;
extern crate env_logger;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_error;
extern crate dotenv;

pub mod db;
pub mod error;
pub mod schema;

use actix_web::{server, App, HttpRequest};
use actix_web::middleware::Logger;
use actix::prelude::*;
use dotenv::dotenv;
use error::{Error, Result};

struct State {
    db: Addr<db::DbExecutor>
}

fn index(_req: HttpRequest<State>) -> &'static str {
    "Hello, world"
}

fn main() -> Result {
    // Initialize Logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Initialize Database
    let _ = dotenv();
    let db_url = std::env::var("DATABASE_URL").map_err(|_| Error::DbUrlError)?;
    let db_addr = SyncArbiter::start(3, move || {
        db::DbExecutor::init(&db_url).unwrap()
    });

    // Initialize server
    server::new(move || App::with_state(State {db: db_addr.clone()})
        .middleware(Logger::default())
        .resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
    Ok(())
}
