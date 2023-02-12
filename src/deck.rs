use crate::card::*;
use std::collections::VecDeque;
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

    pub fn peek(& self) -> Option<&Card> {
        let size = self.cards.len();
        self.cards.get(size - 1)
    }

    pub fn put(&mut self, card: Card) {
        self.cards.push_front(card);
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
