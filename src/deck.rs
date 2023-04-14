use crate::card::*;
use rand::Rng;
use std::collections::vec_deque::VecDeque;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        use strum::EnumCount;

        let mut cards: VecDeque<Card> = VecDeque::from([
            Card::new(Arcana::Major {
                order: 1,
                name: "The Magician".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 2,
                name: "The High Priestess".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 3,
                name: "The Empress".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 4,
                name: "The Emperor".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 5,
                name: "The Hierophant".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 6,
                name: "The Lovers".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 7,
                name: "The Chariot".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 8,
                name: "Strength".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 9,
                name: "The Hermit".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 10,
                name: "The Wheel of Fortune".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 11,
                name: "Justice".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 12,
                name: "The Hanged Man".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 13,
                name: "Death".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 14,
                name: "Temperance".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 15,
                name: "The Devil".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 16,
                name: "The Tower".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 17,
                name: "The Star".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 18,
                name: "The Moon".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 19,
                name: "The Sun".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 20,
                name: "Judgement".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 21,
                name: "The World".to_string(),
            }),
            Card::new(Arcana::Major {
                order: 0,
                name: "The Fool".to_string(),
            }),
        ]);

        for suit in 0..Suit::COUNT {
            for rank in (1..=Rank::COUNT).rev() {
                cards.push_back(Card::new(Arcana::Minor {
                    rank: Rank::from_repr(rank).unwrap(),
                    suit: Suit::from_repr(suit).unwrap(),
                }));
            }
        }

        // reverse cards
        cards.make_contiguous().reverse();

        Deck { cards }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    pub fn put(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    pub fn random_shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        self.cards.make_contiguous().shuffle(&mut rng);
        self.cards.iter_mut().for_each(|card| {
            if rng.gen_bool(0.5) {
                card.reverse();
            }
        })
    }

    pub fn strip_shuffle(&mut self) {
        if self.cards.len() == 0 {
            return;
        }

        let mut rng = rand::thread_rng();
        let size = self.cards.len();
        let cut = rng.gen_range(0..(self.cards.len() / 2));
        let cards = self.cards.split_off(self.cards.len() - cut);
        let size = self.cards.len();
        let insertion = rng.gen_range(0..=self.cards.len());

        for card in cards.into_iter().rev() {
            self.cards.insert(insertion, card);
        }
    }

    pub fn riffle_shuffle(&mut self) {
        if self.cards.len() == 0 {
            return;
        }

        let mut rng = rand::thread_rng();
        let size: usize = self.cards.len();
        let cut: usize = rng.gen_range(0..size);

        let mut left = self.cards.split_off(cut);
        let mut right = VecDeque::with_capacity(size);
        std::mem::swap(&mut right, &mut self.cards);

        if rng.gen_bool(0.5) {
            left.iter_mut().for_each(|card| card.reverse());
        }

        if rng.gen_bool(0.5) {
            right.iter_mut().for_each(|card| card.reverse());
        }

        if rng.gen_bool(0.5) {
            self.cards = itertools::interleave(right, left).collect();
        } else {
            self.cards = itertools::interleave(left, right).collect();
        }
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
    fn pop_and_put() {
        let mut deck = new_test_deck();
        let size = deck.cards.len();
        let card = deck.pop().unwrap();

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

    #[test]
    fn random_shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        deck.random_shuffle();
        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn strip_shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        deck.strip_shuffle();
        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn riffle_shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        deck.riffle_shuffle();
        assert_eq!(deck.cards.len(), size);
    }
}
