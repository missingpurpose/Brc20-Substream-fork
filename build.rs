extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["proto/cap_table.proto"],
        &["proto"],
    ).unwrap();
}