use std::{fs, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    name: String,
    color_text: [u8; 3],
    color_border: [u8; 3],
    color_background: [u8; 3],
    alert_sound: [u16; 2],
    effect: String,
    minimap_icon: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    sets: Vec<Settings>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read settings from the configuration file
    let config_text = fs::read_to_string("colours.yaml")?;
    let config: Config = serde_yaml::from_str(&config_text)?;

    // Read the base filter file
    let mut filter_text = fs::read_to_string("base.filter")?;

    // Iterate over each settings set and replace placeholders in the base filter text
    for settings in config.sets.iter() {
        let placeholder = format!("${}", settings.name);
        let replacement_text = format!(
          "SetFontSize 45
  SetTextColor {} {} {}
  SetBorderColor {} {} {}
  SetBackgroundColor {} {} {}
  PlayAlertSound {} {}
  PlayEffect {}
  MinimapIcon {} {}",
          settings.color_text[0], settings.color_text[1], settings.color_text[2], // Text color
          settings.color_border[0], settings.color_border[1], settings.color_border[2], // Border color
          settings.color_background[0], settings.color_background[1], settings.color_background[2], // Background color
          settings.alert_sound[0], settings.alert_sound[1], // Alert sound
          settings.effect, // Effect
          settings.minimap_icon[0], settings.minimap_icon[1] // Minimap icon
      );

        // Replace the placeholder in the base filter file with the replacement text
        filter_text = filter_text.replace(&placeholder, &replacement_text);
    }

    // Write the modified filter text to a new file
    fs::write("modified_filter.filter", filter_text)?;

    Ok(())
}
