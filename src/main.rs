use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use toml::from_str;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Variable {
    Colour([u8; 3]),
    FontSize(u8),
}

#[derive(Debug, Deserialize)]
struct Config {
    variables: HashMap<String, Variable>,
}

#[derive(Debug, Deserialize)]
struct Epic {
    font_size: u8,
    text_colour: [u8; 3],
    border_colour: [u8; 3],
    background_colour: [u8; 3],
}

#[derive(Debug, Deserialize)]
struct Filter {
    epic: Epic,
}

fn load_config(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_content = read_to_string(config_file_path)?;
    dbg!(&config_content);
    let config: Config = from_str(&config_content)?;
    Ok(config)
}

fn load_filters(paths: &[&str]) -> Result<String, Box<dyn Error>> {
    let combined_contents: String = paths
        .iter()
        .map(|&path| read_to_string(path))
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");
    Ok(combined_contents)
}

fn replace_variables(config: &Config, filter_content_raw: &str) -> Result<Filter, Box<dyn Error>> {
    let mut modified_filter_content = filter_content_raw.to_string();

    // Iterate over variables and replace placeholders in the filter content
    for (key, var) in config.variables.iter() {
        let placeholder = format!("\"&{}\"", key);
        let value_str = match var {
            Variable::Colour(rgb) => format!("{:?}", rgb),
            Variable::FontSize(size) => size.to_string(),
        };
        // dbg!(&value_str);
        // Replace placeholder with the value directly
        modified_filter_content = modified_filter_content.replace(&placeholder, &value_str);
    }

    dbg!(&modified_filter_content);
    // Deserialize the modified filter content into the Filter struct
    let filter: Filter = from_str(&modified_filter_content)?;
    Ok(filter)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = "./config/config.toml";
    let config = load_config(config_file_path)?;
    let filter_file_paths = vec!["./config/filter.toml"]; //, "./config/loot.yaml"];
    let filter_content_raw: String = load_filters(&filter_file_paths)?;
    let filter_content = replace_variables(&config, &filter_content_raw)?;
    dbg!(&filter_content);
    Ok(())
}
