use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub keybindings: Vec<Keybinding>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub modifiers: Vec<String>,
    #[serde(flatten)]
    pub variant: KeybindingVariant,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum KeybindingVariant {
    Keysym { key: String },
    Keycode { code: u8 },
    Unknown,
}

impl Default for KeybindingVariant {
    fn default() -> Self {
        KeybindingVariant::Unknown
    }
}
