use std::fmt;
use std::fmt::Formatter;

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
    Jack,
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
            11 => Jack,
            12 => Queen,
            13 => King,
            _ => return Err("outside of range for Rank")
        })
    }
}

#[derive(Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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

        Ok (match value {
            0 => Clubs,
            1 => Diamonds,
            2 => Hearts,
            3 => Spades,
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
            Minor { rank,suit } => write!(f, "{rank} of {suit}"),
        }
    }
}