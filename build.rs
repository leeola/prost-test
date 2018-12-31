extern crate prost_build;
extern crate prost_twirp;

fn main() {
  let mut conf = prost_build::Config::new();
  conf.service_generator(Box::new(prost_twirp::TwirpServiceGenerator::new()));
  conf.compile_protos(&["foo.proto"],
                      &["./proto"]).unwrap();
}

