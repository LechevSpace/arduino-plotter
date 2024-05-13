use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use parse_display::{Display, FromStr};

/// The generic Command structure defined by the Arduino serial plotter README.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command<T> {
    pub command: CommandName,
    pub data: T,
}

/// Data lines message that can be send to the Arduino serial plotter
///
/// <https://docs.arduino.cc/software/ide-v2/tutorials/ide-v2-serial-plotter>
///
/// ```
/// use arduino_plotter::protocol::Data;
///
/// // data with no line ending
/// let data = Data(vec![
///     "L1:1,L2:2,L3:3,L4:4".to_string(),
///     "Label_1:99,Label_2:98,Label_3:97,Label_4:96".to_string(),
/// ]);
/// let data_json = serde_json::json!([
///     "L1:1,L2:2,L3:3,L4:4",
///     "Label_1:99,Label_2:98,Label_3:97,Label_4:96"
/// ]);
/// let from_json = serde_json::from_value::<Data<String>>(data_json).expect("should be valid");
///
/// assert_eq!(data, from_json);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Data<T: core::fmt::Display>(pub Vec<T>);

/// All the available Command names for both Client ([`ClientCommand`]) and Middleware ([`MiddlewareCommand`]).
#[derive(Debug, Clone, Serialize, Deserialize, Display, FromStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[display(style = "SNAKE_CASE")]
pub enum CommandName {
    /// Middleware Command (from WebSocket to Arduino Serial Plotter UI)
    OnSettingsDidChange,
    // Client Command (from Arduino Serial Plotter UI to WebSocket)
    SendMessage,
    // Client Command (from Arduino Serial Plotter UI to WebSocket)
    ChangeSettings,
}

/// Middleware Command (from WebSocket to Arduino Serial Plotter UI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    into = "Command<MonitorSettings>",
    try_from = "Command<MonitorSettings>"
)]
pub struct MiddlewareCommand(pub MonitorSettings);
impl From<MiddlewareCommand> for Command<MonitorSettings> {
    fn from(value: MiddlewareCommand) -> Self {
        Self {
            command: CommandName::OnSettingsDidChange,
            data: value.0,
        }
    }
}

impl TryFrom<Command<MonitorSettings>> for MiddlewareCommand {
    type Error = serde_json::Error;

    fn try_from(middleware_command: Command<MonitorSettings>) -> Result<Self, Self::Error> {
        match middleware_command.command {
            CommandName::OnSettingsDidChange => Ok(MiddlewareCommand(middleware_command.data)),
            command_name => Err(serde::de::Error::custom(format!(
                "ON_SETTING_DID_CHANGE command expected, got {command_name}"
            ))),
        }
    }
}

/// Client Commands from Arduino Serial Plotter UI to WebSocket)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientCommand {
    SendMessage(String),
    ChangeSettings(MonitorSettings),
}

impl From<ClientCommand> for Command<serde_json::Value> {
    fn from(value: ClientCommand) -> Self {
        match value {
            ClientCommand::SendMessage(send_message) => Self {
                command: CommandName::SendMessage,
                data: serde_json::to_value(send_message).unwrap(),
            },
            ClientCommand::ChangeSettings(monitor_settings) => Self {
                command: CommandName::ChangeSettings,
                data: serde_json::to_value(monitor_settings).unwrap(),
            },
        }
    }
}

/// A single Pluggable monitor setting
/// ```json
/// {
///     "id": "baudrate",
///     "label": "Baudrate",
///     "type": "enum",
///     "values": ["300","9600", "115200"],
///     "selectedValue": "9600",
///   }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PluggableMonitorSetting {
    /// The setting identifier, e.g. `"baudrate"`
    pub id: Option<String>,
    /// A human-readable label of the setting (to be displayed on the GUI), e.g. `"Baudrate"`
    pub label: Option<String>,
    /// The setting type (at the moment only "enum" is available)
    pub r#type: Option<LabelType>,
    /// The values allowed on "enum" types, e.g. `vec!["300".to_string(), "9600".into(), "115200".into()]`
    #[serde(default)]
    pub values: Vec<String>,
    /// The selected value, e.g. `"9600"`
    pub selected_value: String,
}

/// The Pluggable Monitor setting type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LabelType {
    Enum,
}

