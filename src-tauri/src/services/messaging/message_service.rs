use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::{
    net::{TcpSocket, UdpSocket},
    task::JoinHandle,
};

type Result<T> = std::result::Result<T, String>;

pub struct MessageService {
    listening_task: Option<JoinHandle<()>>,
    net: Arc<UdpSocket>,
}

impl MessageService {
    pub async fn start() -> Result<Self> {
        let net = UdpSocket::bind("0.0.0.0:45000")
            .await
            .map_err(|e| e.to_string())?;

        Ok(Self {
            listening_task: None,
            net: Arc::new(net),
        })
    }

    pub async fn listen(&mut self) -> Result<()> {
        let mut buf = [0; 1024];
        let net = Arc::clone(&self.net);

        let task = tokio::spawn(async move {
            loop {
                let count = match net.recv(&mut buf).await {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("Error while reading from message socket! {:?}", e);
                        continue;
                    }
                };

                let data = &buf[..count];
                match serde_json::from_slice(data) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("Could not deserialize data into json! {:?}", e);
                        continue;
                    }
                };
            }
        });
        self.listening_task = Some(task);

        Ok(())
    }
}
