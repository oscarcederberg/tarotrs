use std::collections::VecDeque;

use rand::Rng;

use crate::deck::Deck;

pub trait Shuffle {
    fn shuffle(deck: &mut Deck);
}

pub struct RandomShuffler;
pub struct RiffleShuffler;
pub struct StripShuffler;

impl Shuffle for RandomShuffler {
    fn shuffle(deck: &mut Deck) {
        if deck.cards.is_empty() {
            return;
        }

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        deck.cards.make_contiguous().shuffle(&mut rng);
        deck.cards.iter_mut().for_each(|card| {
            if rng.gen_bool(0.5) {
                card.reverse();
            }
        })
    }
}

impl Shuffle for RiffleShuffler {
    fn shuffle(deck: &mut Deck) {
        if deck.cards.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let size: usize = deck.cards.len();
        let cut: usize = rng.gen_range(0..size);

        let mut left = deck.cards.split_off(cut);
        let mut right = VecDeque::with_capacity(size);
        std::mem::swap(&mut right, &mut deck.cards);

        if rng.gen_bool(0.5) {
            left.iter_mut().for_each(|card| card.reverse());
        }

        if rng.gen_bool(0.5) {
            right.iter_mut().for_each(|card| card.reverse());
        }

        if rng.gen_bool(0.5) {
            deck.cards = itertools::interleave(right, left).collect();
        } else {
            deck.cards = itertools::interleave(left, right).collect();
        }
    }
}

impl Shuffle for StripShuffler {
    fn shuffle(deck: &mut Deck) {
        if deck.cards.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let cut = rng.gen_range(0..(deck.cards.len() / 2));
        let cards = deck.cards.split_off(deck.cards.len() - cut);
        let insertion = rng.gen_range(0..=deck.cards.len());

        for card in cards.into_iter().rev() {
            deck.cards.insert(insertion, card);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn random_shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        RandomShuffler::shuffle(&mut deck);
        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn riffle_shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        RiffleShuffler::shuffle(&mut deck);
        assert_eq!(deck.cards.len(), size);
    }

    #[test]
    fn strip_shuffle() {
        let mut deck = Deck::new();
        let size = deck.cards.len();
        StripShuffler::shuffle(&mut deck);
        assert_eq!(deck.cards.len(), size);
    }
}