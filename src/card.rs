use crate::enum_try_from;
use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

pub const NUM_RANKS:usize = 14;
pub const NUM_SUITS:usize = 4;

enum_try_from! {
    #[repr(u32)]
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum Rank {
        Ace = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
        Ten = 10,
        Page = 11,
        Knight = 12,
        Queen = 13,
        King = 14,
    }
}

enum_try_from! {
    #[repr(u32)]
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum Suit {
        Wands = 0,
        Cups = 1,
        Swords = 2,
        Pentacles = 3,
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Upright,
    Reverse,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Arcana {
    Major { order: u32, name: String },
    Minor { rank: Rank, suit: Suit },
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Card {
    pub arcana: Arcana,
    pub orientation: Orientation,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.arcana {
            Arcana::Major { order, name } => {
                let numeral = if *order == 0 {
                    String::from("0")
                } else {
                    roman::to(i32::try_from(*order).expect("Order outside of range")).unwrap()
                };

                if Orientation::Upright == self.orientation {
                    write!(f, "{numeral} - {name}")
                } else {
                    write!(f, "Reversed {numeral} - {name}")
                }
            },
            Arcana::Minor { rank, suit } => {
                if Orientation::Upright == self.orientation {
                    write!(f, "{rank} of {suit}")
                } else {
                    write!(f, "Reversed {rank} of {suit}")
                }
            },
        }
    }
}

impl Card {
    pub fn new(arcana:Arcana) -> Card {
        Card { arcana: arcana, orientation: Orientation::Upright }
    }

    pub fn reverse(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Upright => Orientation::Reverse,
            Orientation::Reverse => Orientation::Upright,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_card() {
        let mut card = Card {
            arcana: Arcana::Major { order: 0, name: String::from("a")},
            orientation: Orientation::Upright,
        };

        assert_eq!(card.orientation, Orientation::Upright);

        card.reverse();
    }
}
