use std::net::SocketAddr;

use tokio::{
    io::{copy_bidirectional, AsyncRead, AsyncWrite},
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};
use tracing::{debug, error, info};

pub async fn relay<I, O>(mut input: I, mut output: O) -> anyhow::Result<()>
where
    I: AsyncRead + AsyncWrite + Unpin,
    O: AsyncRead + AsyncWrite + Unpin,
{
    match copy_bidirectional(&mut input, &mut output).await {
        Ok(res) => info!("transfer done,bytes transfered: ({} ===> {})", res.0, res.1),
        Err(err) => error!("transfer error: {:?}", err),
    };

    Ok(())
}

pub async fn tcp_connect<T>(addr: T) -> anyhow::Result<TcpStream>
where
    T: ToSocketAddrs,
{
    match TcpStream::connect(addr).await {
        Ok(stream) => Ok(stream),
        Err(err) => Err(err.into()),
    }
}

pub async fn tcp_connect_with_timeout<T>(
    addr: T,
    duration: std::time::Duration,
) -> anyhow::Result<TcpStream>
where
    T: ToSocketAddrs,
{
    let fut = tcp_connect(addr);
    match timeout(duration, fut).await {
        Ok(result) => result,
        Err(err) => Err(err.into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddrError {
    #[error("DNS Resolution failed")]
    DNSResolutionFailed,
    #[error("Can't read IPv4")]
    IPv4Unreadable,
    #[error("Can't read IPv6")]
    IPv6Unreadable,
    #[error("Can't read port number")]
    PortNumberUnreadable,
    #[error("Can't read domain len")]
    DomainLenUnreadable,
    #[error("Can't read Domain content")]
    DomainContentUnreadable,
    #[error("Malformed UTF-8")]
    Utf8,
    #[error("Unknown address type")]
    IncorrectAddressType,
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum TargetAddr {
    Ip(SocketAddr),
    Domain(String, u16),
}

impl TargetAddr {
    pub async fn resolve_dns(self) -> anyhow::Result<TargetAddr> {
        match self {
            TargetAddr::Ip(ip) => Ok(TargetAddr::Ip(ip)),
            TargetAddr::Domain(domain, port) => {
                let rsp = tokio::net::lookup_host((&domain[..], port))
                    .await?
                    .next()
                    .ok_or(AddrError::Custom(
                        "Can't fetch DNS to the domain.".to_string(),
                    ))?;

                debug!("domain name resolved to {}", &rsp);
                // has been converted to an ip
                Ok(TargetAddr::Ip(rsp))
            }
        }
    }
}

pub mod consts {
    pub const VERSION: u8 = 0x05;

    pub const AUTH_METHOD_NONE: u8 = 0x00;
    pub const AUTH_METHOD_GSSAPI: u8 = 0x01;
    pub const AUTH_METHOD_PASSWORD: u8 = 0x02;
    pub const AUTH_METHOD_NOT_ACCEPTABLE: u8 = 0xff;

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

pub enum Atype {
    V4([u8; 4]),
    V6([u8; 16]),
    Domain(String),
}
