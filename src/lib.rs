pub mod card;
pub mod spread;
pub mod deck;

use crate::deck::Deck;
use serde::{Deserialize, Serialize};
use toml::de::Error;

#[macro_export]
macro_rules! enum_try_from {
    (
        #[repr($T: ident)]
        $( #[$meta: meta] )*
        $vis: vis enum $Name: ident {
            $(
                $Variant: ident = $value: expr
            ),*
            $( , )?
        }
    ) => {
        #[repr($T)]
        $( #[$meta] )*
        $vis enum $Name {
            $(
                $Variant = $value
            ),*
        }

        impl std::convert::TryFrom<$T> for $Name {
            type Error = ();

            fn try_from(value: $T) -> Result<$Name, ()> {
                match value {
                    $(
                        $value => Ok($Name::$Variant),
                    )*
                    _ => Err(())
                }
            }
        }
    }
}

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

    pub fn deserialize(from: &str) -> Result<Instance, Error> {
        toml::from_str(from)
    }
}