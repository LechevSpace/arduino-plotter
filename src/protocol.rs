use std::{collections::HashMap, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

///
/// ```
/// use arduino_plotter::protocol::{CommandName, Command};
///
/// let data = CommandName::Data;
/// let data_json = serde_json::json!({ "command": "", "data": [1, 2, 3] });
/// let command = serde_json::from_value::<Command<Vec<u64>>>(data_json).expect("should be valid");
///
/// assert!(matches!(command.command, CommandName::Data));
///
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command<T> {
    pub command: CommandName,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandName {
    #[serde(rename = "")]
    Data,
    OnSettingsDidChange,
    SendMessage,
    ChangeSettings,
}

pub struct MiddlewareCommand(pub MonitorSettings);
impl From<MiddlewareCommand> for Command<MonitorSettings> {
    fn from(value: MiddlewareCommand) -> Self {
        Self {
            command: CommandName::OnSettingsDidChange,
            data: value.0,
        }
    }
}

pub enum ClientCommand {
    SendMessage {},
    ChangeSettings(MonitorSettings),
}
impl From<ClientCommand> for Command<serde_json::Value> {
    fn from(value: ClientCommand) -> Self {
        match value {
            ClientCommand::SendMessage {} => Self {
                command: CommandName::SendMessage,
                data: serde_json::to_value("TODO").unwrap(),
            },
            ClientCommand::ChangeSettings(monitor_settings) => Self {
                command: CommandName::ChangeSettings,
                data: serde_json::to_value(monitor_settings).unwrap(),
            },
        }
    }
}

///
/// ```text
/// let a = serde_json::json!({
/// pluggableMonitorSettings: {
///   baudrate: {
///     id: "baudrate",
///     label: "Baudrate",
///     type: "enum",
///     values: ["300","9600", "115200"],
///     selectedValue: "9600"
///   },
///   otherSetting: {
///     id: "otherSetting",
///     label: "Other Setting",
///     type: "enum",
///     values: ["A","B", "C"],
///     selectedValue: "B"
///   }
/// }
/// })
/// ```

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
///
/// PluggableMonitorSettings
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

pub const EOL: &[&str] = &["", "\n", "\r", "\r\n"];

/// # Examples
///
/// ```
/// use arduino_plotter::protocol::EndOfLine;
///
/// let no_line_ending = EndOfLine::NoLineEnding;
/// assert_eq!("", serde_json::to_value(&no_line_ending).unwrap().as_str().unwrap());
/// let new_line = EndOfLine::NewLine;
/// assert_eq!("\n", serde_json::to_value(&new_line).unwrap().as_str().unwrap());
/// let carriage_return = EndOfLine::CarriageReturn;
/// assert_eq!("\r", serde_json::to_value(&carriage_return).unwrap().as_str().unwrap());
/// let carriage_return_new_line = EndOfLine::CarriageReturnNewLine;
/// assert_eq!("\r\n", serde_json::to_value(&carriage_return_new_line).unwrap().as_str().unwrap());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndOfLine {
    #[serde(rename = "")]
    NoLineEnding,
    #[serde(rename = "\n")]
    NewLine,
    #[serde(rename = "\r")]
    CarriageReturn,
    #[serde(rename = "\r\n")]
    CarriageReturnNewLine,
}

pub fn is_eol(str: String) -> bool {
    EOL.iter().any(|eol| str.contains(eol))
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorModelState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscroll: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<bool>,
    // todo: use an enum?!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pluggable_monitor_settings: Option<PluggableMonitorSettings>,
    #[serde(rename = "monitorUISettings", skip_serializing_if = "Option::is_none")]
    pub monitor_ui_settings: Option<MonitorModelState>,
}

//   pub struct PluggableMonitor {
//     export namespace Protocol {
//       export enum ClientCommand {
//         SEND_MESSAGE = "SEND_MESSAGE",
//         CHANGE_SETTINGS = "CHANGE_SETTINGS",
//       }

//       export enum MiddlewareCommand {
//         ON_SETTINGS_DID_CHANGE = "ON_SETTINGS_DID_CHANGE",
//       }

//       export type ClientCommandMessage = {
//         command: ClientCommand;
//         data: Partial<MonitorSettings> | string;
//       };
//       type MiddlewareCommandMessage = {
//         command: MiddlewareCommand;
//         data: Partial<MonitorSettings>;
//       };
//       type DataMessage = string[];

//       export type Message =
//         | ClientCommandMessage
//         | MiddlewareCommandMessage
//         | DataMessage;

//       export function isClientCommandMessage(
//         message: Message
//       ): message is ClientCommandMessage {
//         return (
//           !Array.isArray(message) &&
//           typeof message.command === "string" &&
//           Object.keys(ClientCommand).includes(message.command)
//         );
//       }
//       export function isMiddlewareCommandMessage(
//         message: Message
//       ): message is MiddlewareCommandMessage {
//         return (
//           !Array.isArray(message) &&
//           typeof message.command === "string" &&
//           Object.keys(MiddlewareCommand).includes(message.command)
//         );
//       }
//       export function isDataMessage(message: Message): message is DataMessage {
//         return Array.isArray(message);
//       }
//     }
//   }

pub const LINE_COLORS: [&str; 8] = [
    "#0072B2", "#D55E00", "#009E73", "#E69F00", "#CC79A7", "#56B4E9", "#F0E442", "#95A5A6",
];

//   let existingDatasetsMap: {
//     [key: string]: ChartDataset<"line">;
//   } = {};

//   export const resetExistingDatasetsMap = () => {
//     existingDatasetsMap = {};
//   };
//   export const resetDatapointCounter = () => {
//     datapointCounter = 0;
//   };

//   export let datapointCounter = 0;

//   export const addDataPoints = (
//     parsedMessages: {
//       datasetNames: string[];
//       parsedLines: { [key: string]: number }[];
//     },
//     chart: ChartJSOrUndefined,
//     opts: ChartOptions<"line">,
//     cubicInterpolationMode: "default" | "monotone",
//     dataPointThreshold: number,
//     setForceUpdate: React.Dispatch<any>
//   ) => {
//     if (!chart) {
//       return;
//     }

//     // if the chart has been crated, can add data to it
//     if (chart && chart.data.datasets) {
//       const { datasetNames, parsedLines } = parsedMessages;

//       const existingDatasetNames = Object.keys(existingDatasetsMap);

//       // add missing datasets to the chart
//       existingDatasetNames.length < 8 &&
//         datasetNames.forEach((datasetName) => {
//           if (
//             !existingDatasetNames.includes(datasetName) &&
//             existingDatasetNames.length < 8
//           ) {
//             const newDataset = {
//               data: [],
//               label: datasetName,
//               borderColor: lineColors[existingDatasetNames.length],
//               backgroundColor: lineColors[existingDatasetNames.length],
//               borderWidth: 1,
//               pointRadius: 0,
//               cubicInterpolationMode,
//             };

//             existingDatasetsMap[datasetName] = newDataset;
//             chart.data.datasets.push(newDataset);
//             existingDatasetNames.push(datasetName);

//             // used to force a re-render in the parent component
//             setForceUpdate(existingDatasetNames.length);
//           }
//         });

//       // iterate every parsedLine, adding each variable to the corrisponding variable in the dataset
//       // if a dataset has not variable in the line, fill it with and empty value
//       parsedLines.forEach((parsedLine) => {
//         const xAxis =
//           opts.scales!.x?.type === "realtime" ? Date.now() : datapointCounter++;

//         // add empty values to datasets that are missing in the parsedLine
//         Object.keys(existingDatasetsMap).forEach((datasetName) => {
//           const newPoint =
//             datasetName in parsedLine
//               ? {
//                   x: xAxis,
//                   y: parsedLine[datasetName],
//                 }
//               : null;

//           newPoint && existingDatasetsMap[datasetName].data.push(newPoint);
//         });
//       });

//       const oldDataValue = datapointCounter - dataPointThreshold;
//       for (let s = 0; s < chart.data.datasets.length; s++) {
//         const dataset = chart.data.datasets[s];

//         let delCount = 0;
//         for (let i = 0; i < dataset.data.length; i++) {
//           if (dataset.data[i] && (dataset.data[i] as any).x < oldDataValue) {
//             delCount++;
//           } else {
//             dataset.data.splice(0, delCount);
//             break; // go to the next dataset
//           }

//           // purge the data if we need to remove all points
//           if (dataset.data.length === delCount) {
//             // remove the whole dataset from the chart and the map
//             delete existingDatasetsMap[dataset.label!];
//             chart.data.datasets.splice(s, 1);
//             setForceUpdate(-1);
//           }
//         }
//       }
//       chart.update();
//     }
//   };
