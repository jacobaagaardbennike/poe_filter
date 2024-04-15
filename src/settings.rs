use serde::{Deserialize, Serialize};
use crate::replace_colours::{ColourSettings};

#[derive(Debug, Deserialize, Serialize)]
pub enum SettingType {
    ColourSettings,
    // Add other setting types here if needed
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub name: String,
    pub setting_type: SettingType,
    pub colour_options: Option<ColourSettings>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub sets: Vec<Settings>
}