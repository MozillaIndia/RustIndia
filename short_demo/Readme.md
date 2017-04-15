# Getting started with Rust - Introduction

*Reference link* :- https://doc.rust-lang.org/book

## To install in linux

~~~~
curl -sSf https://static.rust-lang.org/rustup.sh | sh
~~~~

## To check the version

~~~~
rustc --verison
~~~~

## To uninstall rust 

~~~~
sudo /usr/local/lib/rustlib/uninstall.sh
~~~~

## Running the first hello world problem

* basic code
~~~~
fn main() {
    println!("Hello, world!");
}
~~~~

* How to compile and run
~~~~
$ rustc main.rs
$ ./main
Hello, world!
~~~~

## Cargo - Rust’s build system and package manager

* check if Cargo is installed

~~~~
cargo --version
~~~~

* Cargo expects your code to be inside src dir
* Cargo.toml is a configuration file

* Building and running a Cargo project

~~~~
$ cargo build
   Compiling hello_world v0.0.1 (file:///home/yourname/projects/hello_world)
$ ./target/debug/hello_world
Hello, world!

//or

$ cargo run
     Running `target/debug/hello_world`
Hello, world!
~~~~

* Cargo.lock - tracks the dependencies of the application

* Makes a complete Cargo project to hack on
~~~~
cargo new hello_world --bin
~~~~



