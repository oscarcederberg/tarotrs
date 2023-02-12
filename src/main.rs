use std::io;
use std::io::Write;
use tarotrs::Instance;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("tarotrs ---");
    let mut instance = Instance::new();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read stdin");

        match input.trim() {
            "help" => {
                println!("tarotrs ({VERSION})\n");
                println!("pop\t\tview the top card and return it to the bottom of the deckh");
                println!("peek\t\tview the top card of the deck");
                println!("shuffle\t\tshuffles the deck randomly");
                println!("overhand\t\toverhand shuffle the deck");
                println!("riffle\t\triffle shuffle the deck");
                println!("exit\t\texits the program");
            }
            "pop" => {
                let card = instance.deck.pop();
                match card {
                    Some(card) => {
                        println!("the top card was {card}.");
                        instance.deck.put(card);
                    },
                    None => println!("the deck is empty.")
                }
            }
            "peek" => {
                let card = instance.deck.peek();
                match card {
                    Some(card) => println!("the top card is {card}."),
                    None => println!("the deck is empty.")
                }
            }
            "shuffle" => {
                instance.deck.shuffle();
                println!("the deck has been shuffled.");
            }
            "overhand" => {
                instance.deck.overhand();
                println!("the deck has been overhand shuffled.");
            }
            "riffle" => {
                instance.deck.riffle();
                println!("the deck has been riffle shuffled.");
            }
            "exit" => break,
            _ => println!("invalid input, try typing help.")
        }
    }
}
