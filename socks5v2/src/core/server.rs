use std::{
    io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    sync::Arc,
};

use anyhow::Context;
use async_stream::try_stream;
use futures_core::Stream;
use futures_util::{pin_mut, stream::StreamExt};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{lookup_host, TcpListener, TcpStream, ToSocketAddrs},
};
use tracing::{debug, error, info, trace};

use super::{
    auth::Config,
    types::{consts, Atype, AuthMod},
};
use crate::core::{
    relay::{relay, tcp_connect_with_timeout},
    types::SocksError,
};

pub struct Server {
    pub listener: TcpListener,
    pub config: Arc<Config>,
}

impl Server {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Server> {
        let listener = TcpListener::bind(&addr).await?;
        let config = Arc::new(Config::default());
        Ok(Server { listener, config })
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = Arc::new(config);
    }

    pub fn incoming(self) -> impl Stream<Item = io::Result<TcpSession<TcpStream>>> {
        try_stream! {
            loop {
                let (stream, _addr) = self.listener.accept().await?;
                let socket = TcpSession::new(stream, self.config.clone());
                yield socket;
            }
        }
    }

    pub async fn serve(self) {
        let income = self.incoming();
        pin_mut!(income);

        while let Some(conn_res) = income.next().await {
            match conn_res {
                Ok(conn) => {
                    tokio::spawn(async move {
                        if let Err(e) = conn.run().await {
                            error!("spawn error: {:#}", &e);
                        }
                    });
                }
                Err(err) => {
                    error!("exception:{:?}", err);
                }
            }
        }
    }
}

pub struct TcpSession<T: AsyncRead + AsyncWrite + Unpin> {
    inner: T,
    config: Arc<Config>,
    authmod: AuthMod,
}

impl<T: AsyncRead + AsyncWrite + Unpin> TcpSession<T> {
    pub fn new(socket: T, config: Arc<Config>) -> Self {
        let mut authmod = AuthMod::AuthNone;

        if !config.auth_db.is_empty() {
            authmod = AuthMod::Password;
        }
        TcpSession {
            inner: socket,
            config,
            authmod,
        }
    }

    pub async fn run(mut self) -> anyhow::Result<TcpSession<T>> {
        trace!("incoming request.{:?}", std::thread::current().id());
        self.auth().await?;
        self.relay().await?;
        trace!("relay done");
        Ok(self)
    }

    async fn auth(&mut self) -> anyhow::Result<()> {
        let mut buf = [0u8; 2];
        let [ver, mlen] = self
            .inner
            .read_exact(&mut buf)
            .await
            .map(|_| buf)
            .context("read method error")?;

        debug!("auth. {ver}, {mlen}");
        let mut buf = vec![0u8; mlen as usize];

        let mthds = self
            .inner
            .read_exact(&mut buf)
            .await
            .map(|_| buf)
            .context("read method error")?;

        debug!("mthds:{:?}", mthds);

        let reply = match self.authmod {
            AuthMod::AuthNone => consts::AUTH_SUCCESS,
            AuthMod::Password => {
                if !mthds.contains(&consts::AUTH_METHOD_PASSWORD) {
                    consts::AUTH_METHOD_NOT_ACCEPTABLE
                } else {
                    consts::AUTH_METHOD_PASSWORD
                }
            }
        };
        debug!("auth_reply:{:?}", reply);

        self.inner
            .write(&[consts::VERSION, reply])
            .await
            .context("reply auth failed")?;

        if reply == consts::AUTH_METHOD_NOT_ACCEPTABLE {
            return Err(SocksError::UnsupportedAuthMtd.into());
        }

        if reply != consts::AUTH_METHOD_NONE {
            // read username
            let mut buf = [0u8; 2];
            self.inner.read_exact(&mut buf).await?;
            let mut name = Vec::with_capacity(buf[1] as usize);
            self.inner.read_buf(&mut name).await?;

            // read password
            let passwd_len = self.inner.read_u8().await?;
            let mut passwd = Vec::with_capacity(passwd_len as usize);
            self.inner.read_buf(&mut passwd).await?;

            let username = String::from_utf8(name)?;
            let passwd = String::from_utf8(passwd)?;

            debug!("user:{:?} passwd:{:?}", &username, &passwd);
            let mut auth_rsp = consts::AUTH_METHOD_NOT_ACCEPTABLE;
            if let Some(password) = self.config.auth_db.get(username.as_str()) {
                if passwd.eq(password) {
                    auth_rsp = consts::AUTH_SUCCESS;
                }
            }
            self.inner.write(&[1, auth_rsp]).await?;
            debug!("auth success");
        } else {
            debug!("no need auth");
        }

        Ok(())
    }

