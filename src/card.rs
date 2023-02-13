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
#[serde(tag = "type")]
pub enum Card {
    Major { order: u32, name: String },
    Minor { rank: Rank, suit: Suit },
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
        use Card::*;

        match self {
            Major { order, name } => {
                let numeral = if *order == 0 {
                    String::from("0")
                } else {
                    roman::to(i32::try_from(*order).expect("Order outside of range")).unwrap()
                };

                write!(f, "{numeral} - {name}")
            },
            Minor { rank, suit } => write!(f, "{rank} of {suit}"),
        }
    }
}