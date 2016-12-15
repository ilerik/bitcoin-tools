extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate regex;
#[macro_use] extern crate clap;

use secp256k1::{Secp256k1, ContextFlag};
use secp256k1::key::{PublicKey, SecretKey};
use bitcoin::util::address::{Address, Privkey};
use bitcoin::network::constants::Network;
use bitcoin::util::base58::ToBase58;
use rand::{thread_rng};
use regex::Regex;

fn main() {
    // command line arguments and help
    let matches = clap_app!(genaddress =>
            (version: "1.0")
            (author: "Ilya E. <erik.lite@gmail.com>, Sergei K. <sergant_chern@mail.ru")
            (about: "Generates random Bitcoin address (base58 encoded) and corresponding private key (base58 encoded)")
            (@arg attempts: -a +takes_value "Sets the maximum number of keypairs to be generated during search")
            (@arg pattern: -p +takes_value "Specifies regular expression to be matched against address string")
            (@arg truncate: -t +takes_value "Truncate address string to given lenght and match only first symbols")
            (@arg verbose: -v --verbose "Outputs generated addresses during search")
        ).get_matches();

    // extract arguments
    let attempts: i64 = matches.value_of("attempts").unwrap_or("0").parse().unwrap_or(0);
    let truncate: usize = matches.value_of("truncate").unwrap_or("0").parse().unwrap_or(0);
    let pattern = matches.value_of("pattern").unwrap_or(r"*");
    let is_verbose = matches.is_present("verbose");
    let re = Regex::new(pattern).unwrap();
    let network = Network::Bitcoin;
    let compressed = false;

    // let's generate private\public keypairs until we find a match
    println!("Searching for address that matches the following pattern: {}", pattern);
    let secp = Secp256k1::with_caps(ContextFlag::SignOnly);

    // infinite loop
    let mut i : i64 = 0; // number of attempts made so far
    loop {
        // generate keypair from secp256k1 elliptic curve
        let sk = SecretKey::new(&secp, &mut thread_rng());
        let pk = PublicKey::from_secret_key(&secp, &sk).expect("Failed to create public key");

        // convert public key to Bitcoin address and private key to base58
        let address = Address::from_key(network, &pk, compressed);
        let privkey = Privkey::from_key(network, sk, compressed);
        let mut address_base58str = address.to_base58check();

        //Output number of attempts and addresses generated in verbose mode
        if is_verbose { println!("{} : {:?}", i, address_base58str) };

        // Search for desired pattern and truncate if necessary
        if (truncate != 0) { address_base58str.truncate(truncate) };
        if re.is_match(&address_base58str) {
            println!("Match was found for address {} and private key {}",
             address.to_base58check(),
             privkey.to_base58check());
            break;
        }

        // Maximum number of attempts check
        i = i + 1;
        if i == attempts {
            // Unsuccessfull search
            println!("Match was not found! Try again later.");
            break ;
        };
    }
}
