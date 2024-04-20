use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_to_string, write};
use toml::{from_str, to_string};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SetupVariableType {
    Colour([u8; 3]),
    FontSize(u8),
    Icon(String),
}

#[derive(Debug, Deserialize)]
struct Setup {
    variables: HashMap<String, SetupVariableType>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MinimapIcon {
    size: u8,
    colour: String,
    shape: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Loot {
    name: String,
    SetFontSize: u8,
    SetTextColor: [u8; 3],
    SetBorderColor: [u8; 3],
    SetBackgroundColor: [u8; 3],
    PlayAlertSound: [u16; 2],
    PlayEffect: String,
    MinimapIcon: MinimapIcon,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    loots: Vec<Loot>,
}

fn replace_variables(setup: &Setup, config_raw: &str) -> Result<Config, Box<dyn Error>> {
    let mut modified_config_content = config_raw.to_string();

    // Iterate over variables and replace placeholders in the filter content
    for (key, var) in setup.variables.iter() {
        let placeholder = format!("\"&{}\"", key);
        let value_str = match var {
            SetupVariableType::Colour(rgb) => format!("{:?}", rgb),
            SetupVariableType::FontSize(size) => size.to_string(),
            SetupVariableType::Icon(icon) => format!("\"{}\"", icon),
        };
        // Replace placeholder with the value directly
        modified_config_content = modified_config_content.replace(&placeholder, &value_str);
    }
    //dbg!(&modified_config_content);
    let config: Config = from_str(&modified_config_content)?;
    Ok(config)
}

fn get_config(config_raw: &str) -> Result<Config, Box<dyn Error>> {
    //    dbg!(&config_raw);
    let setup: Setup = from_str(&config_raw)?;
    let config = replace_variables(&setup, &config_raw)?;
    Ok(config)
}

fn save_filter(filter_final: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    write(output_file_path, filter_final)?;
    Ok(())
}

fn save_config(config: &Config, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let toml_string = to_string(&config)?;
    write(output_file_path, toml_string)?;
    Ok(())
}

fn load_config(config_file_path: &str) -> Result<String, Box<dyn Error>> {
    let config_as_string = read_to_string(config_file_path)?;
    Ok(config_as_string)
}

fn load_filter(filter_file_path: &str) -> Result<String, Box<dyn Error>> {
    let filter_as_string = read_to_string(filter_file_path)?;
    Ok(filter_as_string)
}

fn replace_filter_variables(
    config: &Config,
    filter_as_string: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut base_filter_content = filter_as_string.to_string();
    // Iterate over the loots vector
    for loot in &config.loots {
        // Construct filter rules
        let filter_rule = format!(
            r#"SetFontSize {}
  SetTextColor {} {} {}
  SetBorderColor {} {} {}
  SetBackgroundColor {} {} {}
  PlayAlertSound {} {}
  PlayEffect {}
  MinimapIcon {} {} {}"#,
            loot.SetFontSize,
            loot.SetTextColor[0],
            loot.SetTextColor[1],
            loot.SetTextColor[2],
            loot.SetBorderColor[0],
            loot.SetBorderColor[1],
            loot.SetBorderColor[2],
            loot.SetBackgroundColor[0],
            loot.SetBackgroundColor[1],
            loot.SetBackgroundColor[2],
            loot.PlayAlertSound[0],
            loot.PlayAlertSound[1],
            loot.PlayEffect,
            loot.MinimapIcon.size,
            loot.MinimapIcon.colour,
            loot.MinimapIcon.shape
        );

        base_filter_content = base_filter_content.replace(&format!("${}", loot.name), &filter_rule);
    }
    Ok(base_filter_content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = "./config/config.toml";
    let filter_file_path = "./filter/base.filter";
    let output_file_path = "./filter/new.filter";

    let config_raw = load_config(&config_file_path)?;
    let config = get_config(&config_raw)?;
    let filter_raw = load_filter(&filter_file_path)?;
    let filter_final = replace_filter_variables(&config, &filter_raw)?;
    save_filter(&filter_final, output_file_path)?;
    // save_config(&config, output_file_path)?;
    Ok(())
}
