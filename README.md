# simple-grep
Simple grep implementation written in Rust

## Installation

Make sure cargo is installed. If not, run
```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```
or (insert favorite package manager here)
```
brew install cargo
```

To install simple grep, run
```
cargo install --git https://github.com/avanishd-3/simple-grep.git
```

Run
```
simple-grep --h
````
to see all available commands

## Building

You need [Rust](https://www.rust-lang.org/) to compile simple-grep.

Once installed do,
```
git clone https://github.com/avanishd-3/simple-grep.git
cd simple-grep
cargo build --release
./target/release/simple-grep
```

## Testing

Run
```
cargo test --all
```
in simple-grep root directory
