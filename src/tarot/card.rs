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

#[derive(Debug)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[derive(Debug)]
pub enum Card {
    Major { name: &'static str },
    Minor { rank: Rank, suit: Suit },
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
