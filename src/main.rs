use std::error::Error;
use std::fs::{read_to_string, write};

mod cards;
mod config;

fn load_filter(filter_file_path: &str) -> Result<String, Box<dyn Error>> {
    let filter_as_string = read_to_string(filter_file_path)?;
    Ok(filter_as_string)
}

fn save_filter(filter_final: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    write(output_file_path, filter_final)?;
    Ok(())
}

fn load_config(config_file_path: &str) -> Result<String, Box<dyn Error>> {
    let config_as_string = read_to_string(config_file_path)?;
    Ok(config_as_string)
}

fn main() -> Result<(), Box<dyn Error>> {
    let base_filter_file_path = "./filter/base.filter";
    let output_file_path = "./filter/new.filter";
    let base_config_file_path = "./config/config.toml";
    let config_raw_string = load_config(&base_config_file_path)?;
    let base_filter_string = load_filter(&base_filter_file_path)?;
    let filter_final = config::get_config_filter(&config_raw_string, &base_filter_string)?;
    save_filter(&filter_final, &output_file_path)?;
    let cards_file_path = "./config/cards.toml";
    let cards_config = load_config(&cards_file_path)?;
    cards::get_cards_filter(&cards_config)?;
    Ok(())
}
