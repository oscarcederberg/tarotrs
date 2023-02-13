use crate::card::*;
use std::collections::vec_deque::VecDeque;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        use Card::*;

        let mut cards: VecDeque<Card> = VecDeque::from([
            Major { order: 1, name: "The Magician".to_string(), },
            Major { order: 2, name: "The High Priestess".to_string(), },
            Major { order: 3, name: "The Empress".to_string(), },
            Major { order: 4, name: "The Emperor".to_string(), },
            Major { order: 5, name: "The Hierophant".to_string(), },
            Major { order: 6, name: "The Lovers".to_string() },
            Major { order: 7, name: "The Chariot".to_string(), },
            Major { order: 8, name: "Strength".to_string() },
            Major { order: 9, name: "The Hermit".to_string() },
            Major { order: 10, name: "The Wheel of Fortune".to_string(), },
            Major { order: 11, name: "Justice".to_string() },
            Major { order: 12, name: "The Hanged Man".to_string(), },
            Major { order: 13, name: "Death".to_string() },
            Major { order: 14, name: "Temperance".to_string() },
            Major { order: 15, name: "The Devil".to_string() },
            Major { order: 16, name: "The Tower".to_string() },
            Major { order: 17, name: "The Star".to_string() },
            Major { order: 18, name: "The Moon".to_string() },
            Major { order: 19, name: "The Sun".to_string() },
            Major { order: 20, name: "Judgement".to_string() },
            Major { order: 21, name: "The World".to_string() },
            Major { order: 0, name: "The Fool".to_string() },
        ]);

        for suit in 0..NUM_SUITS {
            for rank in (1..=NUM_RANKS).rev() {
                cards.push_back(Minor {
                    rank: Rank::try_from(u32::try_from(rank).unwrap()).unwrap(),
                    suit: Suit::try_from(u32::try_from(suit).unwrap()).unwrap(),
                });
            }
        }

        Deck { cards }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    pub fn pop_n(&mut self, n: usize) -> Option<Vec<Card>> {
        if n > self.cards.len() || n == 0 {
            return None;
        }
        let mut cards = Vec::new();

        for _ in 0..n {
            cards.push(self.pop().unwrap());
        }

        Some(cards)
    }

    pub fn peek(& self) -> Option<&Card> {
        self.cards.back()
    }

    pub fn put(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    pub fn put_n(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.put(card);
        }
    }

    pub fn shuffle(&mut self) {
        use rand::{seq::SliceRandom, thread_rng};
        self.cards.make_contiguous().shuffle(&mut thread_rng())
    }

    pub fn overhand(&mut self) {
        let cut: usize = rand::random::<usize>() % self.cards.len();
        for _ in 0..=cut {
            let card = self.cards.pop_front().unwrap();
            self.cards.push_back(card);
        }
    }

    pub fn riffle(&mut self) {
        let size: usize = self.cards.len();
        let cut: usize = rand::random::<usize>() % size;

        let left = self.cards.split_off(cut);
        let mut right = VecDeque::with_capacity(size);
        std::mem::swap(&mut right, &mut self.cards);

        self.cards = itertools::interleave(right, left).collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card::Major;
    use super::*;

    fn new_test_deck() -> Deck {
        Deck {
            cards: VecDeque::from([
                Major {
                    order: 0, name: String::from("a")
                },
                Major {
                    order: 1, name: String::from("b")
                },
                Major {
                    order: 2, name: String::from("c")
                }
            ])
        }
    }

    #[test]
    fn peek() {
        let deck = new_test_deck();
        let size = deck.cards.len();
        let card = deck.peek().unwrap();

        assert_eq!(*card, Major {
            order: 2, name: String::from("c")
        });

        assert_eq!(*(deck.cards.back().unwrap()), Major {
            order: 2, name: String::from("c")
        });

        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn pop_and_put() {
        let mut deck = new_test_deck();
        let size = deck.cards.len();
        let card = deck.pop().unwrap();

        assert_eq!(card, Major {
            order: 2, name: String::from("c")
        });

        assert_eq!(*(deck.peek().unwrap()), Major {
            order: 1, name: String::from("b")
        });

        assert_eq!(deck.cards.len(), size - 1);

        deck.put(card);

        assert_eq!(*(deck.cards.front().unwrap()), Major {
            order: 2, name: String::from("c")
        });

        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn pop_and_put_n() {
        let mut deck = new_test_deck();
        let size = deck.cards.len();
        let cards = deck.pop_n(size).unwrap();

        assert_eq!(cards, Vec::from([
            Major {
                order: 2, name: String::from("c")
            },
            Major {
                order: 1, name: String::from("b")
            },
            Major {
                order: 0, name: String::from("a")
            },
        ]));

        assert_eq!(deck.cards.len(), 0);

        deck.put_n(cards);

        assert_eq!(deck.cards, Vec::from([
            Major {
                order: 0, name: String::from("a")
            },
            Major {
                order: 1, name: String::from("b")
            },
            Major {
                order: 2, name: String::from("c")
            },
        ]));

        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        deck.shuffle();
        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn overhand() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        deck.overhand();
        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn riffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        deck.riffle();
        assert_eq!(deck.cards.len(), size);
    }
}