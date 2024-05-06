//! Change settings of the Arduino Plotter

use arduino_plotter::protocol::{Command, MiddlewareCommand, MonitorModelState, MonitorSettings};
use futures_util::SinkExt;
use http::Uri;
use tokio_websockets::{ClientBuilder, Error, Message};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let uri = Uri::from_static("ws://localhost:3000");

    let (mut client, _) = ClientBuilder::from_uri(uri).connect().await?;

    let settings = MiddlewareCommand(MonitorSettings {
        pluggable_monitor_settings: None,
        monitor_ui_settings: Some(MonitorModelState {
            dark_theme: Some(true),
            ..Default::default()
        }),
    });
    let command = Command::from(settings);

    println!("Settings command to be sent: {command:?}");
    let command_json = serde_json::to_string(&command).unwrap();
    println!("Settings command JSON to be sent: {command_json:?}");

    match client.send(Message::text(command_json)).await {
        Ok(_) => {}
        Err(err) => eprintln!("Failed to set settings: {err}"),
    }

    Ok(())
}
