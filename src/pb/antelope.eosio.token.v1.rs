// @generated
// This file is @generated by prost-build.
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
    /// block information
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub action_ordinal: u32,
    /// Action execution_index
    #[prost(uint32, tag="7")]
    pub index: u32,
    /// Vec<PermissionLevel> (ex: \["account@active"\])
    #[prost(string, repeated, tag="8")]
    pub authorization: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// contract & scope
    ///
    /// Name (ex: "eosio.token")
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    /// SymbolCode (ex: "EOS")
    #[prost(string, tag="11")]
    pub symcode: ::prost::alloc::string::String,
    /// ExtendedSymbol (ex: "4,EOS@eosio.token")
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="13")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub memo: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="17")]
    pub precision: u32,
    #[prost(int64, tag="18")]
    pub amount: i64,
    #[prost(double, tag="19")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issue {
    /// block information
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub action_ordinal: u32,
    /// Action execution_index
    #[prost(uint32, tag="7")]
    pub index: u32,
    /// Vec<PermissionLevel> (ex: \["account@active"\])
    #[prost(string, repeated, tag="8")]
    pub authorization: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// contract & scope
    ///
    /// Name (ex: "eosio.token")
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    /// SymbolCode (ex: "EOS")
    #[prost(string, tag="11")]
    pub symcode: ::prost::alloc::string::String,
    /// ExtendedSymbol (ex: "4,EOS@eosio.token")
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="13")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub memo: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="16")]
    pub precision: u32,
    #[prost(int64, tag="17")]
    pub amount: i64,
    #[prost(double, tag="18")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Retire {
    /// block information
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub action_ordinal: u32,
    /// Action execution_index
    #[prost(uint32, tag="7")]
    pub index: u32,
    /// Vec<PermissionLevel> (ex: \["account@active"\])
    #[prost(string, repeated, tag="8")]
    pub authorization: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// contract & scope
    ///
    /// Name (ex: "eosio.token")
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    /// SymbolCode (ex: "EOS")
    #[prost(string, tag="11")]
    pub symcode: ::prost::alloc::string::String,
    /// ExtendedSymbol (ex: "4,EOS@eosio.token")
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="16")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub memo: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="18")]
    pub precision: u32,
    #[prost(int64, tag="19")]
    pub amount: i64,
    #[prost(double, tag="20")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Create {
    /// block information
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub action_ordinal: u32,
    /// Action execution_index
    #[prost(uint32, tag="7")]
    pub index: u32,
    /// Vec<PermissionLevel> (ex: \["account@active"\])
    #[prost(string, repeated, tag="8")]
    pub authorization: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// contract & scope
    ///
    /// Name (ex: "eosio.token")
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    /// SymbolCode (ex: "EOS")
    #[prost(string, tag="11")]
    pub symcode: ::prost::alloc::string::String,
    /// ExtendedSymbol (ex: "4,EOS@eosio.token")
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="13")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub maximum_supply: ::prost::alloc::string::String,
    /// extras
    #[prost(uint32, tag="15")]
    pub precision: u32,
    #[prost(int64, tag="16")]
    pub amount: i64,
    #[prost(double, tag="17")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    /// block information
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub trx_id: ::prost::alloc::string::String,
    /// Action execution_index
    #[prost(uint32, tag="6")]
    pub action_index: u32,
    /// DbOps index
    #[prost(uint32, tag="7")]
    pub index: u32,
    /// contract & scope
    ///
    /// Name (ex: "eosio.token")
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    /// SymbolCode (ex: "EOS")
    #[prost(string, tag="11")]
    pub symcode: ::prost::alloc::string::String,
    /// ExtendedSymbol (ex: "4,EOS@eosio.token")
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
    /// DbOps
    ///
    /// db_op::Operation
    #[prost(string, tag="13")]
    pub operation: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="14")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub balance: ::prost::alloc::string::String,
    #[prost(int64, tag="16")]
    pub balance_delta: i64,
    /// extras
    #[prost(uint32, tag="17")]
    pub precision: u32,
    #[prost(int64, tag="18")]
    pub amount: i64,
    #[prost(double, tag="19")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyChange {
    /// block information
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub trx_id: ::prost::alloc::string::String,
    /// Action execution_index
    #[prost(uint32, tag="6")]
    pub action_index: u32,
    /// DbOps index
    #[prost(uint32, tag="7")]
    pub index: u32,
    /// contract & scope
    ///
    /// Name (ex: "eosio.token")
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    /// SymbolCode (ex: "EOS")
    #[prost(string, tag="11")]
    pub symcode: ::prost::alloc::string::String,
    /// ExtendedSymbol (ex: "4,EOS@eosio.token")
    #[prost(string, tag="12")]
    pub token: ::prost::alloc::string::String,
    /// DbOps
    ///
    /// db_op::Operation
    #[prost(string, tag="13")]
    pub operation: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="14")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub supply: ::prost::alloc::string::String,
    #[prost(int64, tag="16")]
    pub supply_delta: i64,
    #[prost(string, tag="17")]
    pub max_supply: ::prost::alloc::string::String,
    #[prost(int64, tag="18")]
    pub max_supply_delta: i64,
    /// extras
    #[prost(uint32, tag="19")]
    pub precision: u32,
    #[prost(int64, tag="20")]
    pub amount: i64,
    #[prost(double, tag="21")]
    pub value: f64,
}
// @@protoc_insertion_point(module)
