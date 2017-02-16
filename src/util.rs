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

//! # Utility functions mostly for interaction with REST services
//!
//! Functions needed by all parts of the Bitcoin tools library

#![feature(conservative_impl_trait)]
#![feature(custom_derive)]

use secp256k1::{Secp256k1, ContextFlag};
use secp256k1::key::{PublicKey, SecretKey};
use bitcoin::util::address::{Address, Privkey};
use bitcoin::network::constants::Network;
use bitcoin::util::base58::ToBase58;
use bitcoin::blockdata::block::Block;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::blockdata::transaction::TxIn;
use bitcoin::blockdata::transaction::TxOut;
use bitcoin::network::serialize::{deserialize, serialize};
use rustc_serialize::hex::FromHex;
use rand::{thread_rng};
use curl::easy::Easy;
use futures::Future;
use futures::future;
use tokio_core::reactor::Core;
use tokio_curl::Session;


fn send_request( request_url : &str ) -> impl Future<Item = Vec<u8>, Error = ()>
{
    let mut buf = Vec::new();
    { // other way libcurl
        let mut handle = Easy::new();
        handle.url(&request_url).unwrap();
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
           &buf.extend_from_slice(data);
           Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let response_data = buf;
    future::ok(response_data)
}
