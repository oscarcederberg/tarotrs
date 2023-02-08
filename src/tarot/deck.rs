use crate::tarot::card::*;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        use Card::*;

        let mut cards: Vec<Card> = vec![
            Major { name: "The Fool" },
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
        ];

        for suit in 0..4 {
            for rank in 1..14 {
                cards.push(Minor {
                    rank: Rank::try_from(rank).unwrap(),
                    suit: Suit::try_from(suit).unwrap(),
                });
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::{seq::SliceRandom, thread_rng};
        self.cards.shuffle(&mut thread_rng())
    }

    pub fn overhand(&mut self) {
        let cut: usize = rand::random::<usize>() % self.cards.len();
        println!("cut: {cut}");
        for _ in 0..=cut {
            let card = self.cards.pop().unwrap();
            self.cards.insert(0, card);
        }
    }
}
