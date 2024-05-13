use std::{sync::Arc, task::Poll};

use futures_util::{
    stream::{SplitSink, SplitStream},
    FutureExt, SinkExt, Stream, StreamExt,
};
use thiserror::Error;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_websockets::{Error, Message, WebSocketStream};
use tracing::{debug, trace};

use crate::protocol::{ClientCommand, MiddlewareCommand, MonitorSettings};

#[derive(Debug, Error)]
pub enum ServerError {
    /// A Websocket Error occurred
    #[error(transparent)]
    Ws(#[from] tokio_websockets::Error),
    /// An error occurred during the deserializing of a JSON to a value
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// WebSocket Message response was not a text one.
    ///
    /// See [`tokio_websockets::Message::as_text`] for more details.
    #[error("Text-based (json) client command is expected from the serial plotter")]
    NonTextMessage,
}

/// Server is needed for receiving messages from the plotter app.
///
/// 2 messages are possible [`ClientCommand`] and a websocket closing message:
/// - `SEND_MESSAGE` - sending message to the board through serial
/// - `CHANGE_SETTINGS` - settings for [`EndOfLine`] has bee changed in the application
///
/// Cheap to clone as it has an internal Atomic reference counter ([`Arc`]) for the Websocket Stream
///
/// [`EndOfLine`]: crate::protocol::EndOfLine
#[derive(Debug, Clone)]
pub struct Server {
    ws_stream: Arc<Mutex<SplitStream<WebSocketStream<TcpStream>>>>,
}
impl Server {
    pub fn new(ws_stream: SplitStream<WebSocketStream<TcpStream>>) -> Self {
        Self {
            ws_stream: Arc::new(Mutex::new(ws_stream)),
        }
    }
}

impl Stream for Server {
    type Item = Result<ClientCommand, ServerError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut pin = Box::pin(self.ws_stream.lock());
        match pin.poll_unpin(cx) {
            Poll::Ready(mut guard) => guard.poll_next_unpin(cx).map(|next_value| {
                next_value.and_then(|result| {
                    let a = result.map_err(ServerError::Ws).and_then(|message| {
                        if message.is_close() {
                            debug!("Websocket closed");
                            return Ok(None);
                        }

                        // causes unsafe precondition panic on Rust 1.78
                        // match message.as_close() {
                        //     Some((close_code, reason)) => {
                        //         debug!(?close_code, reason, "Websocket closed");
                        //         // todo: notify the client for the closed websocket
                        //         return Ok(None);
                        //     }
                        //     None => {}
                        // }

                        let client_command = message
                            .as_text()
                            .ok_or(ServerError::NonTextMessage)
                            .and_then(|text_payload| {
                            trace!(text_payload, "Text WS message received");

                            serde_json::from_str::<ClientCommand>(text_payload)
                                .map_err(ServerError::Json)
                        })?;

                        Ok(Some(client_command))
                    });

                    a.transpose()
                })
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Client for sending Data message or [`MiddlewareCommand`] (i.e. [`MonitorSettings`])
///
/// Cheap to clone as it has an internal Atomic reference counter ([`Arc`]) for the Websocket Stream
#[derive(Debug, Clone)]
pub struct Client {
    ws_sink: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}

impl Client {
    pub fn new(ws_sink: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self {
            ws_sink: Arc::new(Mutex::new(ws_sink)),
        }
    }

    /// Send a [`MonitorSettings`] ([`MiddlewareCommand`]) to the Arduino Serial Plotter UI
    /// through an already established connection.
    pub async fn set_monitor_settings(
        &self,
        monitor_settings: MonitorSettings,
    ) -> Result<(), Error> {
        let settings = MiddlewareCommand(monitor_settings);

        trace!("Settings to be sent: {settings:?}");
        let command_json = serde_json::to_string(&settings).unwrap();
        trace!("Settings command JSON to be sent: {command_json:?}");

        self.ws_sink
            .lock()
            .await
            .send(Message::text(command_json))
            .await
    }

    /// Send a Data lines message to the Arduino Serial Plotter UI to plot.
    pub async fn send(&self, data: &[&str]) -> Result<(), Error> {
        let data_json = serde_json::to_string(data).expect("Should always be serializable!");

        self.ws_sink
            .lock()
            .await
            .send(Message::text(data_json))
            .await
    }
}
