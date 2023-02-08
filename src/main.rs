mod tarot;

fn main() {
    use tarot::deck::Deck;

    println!("tarotrs");
    let mut deck: Deck = Deck::new();

    println!("{deck:?}");
}
