use tokio::{
    io::{copy_bidirectional, AsyncRead, AsyncWrite},
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};
use tracing::{error, info};

pub async fn relay<I, O>(mut input: I, mut output: O) -> anyhow::Result<()>
where
    I: AsyncRead + AsyncWrite + Unpin,
    O: AsyncRead + AsyncWrite + Unpin,
{
    match copy_bidirectional(&mut input, &mut output).await {
        Ok(res) => info!("relay done: {} -> {}", res.0, res.1),
        Err(err) => error!("relay error: {:?}", err),
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
