
//
// Pull in all external dependencies
//
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate sodiumoxide;

#[macro_use] extern crate serde_derive;

extern crate indy;                      // lib-sdk project



// define our crate by defining the modules in the project
#[allow(unused_variables)]
pub mod api;
pub mod utils;
pub mod logic;
pub mod libraries;