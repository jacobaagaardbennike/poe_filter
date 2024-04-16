use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use toml::{from_str, Value};

#[derive(Debug, Deserialize)]
struct Config {
    colours: HashMap<String, [u8; 3]>,
    font_sizes: HashMap<String, u8>,
}

struct Epic {
    font_size: String,
    text_colour: String,
    border_colour: String,
    background_colour: String,
}

struct Filter {
    epic: Epic,
}

fn load_config(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_content = read_to_string(config_file_path)?;
    let config: Config = from_str(&config_content)?;
    Ok(config)
}

fn load_filters_as_string(paths: &[&str]) -> Result<String, Box<dyn Error>> {
    let combined_contents: String = paths
        .iter()
        .map(|&path| read_to_string(path))
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");
    Ok(combined_contents)
}

fn load_filters(paths: &[&str]) -> Result<String, Box<dyn Error>> {
    let combined_contents: String = paths
        .iter()
        .map(|&path| read_to_string(path))
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");
    Ok(combined_contents)
}

fn replace_variables(
    _config_file_path: &str,
    _filter_content_raw: &str,
) -> Result<String, Box<dyn Error>> {
    let tmp = "tmp".to_string();
    Ok(tmp)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = "./config/config.toml";
    let config = load_config(config_file_path)?;
    dbg!(&config);
    let filter_file_paths = vec!["./config/filter.toml"]; //, "./config/loot.yaml"];
    let filter_content_raw: String = load_filters(&filter_file_paths)?;
    let filter_content = replace_variables(config_file_path, &filter_content_raw)?;
    dbg!(&filter_content);
    Ok(())
}
