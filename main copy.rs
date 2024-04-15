use std::{fs, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum AlertSound {
    Sound([u16; 2]),
    Word(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct MinimapIcon {
    size: u8,
    colour: String,
    shape: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Colour {
    #[serde(rename = "0")]
    r: u8,
    #[serde(rename = "1")]
    g: String,
    #[serde(rename = "2")]
    b: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum SettingType {
    Colour,
    // Add other setting types here if needed
}

#[derive(Debug, Deserialize, Serialize)]
struct ColourSettings {
    font_size: u8,
    text_colour: Colour,
    border_colour: Colour,
    background_colour: Colour,
    alert_sound: AlertSound,
    effect: String,
    minimap_icon: MinimapIcon,
}

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    name: String,
    setting_type: SettingType,
    colour_options: Option<ColourSettings>
}


#[derive(Debug, Deserialize, Serialize)]
struct Config {
    sets: Vec<Settings>,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Settings {
//     name: String,
//     setting_type: SettingType,
//     font_size: u8,
//     text_colour: Colour,
//     border_colour: Colour,
//     background_colour: Colour,
//     alert_sound: AlertSound,
//     effect: String,
//     minimap_icon: MinimapIcon,
// }


fn replace_colours(config: &Config, mut filter_text: String) -> Result<String, Box<dyn Error>> {
    // Iterate over each settings set and replace placeholders in the base filter text
    for settings in config.sets.iter() {
        let placeholder = format!("${}", settings.name);
        let replacement_text = match settings.setting_type {
            SettingType::Colour => {
                if let Some(colour_options) = &settings.colour_options {
                    format!(
                        "SetFontSize {}\n  SetTextColor {} {} {}\n  SetBorderColor {} {} {}\n  SetBackgroundColor {} {} {}\n  PlayAlertSound {}\n  PlayEffect {}\n  MinimapIcon {} {} {}",
                        colour_options.font_size,
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
                } else {
                    "".to_string() // handle None case
                }
            }
            // Handle other setting types if needed
        };

        // Replace the placeholder in the base filter file with the replacement text
        filter_text = filter_text.replace(&placeholder, &replacement_text);
    }
    Ok(filter_text)
}



fn load_yaml_files(paths: &[&str]) -> Result<Config, Box<dyn std::error::Error>> {
    let mut sets = Vec::new();
    for path in paths {
        let yaml_content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&yaml_content)?;
        sets.extend(config.sets);
    }
    Ok(Config { sets })
}

fn main() -> Result<(), Box<dyn Error>> {

    let yaml_paths = vec!["./config/colours.yaml"];//, "./config/loot.yaml"];
    //let config_value = load_yaml_files(&yaml_paths)?;
    //let config: Config = serde_yaml::from_value(config_value)?;
    let config: Config = load_yaml_files(&yaml_paths)?;
    //dbg!(&config);

    // Read the base filter file
    let filter_text = std::fs::read_to_string("base.filter")?;

    // Replace placeholders in the base filter text
    let modified_filter_text = replace_colours(&config, filter_text)?;

    // Write the modified filter text to a new file
    std::fs::write("modified_filter.filter", modified_filter_text)?;

    Ok(())
}
