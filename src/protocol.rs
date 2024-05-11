use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use parse_display::{Display, FromStr};

pub const LINE_COLORS: [&str; 8] = [
    "#0072B2", "#D55E00", "#009E73", "#E69F00", "#CC79A7", "#56B4E9", "#F0E442", "#95A5A6",
];
pub const EOL: &[&str] = &["", "\n", "\r", "\r\n"];

pub fn is_eol(str: String) -> bool {
    EOL.iter().any(|eol| str.contains(eol))
}

pub trait IntoCommand {
    fn into_command(&self) -> serde_json::Value;
}

impl IntoCommand for MiddlewareCommand {
    fn into_command(&self) -> serde_json::Value {
        let inner_command = Command::<MonitorSettings>::from(self.clone());

        serde_json::to_value(inner_command).unwrap()
    }
}

impl IntoCommand for ClientCommand {
    fn into_command(&self) -> serde_json::Value {
        let inner_command = Command::<serde_json::Value>::from(self.clone());

        serde_json::to_value(inner_command).unwrap()
    }
}

impl<T> IntoCommand for Data<T>
where
    T: Serialize + core::fmt::Display,
{
    fn into_command(&self) -> serde_json::Value {
        serde_json::to_value(
            self.0
                .iter()
                .map(|display| display.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command<T> {
    pub command: CommandName,
    pub data: T,
}

/// Data lines message that can be send to Arduino serial plotter
///
/// <https://docs.arduino.cc/software/ide-v2/tutorials/ide-v2-serial-plotter>
///
/// ```
/// use arduino_plotter::protocol::Data;
///
/// // data with no line ending
/// let data = Data(vec!["L1:1,L2:2,L3:3,L4:4".to_string(), "Label_1:99,Label_2:98,Label_3:97,Label_4:96".to_string()]);
/// let data_json = serde_json::json!(["L1:1,L2:2,L3:3,L4:4", "Label_1:99,Label_2:98,Label_3:97,Label_4:96"]);
/// let from_json = serde_json::from_value::<Data<String>>(data_json).expect("should be valid");
///
/// assert_eq!(data, from_json);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Data<T: core::fmt::Display>(pub Vec<T>);

#[derive(Debug, Clone, Serialize, Deserialize, Display, FromStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[display(style = "SNAKE_CASE")]
pub enum CommandName {
    /// Middleware Command (from WS to `arduino-serial-plotter`)
    OnSettingsDidChange,
    // Client Command (from `arduino-serial-plotter` to WS)
    SendMessage,
    // Client Command (from `arduino-serial-plotter` to WS)
    ChangeSettings,
}

/// Middleware Command (from WS to `arduino-serial-plotter`)
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

// Client Command (from `arduino-serial-plotter` to WS)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PluggableMonitorSetting {
    // The setting identifier
    pub id: Option<String>,
    // A human-readable label of the setting (to be displayed on the GUI)
    pub label: Option<String>,
    // The setting type (at the moment only "enum" is available)
    pub r#type: Option<LabelType>,
    // The values allowed on "enum" types
    #[serde(default)]
    pub values: Vec<String>,
    // The selected value
    pub selected_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LabelType {
    Enum,
}

//   type PluggableMonitorSettings = Record<"baudrate", PluggableMonitorSetting>;
/// PluggableMonitorSettings
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
/// assert!(settings.get("baudrate").is_some());
/// assert!(settings.get("otherSetting").is_some());
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

/// # Examples
///
/// ```
/// use arduino_plotter::protocol::EndOfLine;
///
/// let no_line_ending = EndOfLine::NoLineEnding;
/// assert_eq!("", serde_json::to_value(&no_line_ending).unwrap().as_str().unwrap());
/// assert_eq!("", &no_line_ending.to_string());
///
/// let new_line = EndOfLine::NewLine;
/// assert_eq!("\n", serde_json::to_value(&new_line).unwrap().as_str().unwrap());
/// assert_eq!("\n", &new_line.to_string());
///
/// let carriage_return = EndOfLine::CarriageReturn;
/// assert_eq!("\r", serde_json::to_value(&carriage_return).unwrap().as_str().unwrap());
/// assert_eq!("\r", &carriage_return.to_string());
///
/// let carriage_return_new_line = EndOfLine::CarriageReturnNewLine;
/// assert_eq!("\r\n", serde_json::to_value(&carriage_return_new_line).unwrap().as_str().unwrap());
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorModelState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscroll: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<EndOfLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_theme: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ws_port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(default)]
    pub generate: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pluggable_monitor_settings: Option<PluggableMonitorSettings>,
    #[serde(default, rename = "monitorUISettings", skip_serializing_if = "Option::is_none")]
    pub monitor_ui_settings: Option<MonitorModelState>,
}
