// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Accounts {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Account>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symcode: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="5")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub balance: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub balance_delta: i64,
    /// extras
    #[prost(uint32, tag="10")]
    pub precision: u32,
    #[prost(int64, tag="11")]
    pub amount: i64,
    #[prost(double, tag="12")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Stat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symcode: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="5")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub max_supply: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub supply: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub supply_delta: i64,
    /// extras
    #[prost(uint32, tag="10")]
    pub precision: u32,
    #[prost(int64, tag="11")]
    pub amount: i64,
    #[prost(double, tag="12")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<TransferEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    /// transaction
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub symcode: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="6")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub memo: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="10")]
    pub precision: u32,
    #[prost(int64, tag="11")]
    pub amount: i64,
    #[prost(double, tag="12")]
    pub value: f64,
}
// @@protoc_insertion_point(module)
