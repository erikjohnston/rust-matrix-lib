#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate base64;
#[macro_use]
extern crate failure;

pub mod auth;
mod event;
mod state_map;

pub use event::Event;