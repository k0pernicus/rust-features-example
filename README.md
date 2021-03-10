# rust-features-example

This repository illustrates the usage of the `features` feature in the Rust programming language, for one of my 
[blog posts](https://carette.xyz).

## Usage

This example implements two different "linguistic backends": english, and french.

You can use one of them using the `features` cargo argument:

* for English regional package: `cargo run --features english`,
* for French regional package: `cargo run --features french`.

The main binary to illustrate the example is a "guess the number" game.
