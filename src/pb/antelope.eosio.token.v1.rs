// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="2")]
    pub issues: ::prost::alloc::vec::Vec<Issue>,
    #[prost(message, repeated, tag="3")]
    pub retires: ::prost::alloc::vec::Vec<Retire>,
    #[prost(message, repeated, tag="4")]
    pub creates: ::prost::alloc::vec::Vec<Create>,
    #[prost(message, repeated, tag="5")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
    #[prost(message, repeated, tag="6")]
    pub supply_changes: ::prost::alloc::vec::Vec<SupplyChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
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
    /// block information
    #[prost(uint64, tag="13")]
    pub block_num: u64,
    #[prost(message, optional, tag="14")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issue {
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
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="9")]
    pub precision: u32,
    #[prost(int64, tag="10")]
    pub amount: i64,
    #[prost(double, tag="11")]
    pub value: f64,
    /// block information
    #[prost(uint64, tag="12")]
    pub block_num: u64,
    #[prost(message, optional, tag="13")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Retire {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub symcode: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="6")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="9")]
    pub precision: u32,
    #[prost(int64, tag="10")]
    pub amount: i64,
    #[prost(double, tag="11")]
    pub value: f64,
    /// block information
    #[prost(uint64, tag="12")]
    pub block_num: u64,
    #[prost(message, optional, tag="13")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Create {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub symcode: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="6")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub maximum_supply: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="8")]
    pub precision: u32,
    #[prost(int64, tag="9")]
    pub amount: i64,
    #[prost(double, tag="10")]
    pub value: f64,
    /// block information
    #[prost(uint64, tag="11")]
    pub block_num: u64,
    #[prost(message, optional, tag="12")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
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
    /// block information
    #[prost(uint64, tag="13")]
    pub block_num: u64,
    #[prost(message, optional, tag="14")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyChange {
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
    /// block information
    #[prost(uint64, tag="13")]
    pub block_num: u64,
    #[prost(message, optional, tag="14")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
// @@protoc_insertion_point(module)
