# bitcoin-tools
Useful bitcoin tools and some rust code behind.

## Building
- Install nightly rust compiler (https://www.rust-lang.org/en-US/downloads.html) and if required cargo package manager as well (https://crates.io/install)
- Clone repository locally
``` sh
git clone https://github.com/ilerik/bitcoin-tools.git
cd bitcoin-tools
```
- Build required tools from project folder using cargo
``` sh
cargo build --release --bin <tool-name>
```

## Tools
Below is the list of tools available.

### 1. genaddress

Allows you to generate bitcoin address \ private key pair for Bitcoin main
blockchain. Also allows you to run brute force search for the address that
matches regular expression you specify with -p option.

~~~ sh
cargo build --release --bin genaddress
cd ./target/release/
./genaddress -p 1User -v -a 10000 # usage example search for 1User* address
~~~

