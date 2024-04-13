use std::{collections::HashMap, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    name: String,
    colors: HashMap<String, Vec<u8>>, // Changed to use Vec<u8> for color values
    alert_sound: [u16; 2],
    effect: String,
    minimap_icon: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    color_codes: HashMap<String, Vec<u8>>, // Changed to use Vec<u8> for color codes
    sets: Vec<Settings>,
}

fn replace_colours(config: &Config, mut filter_text: String) -> Result<String, Box<dyn Error>> {
    // Iterate over each settings set and replace placeholders in the base filter text
    for settings in config.sets.iter() {
        let placeholder = format!("${}", settings.name);
        let replacement_text = format!(
            "SetFontSize 45\n  SetTextColor {} {} {}\n  SetBorderColor {} {} {}\n  SetBackgroundColor {} {} {}\n  PlayAlertSound {} {}\n  PlayEffect {}\n  MinimapIcon {} {}",
            settings.colors["color_text"][0], settings.colors["color_text"][1], settings.colors["color_text"][2], // Text color
            settings.colors["color_border"][0], settings.colors["color_border"][1], settings.colors["color_border"][2], // Border color
            settings.colors["color_background"][0], settings.colors["color_background"][1], settings.colors["color_background"][2], // Background color
            settings.alert_sound[0],
            settings.alert_sound[1],
            settings.effect,
            settings.minimap_icon[0],
            settings.minimap_icon[1],
        );

        // Replace the placeholder in the base filter file with the replacement text
        filter_text = filter_text.replace(&placeholder, &replacement_text);
    }
    Ok(filter_text)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read settings from the configuration file
    let config_text = std::fs::read_to_string("colours.yaml")?;
    let config: Config = serde_yaml::from_str(&config_text)?;

    // Read the base filter file
    let filter_text = std::fs::read_to_string("base.filter")?;

    // Replace placeholders in the base filter text
    let modified_filter_text = replace_colours(&config, filter_text)?;

    // Write the modified filter text to a new file
    std::fs::write("modified_filter.filter", modified_filter_text)?;

    Ok(())
}
