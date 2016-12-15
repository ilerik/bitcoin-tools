# bitcoin-tools
Useful bitcoin tools and code behind

## Building
1. Clone repository locally
~~~ sh
git clone https://github.com/ilerik/bitcoin-tools.git
cd bitcoin-tools
~~~
2. Install nightly rust compiler and cargo package manager
3. Build required tools from project folder using cargo
~~~ sh
cargo build --release --bin <tool-name>
~~~

## Tools
Below is the list of tools availible.

1. ### genaddress1
~~~sh
cargo build --release --bin genaddress
cd ./target/release/
./genaddress -p 1User -v -a 10000
~~~