/// All the Pluggable Monitor settings, i.e. a connected serial device,
/// that can be changed from the Arduino serial plotter UI.
///
/// ```
/// use arduino_plotter::protocol::PluggableMonitorSettings;
///
/// let json = serde_json::json!({
///   "baudrate": {
///     "id": "baudrate",
///     "label": "Baudrate",
///     "type": "enum",
///     "values": ["300","9600", "115200"],
///     "selectedValue": "9600",
///   },
///   "otherSetting": {
///     "id": "otherSetting",
///     "label": "Other Setting",
///     "type": "enum",
///     "values": ["A","B", "C"],
///     "selectedValue": "B",
///   }
/// });
///
/// let settings = serde_json::from_value::<PluggableMonitorSettings>(json).expect("Valid PluggableMonitorSettings");
///
/// assert_eq!(2, settings.len());
/// assert!(settings.contains_key("baudrate"));
/// assert!(settings.contains_key("otherSetting"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct PluggableMonitorSettings(pub HashMap<String, PluggableMonitorSetting>);

impl Deref for PluggableMonitorSettings {
    type Target = HashMap<String, PluggableMonitorSetting>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PluggableMonitorSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// All possible End Of Line values accepted by the Arduino Serial Plotter UI
///
/// # Examples
///
/// ```
/// use arduino_plotter::protocol::EndOfLine;
///
/// let no_line_ending = EndOfLine::NoLineEnding;
/// assert_eq!(
///     "",
///     serde_json::to_value(&no_line_ending)
///         .unwrap()
///         .as_str()
///         .unwrap()
/// );
/// assert_eq!("", &no_line_ending.to_string());
///
/// let new_line = EndOfLine::NewLine;
/// assert_eq!(
///     "\n",
///     serde_json::to_value(&new_line).unwrap().as_str().unwrap()
/// );
/// assert_eq!("\n", &new_line.to_string());
///
/// let carriage_return = EndOfLine::CarriageReturn;
/// assert_eq!(
///     "\r",
///     serde_json::to_value(&carriage_return)
///         .unwrap()
///         .as_str()
///         .unwrap()
/// );
/// assert_eq!("\r", &carriage_return.to_string());
///
/// let carriage_return_new_line = EndOfLine::CarriageReturnNewLine;
/// assert_eq!(
///     "\r\n",
///     serde_json::to_value(&carriage_return_new_line)
///         .unwrap()
///         .as_str()
///         .unwrap()
/// );
/// assert_eq!("\r\n", &carriage_return_new_line.to_string());
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display, FromStr)]
pub enum EndOfLine {
    #[display("")]
    #[serde(rename = "")]
    NoLineEnding,
    #[display("\n")]
    #[serde(rename = "\n")]
    NewLine,
    #[display("\r")]
    #[serde(rename = "\r")]
    CarriageReturn,
    #[display("\r\n")]
    #[serde(rename = "\r\n")]
    CarriageReturnNewLine,
}

impl EndOfLine {
    /// A list of all the EndOfLine values as strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use arduino_plotter::protocol::EndOfLine;
    ///
    /// let all = &[
    ///     EndOfLine::NoLineEnding.to_string(),
    ///     EndOfLine::NewLine.to_string(),
    ///     EndOfLine::CarriageReturn.to_string(),
    ///     EndOfLine::CarriageReturnNewLine.to_string(),
    /// ];
    ///
    /// assert_eq!(EndOfLine::EOL, all);
    /// ```
    pub const EOL: &'static [&'static str] = &["", "\n", "\r", "\r\n"];

    /// Whether a string contains any of the EndOfLine values inside of it.
    pub fn contains_eol(string: String) -> bool {
        Self::EOL.iter().any(|eol| string.contains(eol))
    }
}

/// All the UI Monitor settings that can be changed in the Arduino serial
/// plotter application.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorModelState {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Used by the serial monitors to stick at the bottom of the window.
    pub autoscroll: Option<bool>,
    /// Enable timestamp next to the actual data used by the serial monitors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<bool>,
    /// Clients store the information about the last EOL used when sending a message to the board.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<EndOfLine>,
    /// Enables interpolation of the chart in the Serial Plotter App.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<bool>,
    // Whether to enable Dark theme or stick to the Light theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_theme: Option<bool>,
    /// the current websocket port where the communication happens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ws_port: Option<u16>,
    /// The port at which the pluggable monitor in the middleware is connected to,
    /// e.g. `/dev/ttyACM0` (linux), `/dev/ttyUSB0` (linux), etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The connection status of the pluggable monitor to the actual board.
    pub connected: Option<bool>,
    /// Enable mocked data generation.
    #[serde(default)]
    pub generate: bool,
}

/// The [`MiddlewareCommand`] Monitor settings that are sent to the
/// Arduino serial plotter UI.
/// This contains both [`PluggableMonitorSettings`] and [`MonitorModelState`].
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pluggable_monitor_settings: Option<PluggableMonitorSettings>,
    #[serde(
        default,
        rename = "monitorUISettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor_ui_settings: Option<MonitorModelState>,
}
