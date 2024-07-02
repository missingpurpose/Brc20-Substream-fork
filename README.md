# BRC20 Substreams Powered Subgraph

This project is a subgraph for tracking Bitcoin cap table events and balances using Substreams. It leverages Rust, Protobuf, and various dependencies to process Bitcoin blockchain data and generate a cap table.

## Project Structure

- **src/**: Contains the main Rust source code.
  - **lib.rs**: Main entry point for the Substreams module.
  - **address.rs**: Utility functions for handling Bitcoin addresses.
  - **pb/**: Generated Protobuf files.
  - **btc_utils.rs**: Utility functions for Bitcoin-related operations.
  - **tables_utils.rs**: Utility functions for handling table operations.
- **proto/**: Contains Protobuf definitions.
  - **cap_table.proto**: Protobuf definitions for the cap table.
  - **bitcoin.proto**: Protobuf definitions for Bitcoin transactions.
- **build.rs**: Build script for compiling Protobuf files.
- **Cargo.toml**: Rust project configuration.
- **package.json**: NPM package configuration for managing scripts and dependencies.
- **substreams.yaml**: Configuration for Substreams modules.
- **subgraph.yaml**: Configuration for the subgraph.
- **schema.graphql**: GraphQL schema for the subgraph.

## Installation

1. **Clone the repository:**


2. **Install Rust:**
   Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust.

3. **Install Protobuf Compiler:**
   Download and install the Protobuf compiler from [protobuf/releases](https://github.com/protocolbuffers/protobuf/releases).

4. **Install Node.js and Yarn:**
   Download and install Node.js from [nodejs.org](https://nodejs.org/).
   Then install Yarn
   
   
5. **Install dependencies:**

clang, buf , protoc,


## Building the Project

1. **Generate Protobuf files:**


2. **Build the Substreams module:**


3. **Package the Substreams module:**


## Running the Substreams

To run the Substreams and start processing data:


## Scripts

- **codegen**: `graph codegen`
- **deploy**: `graph deploy`
- **deploy-local**: `graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 brc20`
- **remove-local**: `graph remove --node http://localhost:8020/ brc20`
- **subgraph:build**: `graph build`
- **substreams:build**: `cargo build --target wasm32-unknown-unknown --release`
- **substreams:clean**: `rm -rf ./target && rm -rf ./src/pb`
- **substreams:package**: `substreams pack ./substreams.yaml`
- **substreams:prepare**: `yarn substreams:protogen && yarn substreams:build && yarn substreams:package`
- **substreams:protogen**: `substreams protogen ./substreams.yaml --exclude-paths='sf/substreams,google'`
- **substreams:stream**: `substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml graph_out -s 12292922 -t +10`

## Configuration Files

- **buf.gen.yaml**: Configuration for generating Protobuf files.
- **buf.yaml**: Configuration for Buf tool.
- **substreams.yaml**: Configuration for Substreams modules.
- **subgraph.yaml**: Configuration for the subgraph.

## Protobuf Definitions

- **cap_table.proto**: Defines the structure for the cap table.
- **bitcoin.proto**: Defines the structure for Bitcoin transactions.

## GraphQL Schema

- **schema.graphql**: Defines the GraphQL schema for the subgraph.

## License

This project is licensed under the MIT License.