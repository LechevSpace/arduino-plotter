use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_websockets::ServerBuilder;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use arduino_plotter::{
    protocol::{EndOfLine, MonitorModelState, MonitorSettings},
    Client, Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    // listen at port 3030
    let listener = TcpListener::bind("127.0.0.1:3030").await?;

    loop {
        while let Ok((stream, _plotter_addr)) = listener.accept().await {
            let ws_stream = match ServerBuilder::new().accept(stream).await {
                Ok(x) => x,
                Err(err) => {
                    error!("Error performing HTTP upgrade handshake request: {err}");
                    continue;
                }
            };

            let (ws_sink, ws_stream) = ws_stream.split();
            let (client, mut server) = (Client::new(ws_sink), Server::new(ws_stream));

            let server_fut = async {
                while let Some(result) = server.next().await {
                    info!("Client command received result: {result:?}")
                }
            };

            let client_fut = async {
                // set some settings
                {
                    let settings = MonitorSettings {
                        pluggable_monitor_settings: None,
                        monitor_ui_settings: Some(MonitorModelState {
                            // A connection to a serial device has been established
                            connected: Some(true),
                            line_ending: Some(EndOfLine::NewLine),
                            ..Default::default()
                        }),
                    };

                    info!("Monitor Settings to be sent: {settings:?}");

                    match client.set_monitor_settings(settings).await {
                        Ok(_) => {}
                        Err(err) => error!("Failed to set settings: {err}"),
                    }
                }
            };

            // will send a single Monitor settings message (client) and run the server forever.
            futures_util::join!(server_fut, client_fut);
        }
    }
}
