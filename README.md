# bitcoin-tools
Useful bitcoin tools and code behind

## Building
1. Clone repository locally
'''shell
git clone 
'''
Below is the list of tools availible.

### genaddress1
'''shell
cargo build --release --bin genaddress
cd ./target/release/
./genaddress -p 1User -v -a 10000
'''
2. Install nightly rust compiler and cargo package manager
3. Build required tools from project folder using cargo
'''shell
cargo build --release --bin <tool-name>
'''

## Tools
Below is the list of tools availible.

### genaddress1
'''shell
cargo build --release --bin genaddress
cd ./target/release/
./genaddress -p 1User -v -a 10000
'''
