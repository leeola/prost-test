extern crate prost_build;

fn main() {
  let mut conf = prost_build::Config::new();
  conf.compile_protos(&["foo.proto"],
                      &["./proto"]).unwrap();
}

