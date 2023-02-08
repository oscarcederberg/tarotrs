pub enum Orientation {
    Upright,
    Reverse
}

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
    King
}

pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

pub enum Card {
    Major {name: &'static str, orientation: Orientation},
    Minor {rank: Rank, suit: Suit, orientation: Orientation}
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Rank {
    pub fn from_u32(x:u32) -> Option<Rank> {
        use Rank::{*};

        match x {
            1 => Some(Ace),
            2 => Some(Two),
            3 => Some(Three),
            4 => Some(Four),
            5 => Some(Five),
            6 => Some(Six),
            7 => Some(Seven),
            8 => Some(Eight),
            9 => Some(Nine),
            10 => Some(Ten),
            11 => Some(Jack),
            12 => Some(Queen),
            13 => Some(King),
            _ => None
        }
    }
}

impl Suit {
    pub fn from_u32(x: u32) -> Option<Suit> {
        use Suit::{*};

        match x {
            0 => Some(Clubs),
            1 => Some(Diamonds),
            2 => Some(Hearts),
            3 => Some(Spades),
            _ => None
        }
    }
}

impl Deck {
    pub fn new() -> Deck {
        use {Orientation::*, Card::*};
        let mut cards: Vec<Card> = vec![
            Major { name: "The Fool", orientation: Upright },
            Major { name: "The Magician", orientation: Upright },
            Major { name: "The High Priestess", orientation: Upright },
            Major { name: "The Empress", orientation: Upright },
            Major { name: "The Emperor", orientation: Upright },
            Major { name: "The Hierophant", orientation: Upright },
            Major { name: "The Lovers", orientation: Upright },
            Major { name: "The Chariot", orientation: Upright },
            Major { name: "Strength", orientation: Upright },
            Major { name: "The Hermit", orientation: Upright },
            Major { name: "The Wheel of Fortune", orientation: Upright },
            Major { name: "Justice", orientation: Upright },
            Major { name: "The Hanged Man", orientation: Upright },
            Major { name: "Death", orientation: Upright },
            Major { name: "The Devil", orientation: Upright },
            Major { name: "The Tower", orientation: Upright },
            Major { name: "The Star", orientation: Upright },
            Major { name: "The Moon", orientation: Upright },
            Major { name: "The Sun", orientation: Upright },
            Major { name: "Judgement", orientation: Upright },
            Major { name: "The World", orientation: Upright },
        ];

        for suit in 0..4 {
            for rank in 1..14 {
                cards.push(Minor {
                    rank: Rank::from_u32(rank).unwrap(),
                    suit: Suit::from_u32(suit).unwrap(),
                    orientation: Upright});
            }
        }

        Deck{cards: cards}
    }
}