use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    time::Duration,
};

use anyhow::bail;
use tokio::{
    io::{copy_bidirectional, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{lookup_host, TcpListener, TcpStream},
};
use tracing::{error, info, trace};

use crate::core::constant::{Atyp, Rsp, CONNECT, NO_AUTH, SUCCESS, VERSION};

pub async fn start(addr: &SocketAddr, timeout: Duration) -> anyhow::Result<()> {
    let lsn = TcpListener::bind(&addr).await?;
    info!("server start at {}", &addr);
    while let Ok((stream, addr)) = lsn.accept().await {
        info!("receive request from {}", addr);
        tokio::spawn(async move {
            if let Err(err) = handle(stream, timeout).await {
                error!("error: {}", err);
            }
        });
    }
    Ok(())
}

async fn handle(mut stream: TcpStream, timeout: Duration) -> anyhow::Result<()> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).await?;

    let ver = header[0];
    let nth = header[1];
    trace!("version:{} auth method num :{}", ver, nth);

    for _ in 0..nth {
        let mut method = [0u8; 1];
        stream.read_exact(&mut method).await?;
        trace!("supported auth method:{:?}", &method);
    }

    let mut response = [0u8; 2];
    response[0] = VERSION;
    response[1] = NO_AUTH;
    stream.write_all(&response).await?;
    trace!("auth success");

    let mut packet = [0u8; 4];
    stream.read(&mut packet).await?;

    let ver = packet[0];
    if ver != VERSION {
        bail!("unsupport version {} ", ver);
    }

    let cmd = packet[1];

    if cmd != CONNECT {
        bail!("unsupport command {} ", cmd);
    }

    let atyp: Atyp = packet[3].into();
    let (addr, port) = read_addr(&mut stream, atyp).await?;
    let target = bytes2socket(addr, port, atyp).await?;
    info!("address info: target {:?}", &target);
    let reply = Rsp::new(SUCCESS, packet[3]);
    stream.write_all(&reply.to_bytes()).await?;

    let target = tokio::time::timeout(
        timeout,
        async move { TcpStream::connect(&target[..]).await },
    )
    .await??;

    proxy(stream, target).await?;
    Ok(())
}

pub async fn proxy<I, O>(mut inbound: I, mut outbound: O) -> anyhow::Result<()>
where
    I: AsyncRead + AsyncWrite + Unpin,
    O: AsyncRead + AsyncWrite + Unpin,
{
    match copy_bidirectional(&mut inbound, &mut outbound).await {
        Ok(res) => {
            info!("proxy done. write size: {}, read size: {}", res.0, res.1);
            Ok(())
        }
        Err(err) => {
            error!("proxy error. {:?}", &err);
            Err(err.into())
        }
    }
}

async fn read_addr(stream: &mut TcpStream, atyp: Atyp) -> anyhow::Result<(Vec<u8>, u16)> {
    let daddr = match atyp {
        Atyp::V4 => {
            let mut addr = [0u8; 4];
            stream.read_exact(&mut addr).await?;
            addr.to_vec()
        }
        Atyp::DOMAIN => {
            let len = stream.read_u8().await?;
            let mut domain = vec![0u8; len as usize];
            stream.read_exact(&mut domain).await?;
            domain.to_vec()
        }
        Atyp::V6 => {
            let mut addr = [0u8; 16];
            stream.read_exact(&mut addr).await?;
            addr.to_vec()
        }
    };

    let dport = stream.read_u16().await?;
    Ok((daddr, dport))
}

async fn bytes2socket(addr: Vec<u8>, port: u16, atyp: Atyp) -> anyhow::Result<Vec<SocketAddr>> {
    match atyp {
        Atyp::V4 => {
            let rsp = SocketAddr::from(SocketAddrV4::new(
                Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]),
                port,
            ));

            Ok(vec![rsp])
        }
        Atyp::V6 => {
            let rsp = (0..8)
                .map(|x| (u16::from(addr[x * 2]) << 8) | u16::from(addr[(x * 2) + 1]))
                .collect::<Vec<u16>>();

            Ok(vec![SocketAddr::from(SocketAddrV6::new(
                Ipv6Addr::new(
                    rsp[0], rsp[1], rsp[2], rsp[3], rsp[4], rsp[5], rsp[6], rsp[7],
                ),
                port,
                0,
                0,
            ))])
        }

        Atyp::DOMAIN => {
            let mut domain = String::from_utf8_lossy(addr.as_slice()).to_string();
            domain.push(':');
            domain.push_str(&port.to_string());
            Ok(lookup_host(domain).await?.collect())
        }
    }
}
