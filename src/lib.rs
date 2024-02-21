pub mod card;
pub mod deck;
pub mod shuffle;
pub mod spread;

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde;

use crate::deck::Deck;

#[derive(Serialize, Deserialize)]
pub struct TarotInstance {
    pub deck: Deck,
}

impl TarotInstance {
    pub fn new() -> TarotInstance {
        TarotInstance { deck: Deck::new() }
    }

    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn deserialize(from: &str) -> Result<TarotInstance, toml::de::Error> {
        toml::from_str(from)
    }
}

impl Default for TarotInstance {
    fn default() -> Self {
        Self::new()
    }
}
