//! This example runs the Client/Server showcasing how to:
//!
//! - Send Settings on established Websocket and act on EndOfLine change from `arduino-serial-plotter-webapp`
//! - Sends a data message every ~1 sec with random data and 2 different data lines in the same message
//!
//! By default, `tracing` will run with TRACE level or you can use the `RUST_LOG` env. variable
//! to override the default level.
use core::time::Duration;

use futures_util::StreamExt;
use rand::prelude::*;
use tokio::net::TcpListener;
use tokio_websockets::{Error, ServerBuilder};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use arduino_plotter::{
    protocol::{ClientCommand, EndOfLine, MonitorModelState, MonitorSettings},
    Client, Server, ServerError,
};

async fn run_server_task(mut server: Server, client: Client) {
    while let Some(value) = server.next().await {
        match value {
            Ok(message) => {
                info!("Received message: {message:?}");

                match message {
                    ClientCommand::SendMessage(_) => {}
                    ClientCommand::ChangeSettings(monitor_settings) => {
                        // if we have an new EndOfLine passed, we need to return it to the UI
                        // in order to get set in the UI as the new value
                        match monitor_settings.monitor_ui_settings {
                            Some(MonitorModelState {
                                line_ending: Some(eol),
                                ..
                            }) => {
                                let eol_result = client
                                    .set_monitor_settings(MonitorSettings {
                                        monitor_ui_settings: Some(MonitorModelState {
                                            line_ending: Some(eol),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    })
                                    .await;

                                match eol_result {
                                    Ok(_) => info!("New End of Line is set: {eol}"),
                                    Err(err) => {
                                        error!(?err, "New End of Line was not set in the UI")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(err) => {
                error!(?err, "Error when receiving from socket");

                match err {
                    ServerError::Ws(Error::Io(_))
                    | ServerError::Ws(Error::AlreadyClosed)
                    | ServerError::Ws(Error::CannotResolveHost) => {
                        // stop the spawned task if the Websocket has been stopped
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}

async fn run_client_task(client: Client) {
    // using existing Client
    {
        let settings = MonitorSettings {
            pluggable_monitor_settings: None,
            monitor_ui_settings: Some(MonitorModelState {
                dark_theme: Some(true),
                connected: Some(true),
                line_ending: Some(EndOfLine::NewLine),
                // this will trigger a Close on the currently established connection, do not send!
                // ws_port: Some(3000),
                ..Default::default()
            }),
        };

        info!("Monitor Settings to be sent: {settings:?}");

        match client.set_monitor_settings(settings).await {
            Ok(_) => {}
            Err(err) => error!("Failed to set settings: {err}"),
        }
    }

    loop {
        let mut data = vec![];
        for _i in 0..6 {
            let mut rng = rand::thread_rng();
            let rand: u32 = rng.gen_range(0..100);
            data.push(rand);
        }
        let data1_str = format!("L1:{},L2:{},L3:{}\n", data[0], data[1], data[2]);
        let data2_str = format!("A:{},B:{},C:{}\n", data[3], data[4], data[5]);
        let data = vec![data1_str, data2_str];

        let send_result = client.send(&data).await;
        match send_result {
            Ok(_) => info!("Sent data message: {data:?}"),
            Err(err) => {
                error!("Sending data message failed: {err:?}");
                if matches!(
                    err,
                    Error::AlreadyClosed | Error::Io(_) | Error::CannotResolveHost
                ) {
                    // stop current task for current connection
                    break;
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

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
            let (client, server) = (Client::new(ws_sink), Server::new(ws_stream));

            tokio::spawn(run_server_task(server, client.clone()));
            tokio::spawn(run_client_task(client));
        }
    }
}
