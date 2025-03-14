use super::{Packet, packet::ReceivedPacket};
use std::sync::Arc;
use tokio::{net::UdpSocket, task::JoinHandle};

const UDP_BUFFER_SIZE: usize = 1024;

pub struct NetworkListener {
    net: Arc<UdpSocket>,
    tx: tokio::sync::mpsc::Sender<ReceivedPacket>,
    listening_task: Option<JoinHandle<()>>,
}

impl NetworkListener {
    pub fn new(net: Arc<UdpSocket>, tx: tokio::sync::mpsc::Sender<ReceivedPacket>) -> Self {
        Self {
            net,
            tx,
            listening_task: None,
        }
    }

    pub async fn start_listen(&mut self) -> crate::Result<()> {
        let net = self.net.clone();
        let tx = self.tx.clone();

        let mut buf = [0; UDP_BUFFER_SIZE];
        let task = tokio::spawn(async move {
            loop {
                let (len, remote) = match net.recv_from(&mut buf).await {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("Error while reading from message socket! {:?}", e);
                        continue;
                    }
                };
                log::debug!("Received message from: {:?}", remote);

                let data = &buf[..len];
                let packet = match serde_json::from_slice::<Packet>(data) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("Error while deserializing incoming packet! {:?}", e);
                        continue;
                    }
                };
                log::debug!("Deserialized received packet: {:?}", packet);

                match tx.send(ReceivedPacket { remote, packet }).await {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!(
                            "Error while sending received packet to message service channel! {:?}",
                            e
                        );
                        continue;
                    }
                };
            }
        });
        self.listening_task = Some(task);

        log::info!("Listening for messages...");

        Ok(())
    }
}