    async fn relay(&mut self) -> anyhow::Result<()> {
        let mut buf = [0u8; 4];

        let [ver, cmd, rsv, atype] = self
            .inner
            .read_exact(&mut buf)
            .await
            .map(|_| buf)
            .context("read frame header error")?;

        info!("relay. read frame success. {ver} {cmd} {rsv} {atype}");

        if ver != consts::VERSION {
            return Err(anyhow::anyhow!("unsupported socks version"));
        }

        if cmd != consts::CMD_TCP_CONNECT {
            return Err(anyhow::anyhow!("unsupported command"));
        }

        let target = self.read_addr(atype).await?;

        trace!("target {:?}", &target);

        let outbound = tcp_connect_with_timeout(target.as_slice(), self.config.timeout).await?;

        let rsp = [
            consts::VERSION,
            consts::REPLY_SUCCEEDED,
            rsv,
            atype,
            consts::ZERO,
            consts::ZERO,
            consts::ZERO,
            consts::ZERO,
            consts::ZERO,
            consts::ZERO,
        ];

        self.inner.write(&rsp).await?;

        info!("reply to client success");

        self.inner.flush().await.context("flush error")?;

        let _ = relay(&mut self.inner, outbound).await;

        Ok(())
    }

    async fn read_addr(&mut self, atype: u8) -> anyhow::Result<Vec<SocketAddr>> {
        let addr = match atype {
            consts::ADDR_TYPE_IPV4 => {
                let mut buf = [0u8; 4];
                self.inner
                    .read_exact(&mut buf)
                    .await
                    .context("read address error")?;
                Atype::V4(buf)
            }
            consts::ADDR_TYPE_IPV6 => {
                let mut buf = [0u8; 16];
                self.inner
                    .read_exact(&mut buf)
                    .await
                    .context("read address error")?;

                Atype::V6(buf)
            }
            consts::ADDR_TYPE_DOMAIN_NAME => {
                let len = self.inner.read_u8().await?;
                let mut buf = vec![0u8; len as usize];

                self.inner
                    .read_exact(&mut buf)
                    .await
                    .context("read address error")?;

                let domain = String::from_utf8(buf)?;

                Atype::Domain(domain)
            }
            _ => return Err(anyhow::anyhow!("unsuppored address")),
        };

        let port = self.inner.read_u16().await?;

        let remotes = match addr {
            Atype::V4(addr) => {
                let rsp = SocketAddr::from(SocketAddrV4::new(
                    Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]),
                    port,
                ));

                vec![rsp]
            }
            Atype::V6(addr) => {
                let rsp = (0..8)
                    .map(|x| (u16::from(addr[x * 2]) << 8) | u16::from(addr[(x * 2) + 1]))
                    .collect::<Vec<u16>>();

                vec![SocketAddr::from(SocketAddrV6::new(
                    Ipv6Addr::new(
                        rsp[0], rsp[1], rsp[2], rsp[3], rsp[4], rsp[5], rsp[6], rsp[7],
                    ),
                    port,
                    0,
                    0,
                ))]
            }
            Atype::Domain(addr) => {
                let mut domain = addr.clone();
                domain.push(':');
                domain.push_str(&port.to_string());
                lookup_host(domain).await?.collect()
            }
        };

        Ok(remotes)
    }
}
