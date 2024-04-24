use serde::Deserialize;
use std::error::Error;
use toml::from_str;

#[derive(Debug, Deserialize)]
struct Card {
    name: String,
    reward: String,
    drop_level: u8,
    info: String,
}

#[derive(Debug, Deserialize)]
struct Cards {
    cards_favorites: Vec<Card>,
    cards_good: Vec<Card>,
    cards_hide: Vec<Card>,
}

pub fn get_cards_filter(config_raw_string: &str) -> Result<(), Box<dyn Error>> {
    let cards: Cards = from_str(&config_raw_string)?;
    dbg!(cards.cards_favorites.get(2));
    Ok(())
}
