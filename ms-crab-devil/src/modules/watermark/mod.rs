pub mod interface;
pub mod module;

mod application;
mod domain;
mod infrastructure;

pub use application::watermark_service_handler;
