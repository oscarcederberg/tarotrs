use std::io;
use std::io::Write;
use toml::from_str;
use tarotrs::Instance;
use crate::Command::{Other, Shuffle};

const VERSION: &str = env!("CARGO_PKG_VERSION");

enum Command {
    Help,
    Pop {amount: Option<usize>},
    Peek,
    Shuffle,
    Overhand,
    Riffle,
    Quit,
    Other
}

fn parse_command(text: String) -> Command {
    use Command::*;
    let words: Vec<&str> = text.trim().split_whitespace().collect();

    if words.is_empty() {
        return Other;
    }

    match words.first().unwrap().to_lowercase().as_str() {
        "help" => if words.len() == 1 { Help } else { Other} ,
        "pop" => if words.len() == 1 { Pop { amount: None } } else {
            if words.len() > 2 {
                return Other;
            }

            match words.get(1) {
                Some(amount) => {
                    match from_str::<usize>(amount) {
                        Ok(amount) => Pop{ amount: Some(amount) },
                        _ => Other,
                    }
                }
                None => Pop{amount: None},
            }
        },
        "peek" => if words.len() == 1 { Peek } else { Other },
        "shuffle" => if words.len() == 1 { Shuffle } else { Other },
        "overhand" => if words.len() == 1 { Overhand } else { Other },
        "riffle" => if words.len() == 1 { Riffle } else { Other },
        "quit" | "exit" | "stop" => if words.len() == 1 { Quit } else { Other },
        _ => Other,
    }
}

fn main() {
    println!("tarotrs ---\n");
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
                println!("pop\t\tview the top card and return it to the bottom of the deck");
                println!("peek\t\tview the top card of the deck");
                println!("shuffle\t\tshuffles the deck randomly");
                println!("overhand\t\toverhand shuffle the deck");
                println!("riffle\t\triffle shuffle the deck");
                println!("quit\t\texits the program");
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
            "quit" => break,
            _ => println!("invalid input, try typing help.")
        }
    }
}
