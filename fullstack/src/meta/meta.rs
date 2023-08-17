#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meta {
    #[prost(message, optional, tag = "1")]
    pub dag: ::core::option::Option<Dag>,
    #[prost(message, optional, tag = "2")]
    pub ack: ::core::option::Option<Ack>,
    #[prost(message, optional, tag = "3")]
    pub bfs: ::core::option::Option<Bfs>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dag {
    #[prost(string, tag = "1")]
    pub predict: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rough_sort: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ack: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ack {
    #[prost(string, tag = "1")]
    pub databus: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enable_abase: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bfs {
    #[prost(string, tag = "1")]
    pub bfs_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enable_fe: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityMeta {
    #[prost(map = "string, message", tag = "1")]
    pub metas: ::std::collections::HashMap<::prost::alloc::string::String, Meta>,
}
