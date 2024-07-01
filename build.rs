<<<<<<< HEAD
extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["proto/cap_table.proto"],
        &["proto"],
    ).unwrap();
=======
use anyhow::{Ok, Result};
use std::process::Command;

fn main() -> Result<(), anyhow::Error> {
    let proto_files = [
        "proto/bitcoin.proto",
        // Add other proto files if needed
    ];

    let _proto_include = ["proto"]; // Prefix with underscore to avoid warning

    // Use npx buf to compile protobuf files
    let output = Command::new("cmd")
        .args(&["/C", "npx buf generate --template buf.gen.yaml"])
        .output()
        .expect("Failed to execute npx buf");

    if !output.status.success() {
        panic!(
            "npx buf generate failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Re-run the build script if any of the proto files change
    for proto in &proto_files {
        println!("cargo:rerun-if-changed={}", proto);
    }

    Ok(())
>>>>>>> 0e6ab0d1b3e5365ae7cc9595fd97694521c5461e
}