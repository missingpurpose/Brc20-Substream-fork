// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitcoinTransaction {
    #[prost(string, tag="1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub inputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(double, repeated, tag="3")]
    pub outputs: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitcoinTransactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<BitcoinTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBalance {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub balance: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBalances {
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<AddressBalance>,
}
/// Renamed to avoid conflict
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitcoinCapTableEntry {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapTable {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<BitcoinCapTableEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Addresses {
    #[prost(string, repeated, tag="1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
