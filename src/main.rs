use crate::tarot::card::Card;

mod tarot;

fn main() {
    use tarot::deck::Deck;

    println!("tarotrs ---");
    let mut deck: Deck = Deck::new();
    let mut card_1 = deck.take_top_card().unwrap();
    let mut card_2 = deck.take_top_card().unwrap();
    let mut card_3 = deck.take_top_card().unwrap();
    println!(
        "first cards:\n{0}\n{1}\n{2}\n",
        card_1, card_2, card_3
    );
    deck.put_at_bottom(card_1);
    deck.put_at_bottom(card_2);
    deck.put_at_bottom(card_3);

    deck.shuffle();
    card_1 = deck.take_top_card().unwrap();
    card_2 = deck.take_top_card().unwrap();
    card_3 = deck.take_top_card().unwrap();
    println!(
        "first cards after shuffling:\n{0}\n{1}\n{2}\n",
        card_1, card_2, card_3
    );
    deck.put_at_bottom(card_1);
    deck.put_at_bottom(card_2);
    deck.put_at_bottom(card_3);

    deck.overhand();
    card_1 = deck.take_top_card().unwrap();
    card_2 = deck.take_top_card().unwrap();
    card_3 = deck.take_top_card().unwrap();
    println!(
        "first cards after overhand:\n{0}\n{1}\n{2}\n",
        card_1, card_2, card_3
    );
    deck.put_at_bottom(card_1);
    deck.put_at_bottom(card_2);
    deck.put_at_bottom(card_3);
}
