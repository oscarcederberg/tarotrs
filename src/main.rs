mod tarot;

fn main() {
    use tarot::deck::Deck;

    println!("tarotrs ---");
    let mut deck: Deck = Deck::new();
    deck.shuffle();
    println!("first card: {0}", deck.cards.first().unwrap());
}
