#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate chrono;
extern crate sha2;
extern crate sodiumoxide;

pub mod auth;
mod event;
pub mod ser;
pub mod server_keys;
pub mod state_map;

pub use event::{Event, EventBase};

use failure::Error;


pub fn get_domain_from_id(string: &str) -> Result<&str, Error> {
    string
        .splitn(2, ":")
        .nth(1)
        .ok_or_else(|| format_err!("invalid ID"))
}
