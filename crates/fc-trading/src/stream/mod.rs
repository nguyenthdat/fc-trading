mod connection;
mod protocol;
mod supervisor;

use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{Result, StreamOptions, TradingClient};

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamEvent {
    Broadcast(String),
    ServerError(String),
}

pub struct TradingStream {
    events: mpsc::Receiver<Result<StreamEvent>>,
    cancellation: CancellationToken,
    task: Option<JoinHandle<Result<()>>>,
}

impl TradingStream {
    pub async fn next(&mut self) -> Option<Result<StreamEvent>> {
        self.events.recv().await
    }

    pub async fn close(mut self) -> Result<()> {
        self.cancellation.cancel();
        match self.task.take() {
            Some(task) => task.await.map_err(crate::Error::StreamSupervisor)?,
            None => Ok(()),
        }
    }
}

impl Drop for TradingStream {
    fn drop(&mut self) {
        self.cancellation.cancel();
    }
}

impl TradingClient {
    pub async fn stream(&self, options: StreamOptions) -> Result<TradingStream> {
        let initial = connection::connect(self, &options).await?;
        let (sender, receiver) = mpsc::channel(options.channel_capacity());
        let cancellation = CancellationToken::new();
        let task = tokio::spawn(supervisor::run(
            self.clone(),
            options,
            initial,
            sender,
            cancellation.child_token(),
        ));
        Ok(TradingStream {
            events: receiver,
            cancellation,
            task: Some(task),
        })
    }
}

#[cfg(test)]
mod tests;
