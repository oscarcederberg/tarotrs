mod tarot;

fn main() {
    use tarot::deck::Deck;

    println!("tarotrs ---");
    let mut deck: Deck = Deck::new();
    println!(
        "first card: {0}",
        deck.cards.first().unwrap()
    );

    deck.shuffle();
    println!(
        "first card after shuffling: {0}",
        deck.cards.first().unwrap()
    );

    deck.overhand();
    println!(
        "first card after overhand: {0}",
        deck.cards.first().unwrap()
    );
}
