use std::collections::VecDeque;
use crate::tarot::card::*;

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        use Card::*;

        let mut cards: VecDeque<Card> = VecDeque::from([
            Major { name: "The Magician", },
            Major { name: "The High Priestess", },
            Major { name: "The Empress", },
            Major { name: "The Emperor", },
            Major { name: "The Hierophant", },
            Major { name: "The Lovers" },
            Major { name: "The Chariot", },
            Major { name: "Strength" },
            Major { name: "The Hermit" },
            Major { name: "The Wheel of Fortune", },
            Major { name: "Justice" },
            Major { name: "The Hanged Man", },
            Major { name: "Death" },
            Major { name: "The Devil" },
            Major { name: "The Tower" },
            Major { name: "The Star" },
            Major { name: "The Moon" },
            Major { name: "The Sun" },
            Major { name: "Judgement" },
            Major { name: "The World" },
            Major { name: "The Fool" },
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

    pub fn take_top_card(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    pub fn put_at_bottom(&mut self, card: Card) {
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
