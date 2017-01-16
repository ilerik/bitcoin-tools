extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
#[macro_use] extern crate clap;

use secp256k1::{Secp256k1, ContextFlag};
use secp256k1::key::{PublicKey, SecretKey};
use bitcoin::util::address::{Address, Privkey};
use bitcoin::network::constants::Network;
use bitcoin::util::base58::ToBase58;
use bitcoin::blockdata::block::Block;
use bitcoin::network::serialize::{deserialize, serialize};
use rustc_serialize::hex::FromHex;
use rand::{thread_rng};
use regex::Regex;

/// fetch block data from bitcoin node using API
fn fetch_block( block_hash : &str ) -> Block {
    // connect to node
    let raw_block = "010000004ddccd549d28f385ab457e98d1b11ce80bfea2c5ab93015ade4973e400000000bf4473e53794beae34e64fccc471dace6ae544180816f89591894e0f417a914cd74d6e49ffff001d323b3a7b0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d026e04ffffffff0100f2052a0100000043410446ef0102d1ec5240f0d061a4246c1bdef63fc3dbab7733052fbbf0ecd8f41fc26bf049ebb4f9527f374280259e7cfa99c48b0e3f39c51347a19a5819651503a5ac00000000010000000321f75f3139a013f50f315b23b0c9a2b6eac31e2bec98e5891c924664889942260000000049483045022100cb2c6b346a978ab8c61b18b5e9397755cbd17d6eb2fe0083ef32e067fa6c785a02206ce44e613f31d9a6b0517e46f3db1576e9812cc98d159bfdaf759a5014081b5c01ffffffff79cda0945903627c3da1f85fc95d0b8ee3e76ae0cfdc9a65d09744b1f8fc85430000000049483045022047957cdd957cfd0becd642f6b84d82f49b6cb4c51a91f49246908af7c3cfdf4a022100e96b46621f1bffcf5ea5982f88cef651e9354f5791602369bf5a82a6cd61a62501fffffffffe09f5fe3ffbf5ee97a54eb5e5069e9da6b4856ee86fc52938c2f979b0f38e82000000004847304402204165be9a4cbab8049e1af9723b96199bfd3e85f44c6b4c0177e3962686b26073022028f638da23fc003760861ad481ead4099312c60030d4cb57820ce4d33812a5ce01ffffffff01009d966b01000000434104ea1feff861b51fe3f5f8a3b12d0f4712db80e919548a80839fc47c6a21e66d957e9c5d8cd108c7a2d2324bad71f9904ac0ae7336507d785b17a2c115e427a32fac00000000".from_hex().unwrap();

    // deserialize block data
    let decode: Result<Block, _> = deserialize(&raw_block);
    let real_decode = decode.unwrap();
    return real_decode;
}

fn main() {
    // command line arguments and help
    let matches = clap_app!(blockdetails =>
            (version: "1.0")
            (author: "Ilya E. <erik.lite@gmail.com>")
            (about: "Retrieves and parses block given it's hash. Performs analysis and outputs various details.")
            (@arg BLOCK_HASH: +required +takes_value "Block hash")
        ).get_matches();

    // extract arguments
    let block_hash = matches.value_of("BLOCK_HASH").unwrap();
    let network = Network::Bitcoin;
    let compressed = false;
    let node_addr : String = "172.17.0.2:8332".parse().unwrap();

    // get the block
    let block = fetch_block(block_hash);

    // output its hash
    println!("{:?}", block.header.merkle_root);
}
