#[derive(thiserror::Error, Debug)]
pub enum SocksError {
    #[error("unsupported auth method.")]
    UnsupportedAuthMtd,
    #[error("unsupported command. {0}")]
    UnsupportedCmd(u8),
    #[error("invalid address type")]
    InvalidAddressType,
    #[error("{0}")]
    Custom(String),
}

pub enum Atype {
    V4([u8; 4]),
    V6([u8; 16]),
    Domain(String),
}

pub mod consts {
    pub const VERSION: u8 = 0x05;
    pub const AUTH_METHOD_NONE: u8 = 0x00;
    pub const AUTH_METHOD_GSSAPI: u8 = 0x01;
    pub const AUTH_METHOD_PASSWORD: u8 = 0x02;
    pub const AUTH_METHOD_NOT_ACCEPTABLE: u8 = 0xff;
    pub const AUTH_SUCCESS: u8 = 0x00;
    pub const CMD_TCP_CONNECT: u8 = 0x01;
    pub const CMD_TCP_BIND: u8 = 0x02;
    pub const CMD_UDP_ASSOCIATE: u8 = 0x03;
    pub const ADDR_TYPE_IPV4: u8 = 0x01;
    pub const ADDR_TYPE_DOMAIN_NAME: u8 = 0x03;
    pub const ADDR_TYPE_IPV6: u8 = 0x04;
    pub const REPLY_SUCCEEDED: u8 = 0x00;
    pub const REPLY_GENERAL_FAILURE: u8 = 0x01;
    pub const REPLY_CONNECTION_NOT_ALLOWED: u8 = 0x02;
    pub const REPLY_NETWORK_UNREACHABLE: u8 = 0x03;
    pub const REPLY_HOST_UNREACHABLE: u8 = 0x04;
    pub const REPLY_CONNECTION_REFUSED: u8 = 0x05;
    pub const REPLY_TTL_EXPIRED: u8 = 0x06;
    pub const REPLY_COMMAND_NOT_SUPPORTED: u8 = 0x07;
    pub const REPLY_ADDRESS_TYPE_NOT_SUPPORTED: u8 = 0x08;
    pub const ZERO: u8 = 0x00;
}

pub enum AuthMod {
    AuthNone,
    Password,
}
