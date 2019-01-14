extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/client",
        input: &["src/client/schema.proto"],
        includes: &["src/client"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}