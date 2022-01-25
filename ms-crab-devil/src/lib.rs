#![feature(async_closure)]
#![feature(future_join)]
#![feature(future_poll_fn)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;

pub mod modules;
mod schema;

use dotenv::dotenv;

pub fn setup() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}
