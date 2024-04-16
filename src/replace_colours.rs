use serde::{Deserialize, Serialize};
use crate::settings::{Settings};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlertSound {
    Sound([u16; 2]),
    Word(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinimapIcon {
    pub size: u8,
    pub colour: String,
    pub shape: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColourSettings {
    pub text_colour: Colour,
    pub border_colour: Colour,
    pub background_colour: Colour,
    pub alert_sound: AlertSound,
    pub effect: String,
    pub minimap_icon: MinimapIcon,
}

pub fn format_colours(settings: &Settings) -> String {
    let colour_options = settings.colour_options.as_ref().expect("No colour options provided");
    format!(
        "SetFontSize {}\n  SetTextColor {} {} {}\n  SetBorderColor {} {} {}\n  SetBackgroundColor {} {} {}\n  PlayAlertSound {}\n  PlayEffect {}\n  MinimapIcon {} {} {}",
        settings.font_size,
        colour_options.text_colour.r,
        colour_options.text_colour.g,
        colour_options.text_colour.b,
        colour_options.border_colour.r,
        colour_options.border_colour.g,
        colour_options.border_colour.b,
        colour_options.background_colour.r,
        colour_options.background_colour.g,
        colour_options.background_colour.b,
        match &colour_options.alert_sound { 
            AlertSound::Sound(sound) => format!("[{}, {}]", sound[0], sound[1]), 
            AlertSound::Word(word) => word.clone(), // Clone the string 
        }, 
        colour_options.effect,
        colour_options.minimap_icon.size,
        colour_options.minimap_icon.colour,
        colour_options.minimap_icon.shape,
    )
}
