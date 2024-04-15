use crate::settings::{Config, SettingType};
use crate::replace_colours::{format_colours, ColourSettings};
use std::{fs, error::Error};

mod settings;
mod replace_colours;

fn replace_filter_variables(config: &Config, mut filter_text: String) -> Result<String, Box<dyn Error>> {
  // Iterate over each settings set and replace placeholders in the base filter text
  for settings in config.sets.iter() {
    let placeholder = format!("${}", settings.name);
    let replacement_text = match settings.setting_type {
      SettingType::ColourSettings => {
        if let Some(colour_options) = &settings.colour_options {
          format_colours(colour_options)
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
  let mut combined_yaml = String::new();
  // Read all YAML files and combine their contents
  for path in paths {
      let yaml_content = fs::read_to_string(path)?;
      combined_yaml.push_str(&yaml_content);
      combined_yaml.push('\n'); // Add newline between YAML files
  }
  // Deserialize the combined YAML into a Config struct
  let config: Config = serde_yaml::from_str(&combined_yaml)?;
  Ok(config)
}

/// Main entry point of the program.
fn main() -> Result<(), Box<dyn Error>> {
  let yaml_paths = vec!["./config/config.yaml", "./config/filter.yaml"];//, "./config/loot.yaml"];
  let config: Config = load_yaml_files(&yaml_paths)?;
  dbg!(&config);

    let filter_text = std::fs::read_to_string("./filter/base.filter")?;
    let modified_filter_text = replace_filter_variables(&config, filter_text)?;

    std::fs::write("./modified_filter.filter", modified_filter_text)?;

  Ok(())
}
