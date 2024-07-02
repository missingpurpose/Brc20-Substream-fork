// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapTable {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<CapTableEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapTableEntry {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, repeated, tag="1")]
    pub tx: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, repeated, tag="1")]
    pub vout: ::prost::alloc::vec::Vec<Vout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vout {
    #[prost(bytes="vec", tag="1")]
    pub script_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag="2")]
    pub value: f64,
}
// @@protoc_insertion_point(module)
