use crate::card::*;
use std::collections::vec_deque::VecDeque;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    pub cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        use strum::EnumCount;

        let mut cards: VecDeque<Card> = VecDeque::default();

        for order in 1..MAJOR_ARCANA_NAMES.len() {
            cards.push_front(Card::new(Arcana::Major {
                order: order.try_into().unwrap(),
                name: MAJOR_ARCANA_NAMES.get(order).unwrap().to_string(),
            }));
        }

        cards.push_front(Card::new(Arcana::Major {
            order: 0,
            name: MAJOR_ARCANA_NAMES.first().unwrap().to_string(),
        }));

        for suit in 0..Suit::COUNT {
            for rank in (1..=Rank::COUNT).rev() {
                cards.push_front(Card::new(Arcana::Minor {
                    rank: Rank::from_repr(rank).unwrap(),
                    suit: Suit::from_repr(suit).unwrap(),
                }));
            }
        }

        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    pub fn put(&mut self, card: Card) {
        self.cards.push_back(card);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_deck() -> Deck {
        Deck {
            cards: VecDeque::from([
                Card::new(Arcana::Major {
                    order: 0,
                    name: String::from("a"),
                }),
                Card::new(Arcana::Major {
                    order: 1,
                    name: String::from("b"),
                }),
                Card::new(Arcana::Major {
                    order: 2,
                    name: String::from("c"),
                }),
            ]),
        }
    }

    #[test]
    fn peek() {
        let deck = new_test_deck();
        let size = deck.cards.len();
        let card = deck.peek().unwrap();

        assert_eq!(
            *card,
            Card::new(Arcana::Major {
                order: 0,
                name: String::from("a")
            })
        );

        assert_eq!(
            *(deck.cards.front().unwrap()),
            Card::new(Arcana::Major {
                order: 0,
                name: String::from("a")
            })
        );

        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn draw_and_put() {
        let mut deck = new_test_deck();
        let size = deck.cards.len();
        let card = deck.draw().unwrap();

        assert_eq!(
            card,
            Card::new(Arcana::Major {
                order: 0,
                name: String::from("a")
            })
        );

        assert_eq!(
            *(deck.peek().unwrap()),
            Card::new(Arcana::Major {
                order: 1,
                name: String::from("b")
            })
        );

        assert_eq!(deck.cards.len(), size - 1);

        deck.put(card);

        assert_eq!(
            *(deck.cards.front().unwrap()),
            Card::new(Arcana::Major {
                order: 1,
                name: String::from("b")
            })
        );

        assert_eq!(deck.cards.len(), size);
    }
}
