extern crate futures;
extern crate hyper;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate prost_twirp;

mod service {
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
}

fn main() {
    println!("Hello, world!");
}
