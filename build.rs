use anyhow::{Ok, Result};
use prost_build::Config;
use std::env;

fn main() -> Result<(), anyhow::Error> {
    // Set the PROTOC environment variable to the path of the protoc binary
    env::set_var("PROTOC", "C:\\protoc\\bin\\protoc.exe");

    let proto_files = [
        "proto/bitcoin.proto",
        "proto/cap_table.proto",
    ];

    let proto_include = ["proto"];

    // Configure prost-build
    let mut config = Config::new();
    config.out_dir("src/pb"); // Ensure the output directory is correct

    // Compile the Protobuf files
    config.compile_protos(&proto_files, &proto_include)?;

    // Re-run the build script if any of the proto files change
    for proto in &proto_files {
        println!("cargo:rerun-if-changed={}", proto);
    }

    Ok(())
}