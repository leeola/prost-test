#[macro_use]
extern crate prost_derive;

mod service {
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
}

fn main() {
    println!("Hello, world!");
}
