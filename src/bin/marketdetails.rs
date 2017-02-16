#![feature(conservative_impl_trait)]
#![feature(custom_derive)]
#![feature(proc_macro)]

extern crate serde;
#[macro_use] extern crate serde_json;
extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
extern crate curl;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;
extern crate env_logger;
extern crate crypto;
#[macro_use] extern crate clap;

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
use regex::Regex;
use curl::easy::Easy;
use futures::Future;
use futures::future;
use tokio_core::reactor::Core;
use tokio_curl::Session;
use serde_json::Value;
use std::io::Read;
use crypto::md5::Md5;
use crypto::digest::Digest;

///
// url: /api/user/token
// method: POST
// [login] - required - user email address
// [password] - optional - hashed user password
// [hash] - optional - random hash request
// [2facode] - optional - 2FA code
// [device] - optional - user device id

/// Acquire random hash for authentification
fn acquire_random_hash( username : String ) -> impl Future<Item = String, Error = ()>
{
    let request_url = "https://bit.ac/api/user/token".to_string();
    let mut data_string = "login=".to_string();
    data_string.push_str(&username);
    data_string.push_str("&hash=1");
    let mut request_data = data_string.as_bytes();

    println!("{:?}", data_string);

    let mut handle = Easy::new();
    handle.url(&request_url).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(request_data.len() as u64).unwrap();

    let mut response_data = Vec::new();

    {
        let mut transfer = handle.transfer();
        transfer.read_function(|buf| {
            Ok(request_data.read(buf).unwrap_or(0))
        }).unwrap();

        transfer.write_function(|data| {
            &response_data.extend_from_slice(data);
            Ok(response_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    //Check for errors
    let v : Value = serde_json::from_slice(&response_data[..]).unwrap();
    let random_hash = v["hash"];

    future::ok(v.get("hash").as_string())
}

/// Acquire authentification token
fn acquire_token( username : String, one_time_password : String ) -> impl Future<Item = String, Error = ()>
{
    let request_url = "https://bit.ac/api/user/token".to_string();
    let mut data_string = "login=".to_string();
    data_string.push_str(&username);
    data_string.push_str("&password=");
    data_string.push_str(&one_time_password);
    let mut request_data = data_string.as_bytes();

    let mut handle = Easy::new();
    handle.url(&request_url).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(request_data.len() as u64).unwrap();

    let mut response_data = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.read_function(|buf| {
            Ok(request_data.read(buf).unwrap_or(0))
        }).unwrap();

        transfer.write_function(|data| {
            &response_data.extend_from_slice(data);
            Ok(response_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    //Check for errors
    let v : Value = serde_json::from_slice(&response_data[..]).unwrap();

    future::ok(v.to_string())
}

fn main() {
    // initialize logger and event loop
    env_logger::init().unwrap();
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());

    // command line arguments and help
    let matches = clap_app!(blockdetails =>
            (version: "1.0")
            (author: "Ilya E. <erik.lite@gmail.com>")
            (about: "Current cryptocurrency market reprot based on external exchange API provided data.")
            (@arg login: -u +takes_value "Login for the selected service")
            (@arg password: -u +takes_value "Password for the selected service")
        ).get_matches();

    // extract parameters
    let user_name = matches.value_of("login").unwrap();
    let user_password = matches.value_of("password").unwrap();;

    // authentificate
    let auth_task = acquire_random_hash( user_name.clone() )
    .and_then(|hash| {
        println!("random hash \"{}\"", hash);
        let mut hasher = Md5::new();
        hasher.input_str(&user_password);
        let hashed_password = hasher.result_str();
        println!("hashed user password : \"{}\"", hashed_password);
        let mut password = hash;
        password.push_str(&hashed_password);
        println!("unhashed password : \"{}\"", password);
        hasher.reset();
        hasher.input_str(&password);
        let password = hasher.result_str();
        println!("resulting password : \"{}\"", password);
        future::ok(password)
    })
    .and_then( |otp| {
        acquire_token( user_name.clone(), otp )
    });

    let response_data = lp.run( auth_task ).unwrap();
    println!("{:?}", response_data);
}
