#![feature(conservative_impl_trait)]
#![feature(custom_derive)]
#![feature(proc_macro)]

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
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
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

/// TRansaction related additional data
#[derive(Serialize, Deserialize)]
struct TransactionDetails {
    n_inputs : usize,
    n_outputs : usize,
    total_input_value : f64,
    total_output_value : f64,
    miner_fee : f64
}

/// Block related additional data
#[derive(Serialize, Deserialize)]
struct BlockDetails {
    tx_details : Vec<TransactionDetails>,
    miner_reward : f64,
    miner_fees : f64,
    miner_entity : String,
    avg_tx_amount : f64,
    max_tx_amount : f64,
    min_tx_amount : f64
}
//serde_struct_impl!(BlockDetails, miner_reward, miner_fees, miner_entity, avg_tx_amount);

/// fetch block data from bitcoin node using API
fn fetch_block( node_ip : &str, block_hash : &str ) -> impl Future<Item = Block, Error = ()>
{
    let mut request_url = "http://".to_string();
    request_url.push_str(node_ip);
    request_url.push_str(":8332/rest/block/");
    request_url.push_str(&block_hash.to_string());
    request_url.push_str(".bin");

    let mut buf = Vec::new();
    { // other way libcurl
        let mut handle = Easy::new();
        handle.url(&request_url).unwrap();
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
           &buf.extend_from_slice(data);
           Ok(data.len())
        }).unwrap();
        print!("Fetching block ... ", );
        transfer.perform().unwrap();
        println!("Done");
    }
    let raw_block = buf;
    let decode: Result<Block, _> = deserialize(&raw_block[..]);
    let block = decode.unwrap();

    future::ok(block)
}

fn process_tx( tx : Transaction ) -> TransactionDetails {
    let mut tx_details = TransactionDetails {
        n_inputs : tx.input.len(),
        n_outputs : tx.output.len(),
        total_input_value : 0.0,
        total_output_value : 0.0,
        miner_fee : 0.0
    };

    // process inputs
    for txIn in tx.input {
        //println!("{:?}", txIn);
    }

    // process outputs
    for txOut in tx.output {
        //println!("{:?}", txOut);
        tx_details.total_output_value += txOut.value as f64;
    }

    tx_details // return
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
            (about: "Retrieves and parses block given it's hash. Performs analysis and outputs various details.")
            (@arg BLOCK_HASH: +required +takes_value "Block hash")
            (@arg node: -n +takes_value "Specifies IP address of bitcoin node which has to have REST API accessible.")
        ).get_matches();

    // extract arguments
    let block_hash = matches.value_of("BLOCK_HASH").unwrap();
    let node_ip = matches.value_of("node").unwrap_or("172.17.0.2");
    let network = Network::Bitcoin;
    let compressed = false;

    // get the block
    let block = lp.run(fetch_block( node_ip, block_hash)).unwrap();

    // process all block transactions and extract statistics of intrest
    let tx_count : usize = block.txdata.len();
    let mut block_details = BlockDetails {
        tx_details : Vec::with_capacity(tx_count),
        miner_reward : 0.0,
        miner_fees : 0.0,
        miner_entity : "Unknown".to_string(),
        avg_tx_amount : 0.0,
        max_tx_amount : 0.0,
        min_tx_amount : 0.0
    };

    for tx in &block.txdata[0..tx_count] {
        let tx_details = process_tx( tx.clone() );
        block_details.avg_tx_amount += tx_details.total_output_value;
        println!("{:?}", serde_json::to_string(&tx_details).unwrap());
        block_details.tx_details.push(tx_details);
    }

    assert!(tx_count != 0);
    block_details.avg_tx_amount /= tx_count as f64;

    //process_tx( block.txdata[0].clone() ); // Always coinbase transaction

    // output its hash and some basic details

    println!("Details for block with hash {:?}", block_hash);
    println!("Previous block hash value : {:?}", block.header.prev_blockhash);
    println!("Merkle root value : {:?}", block.header.merkle_root);
    println!("Transactions count : {:?}", tx_count);

    // output some statistics on block transactions
    println!("Transactions short summary:");
    println!("Average transfered value : {} BTC", block_details.avg_tx_amount / 10E8);
}
