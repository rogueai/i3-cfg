use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub keybindings: Vec<Keybinding>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    #[serde(flatten)]
    pub variant: KeybindingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub modifiers: Vec<String>,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<String>,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum KeybindingType {
    Keysym,
    Keycode,
    Button,
    Unknown,
}

impl Default for KeybindingType {
    fn default() -> Self {
        KeybindingType::Unknown
    }
}
