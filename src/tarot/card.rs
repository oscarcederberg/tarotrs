use std::fmt;
use std::fmt::Formatter;

pub const NUM_RANKS:usize = 14;

#[derive(Debug)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Page,
    Knight,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<u32> for Rank {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Rank::*;

        Ok(match value {
            1 => Ace,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            10 => Ten,
            11 => Page,
            12 => Knight,
            13 => Queen,
            14 => King,
            _ => return Err("outside of range for Rank"),
        })
    }
}

pub const NUM_SUITS:usize = 4;

#[derive(Debug)]
pub enum Suit {
    Wands,
    Cups,
    Swords,
    Pentacles,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<u32> for Suit {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Suit::*;

        Ok(match value {
            0 => Wands,
            1 => Cups,
            2 => Swords,
            3 => Pentacles,
            _ => return Err("outside of range for Suit"),
        })
    }
}

#[derive(Debug)]
pub enum Card {
    Major { name: &'static str },
    Minor { rank: Rank, suit: Suit },
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Card::*;

        match self {
            Major { name } => write!(f, "{name}"),
            Minor { rank, suit } => write!(f, "{rank} of {suit}"),
        }
    }
}
