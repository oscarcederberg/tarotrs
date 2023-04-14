use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, EnumCount, FromRepr)]
pub enum Rank {
    Ace = 1,
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

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, EnumCount, FromRepr)]
pub enum Suit {
    Wands = 0,
    Cups,
    Swords,
    Pentacles,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Upright,
    Reverse,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Arcana {
    Major { order: u32, name: String },
    Minor { rank: Rank, suit: Suit },
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
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
        Card { arcana, orientation: Orientation::Upright }
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
