use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use rand::prelude::*;
use tokio::net::TcpListener;
use tokio_websockets::{Error, Message, ServerBuilder};

use arduino_plotter::protocol::{Command, MiddlewareCommand, MonitorModelState, MonitorSettings};

pub async fn listen(listener: TcpListener) -> ! {
    'main: loop {
        while let Ok((stream, _plotter_addr)) = listener.accept().await {
            let ws_stream = match ServerBuilder::new().accept(stream).await {
                Ok(x) => x,
                Err(err) => {
                    eprintln!("Error performing HTTP upgrade handshake request: {err}");
                    continue;
                }
            };

            let (mut ws_sink, mut ws_stream) = ws_stream.split();

            tokio::spawn(async move {
                while let Some(value) = ws_stream.next().await {
                    match value {
                        Ok(message) => println!("Received message: {message:?}"),
                        Err(err) => {
                            eprintln!("Error when receiving from socket: {err}");

                            match err {
                                Error::Io(_) | Error::AlreadyClosed | Error::CannotResolveHost => {
                                    // stop the spawned task if the Websocket has been stopped
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            });

            // using existing server
            {
                let settings = MiddlewareCommand(MonitorSettings {
                    pluggable_monitor_settings: None,
                    monitor_ui_settings: Some(MonitorModelState {
                        dark_theme: Some(true),
                        connected: Some(true),
                        line_ending: Some("".into()),
                        ..Default::default()
                    }),
                });
                let command = Command::from(settings);

                println!("Settings command to be sent: {command:?}");
                let command_json = serde_json::to_string(&command).unwrap();
                println!("Settings command JSON to be sent: {command_json:?}");

                match ws_sink.send(Message::text(command_json)).await {
                    Ok(_) => {}
                    Err(err) => eprintln!("Failed to set settings: {err}"),
                }
            }

            loop {
                let mut data = vec![];
                for _i in 0..6 {
                    let mut rng = rand::thread_rng();
                    let rand: u32 = rng.gen_range(0..100);
                    data.push(rand);
                }
                // let data = vec![format!("L1:{},")];
                // let data_command = Command {
                //     command: CommandName::Data,
                //     data: vec![format!("L1:{},L2:{},L3:{}\n", data[0], data[1], data[2])],
                // };
                let data1_str = format!("L1:{},L2:{},L3:{}\n", data[0], data[1], data[2]);
                let data2_str = format!("A:{},B:{},C:{}\n", data[3], data[4], data[5]);
                let data = vec![data1_str, data2_str];

                let json = serde_json::to_string(&data).unwrap();
                let send_result = ws_sink.send(Message::text(json.clone())).await;
                match send_result {
                    Ok(_) => println!("Sent data message: {json}"),
                    Err(err) => {
                        eprintln!("Sending data message failed: {err:?}");
                        if matches!(err, Error::AlreadyClosed | Error::Io(_)) {
                            // continue on the accept (http upgrade handshake step)
                            continue 'main;
                        }
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3030").await?;

    let listener = tokio::spawn(listen(listener));

    listener.await?;

    // Ok(())
}
