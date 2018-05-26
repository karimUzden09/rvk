//! # Overview
//! This is a crate for accessing VK API.
//!
//! All of the API [methods](https://vk.com/dev/methods) are located in the `methods`
//! [module](#modules) of this crate (in the corresponding submodules).
//!
//! # Example
//! ```no_run
//! extern crate rvk;
//! use rvk::{APIClient, Params, methods::*};
//!
//! fn main() {
//!     let mut api = APIClient::new("your_access_token").unwrap(); // Create an API Client
//!
//!     let mut params = Params::new(); // Create a HashMap to store parameters
//!     params.insert("user_ids", "1");
//!
//!     let work = users::get(&api, params, |res| match res {
//!         Ok(v) => { // If the API returned a response, you get `serde_json::Value` here
//!
//!             // In this example, `v` corresponds to this JSON:
//!             // [
//!             //   {
//!             //     "id": 1,
//!             //     "first_name": "Pavel",
//!             //     "last_name": "Durov"
//!             //   }
//!             // ]
//!
//!             let user = v.as_array().unwrap().get(0).unwrap();
//!
//!             let first_name = user.get("first_name").unwrap().as_str().unwrap();
//!             let last_name = user.get("last_name").unwrap().as_str().unwrap();
//!             let id = user.get("id").unwrap().as_u64().unwrap();
//!
//!             println!("Success: User #{} is {} {}.", id, first_name, last_name);
//!         }
//!         Err(e) => println!("Error {}: {}", e.code(), e.msg()),
//!     }); // This returns a Future
//!
//!     api.run(work); // Do not forget to run the Future to make it actually do something!
//! }
//! ```

extern crate futures;
extern crate tokio_core;

extern crate hyper;
extern crate hyper_tls;

extern crate url;

extern crate serde_json;

pub mod client;
pub mod methods;

pub use client::APIClient;

/// A HashMap which contains method parameters
pub type Params<'a> = std::collections::HashMap<&'a str, &'a str>;
