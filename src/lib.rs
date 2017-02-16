// Rust Bitcoin tools library
// Written in 2016 by
//   Ilya Eriklintsev <erik.lite@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Bitcoin tools library
//!
//! This is a library that contains convenience tools for manipulating Bitcoin transactions
//! and interpreting other blockchain data.
//!
//! It is written entirely in Rust to illustrate the benefits of strong type
//! safety, including ownership and lifetime, for financial and/or cryptographic
//! software.
//!
//! Inspired by Andrew Apoelstra Rust Bitcoin library

#![crate_name = "bitcoin_tools"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

// Experimental features we need
#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#![feature(conservative_impl_trait)]

// Clippy whitelist
#![cfg_attr(feature = "clippy", allow(needless_range_loop))] // suggests making a big mess of array newtypes
#![cfg_attr(feature = "clippy", allow(extend_from_slice))]   // `extend_from_slice` only available since 1.6

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;

extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate futures;
extern crate env_logger;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;
extern crate curl;
extern crate tokio_core;
extern crate tokio_curl;

pub mod transactions;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
