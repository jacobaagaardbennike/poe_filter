use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_to_string, write};
use toml::{from_str, to_string};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConfigVariable {
    Colour([u8; 3]),
    FontSize(u8),
}

#[derive(Debug, Deserialize)]
struct ConfigVariables {
    config: HashMap<String, ConfigVariable>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Epic {
    font_size: u8,
    text_colour: [u8; 3],
    border_colour: [u8; 3],
    background_colour: [u8; 3],
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    epic: Epic,
}

fn replace_variables(
    config_variables: &ConfigVariables,
    config_raw: &str,
) -> Result<Config, Box<dyn Error>> {
    let mut modified_config_content = config_raw.to_string();

    // Iterate over variables and replace placeholders in the filter content
    for (key, var) in config_variables.config.iter() {
        let placeholder = format!("\"&{}\"", key);
        let value_str = match var {
            ConfigVariable::Colour(rgb) => format!("{:?}", rgb),
            ConfigVariable::FontSize(size) => size.to_string(),
        };
        // Replace placeholder with the value directly
        modified_config_content = modified_config_content.replace(&placeholder, &value_str);
    }
    let config: Config = from_str(&modified_config_content)?;
    Ok(config)
}

fn get_config(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_as_string = read_to_string(config_file_path)?;
    let config_variables: ConfigVariables = from_str(&config_as_string)?;
    let config = replace_variables(&config_variables, &config_as_string)?;
    Ok(config)
}

fn save_config(config: &Config, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let toml_string = to_string(&config)?;
    write(output_file_path, toml_string)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = "./config/config.toml";
    let output_file_path = "./config/new_config.toml";
    let config = get_config(config_file_path)?;
    save_config(&config, output_file_path)?;
    Ok(())
}
