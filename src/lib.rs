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
pub struct Instance {
    pub deck: Deck,
}

impl Instance {
    pub fn new() -> Instance {
        Instance { deck: Deck::new() }
    }

    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn deserialize(from: &str) -> Result<Instance, toml::de::Error> {
        toml::from_str(from)
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}
