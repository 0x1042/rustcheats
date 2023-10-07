use std::{io::Read, net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

pub struct Client {
    pub inner: Arc<Mutex<TcpStream>>,
}

impl Client {
    pub async fn new(addr: SocketAddr) -> Self {
        let stream = TcpStream::connect(&addr).await.unwrap();
        let inner = Arc::new(Mutex::new(stream));
        Self { inner }
    }

    pub async fn run(self) {
        loop {
            let stream = self.inner.clone();
        }
    }

    pub async fn read_frame(self, stream: Arc<Mutex<TcpStream>>) -> anyhow::Result<()> {
        let mut buf = [0u8; 2];
        stream.lock().await.read_exact(&mut buf).await?;

        Ok(())
    }
}
