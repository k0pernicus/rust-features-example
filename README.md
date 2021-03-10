# rust-features-example

This repository illustrates the usage of the `features` feature in the Rust programming language, for one of my 
[blog posts](https://carette.xyz).

## Usage

This example implements two different "linguistic backends": english, and french.

You can use one of them using the `features` cargo argument:

`cargo run --features english` for English,

`cargo run --features french` for French.
