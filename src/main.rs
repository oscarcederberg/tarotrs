use tarotrs::Instance;
use Command::*;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_instance() -> Result<Instance, ()> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/tarotrs/"));
    file_path.push("instance.toml");
    match fs::read_to_string(file_path) {
        Ok(contents) => Instance::deserialize(contents.as_str()),
        Err(_) => Err(()),
    }
}

fn save_instance(instance: &Instance) -> Result<(), ()> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/tarotrs/"));
    fs::create_dir_all(&file_path).unwrap();
    file_path.push("instance.toml");

    let serialized = instance.serialize()?;

    let mut file = File::create(file_path.as_path()).expect("couldn't write to file");
    match file.write_all(serialized.as_bytes()) {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

enum Command {
    Help,
    Pop {amount: usize},
    Peek,
    Shuffle,
    Overhand,
    Riffle,
    Reset,
    Save,
    Load,
    Quit,
    Other
}

fn parse_command(text: &String) -> Command {
    let words: Vec<&str> = text.trim().split_whitespace().collect();

    if words.is_empty() {
        return Other;
    }

    match words.first().unwrap().to_lowercase().as_str() {
        "help" => if words.len() == 1 { Help } else { Other} ,
        "pop" => if words.len() == 1 { Pop { amount: 1 } } else {
            if words.len() > 2 {
                return Other;
            }

            match words.get(1).unwrap_or(&&"1").parse::<usize>() {
                Ok(0) => Other,
                Ok(amount) => Pop{ amount },
                _ => Other,
            }
        },
        "peek" => if words.len() == 1 { Peek } else { Other },
        "shuffle" => if words.len() == 1 { Shuffle } else { Other },
        "overhand" => if words.len() == 1 { Overhand } else { Other },
        "riffle" => if words.len() == 1 { Riffle } else { Other },
        "reset" => if words.len() == 1 { Reset } else { Other },
        "save" => if words.len() == 1 { Save } else { Other },
        "load" => if words.len() == 1 { Load } else { Other },
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
        let command = parse_command(&input);

        match command {
            Help => {
                println!("tarotrs ({VERSION})\n");
                println!("pop\t\tview the top card and return it to the bottom of the deck");
                println!("pop <n>\t\tview the n top cards");
                println!("peek\t\tview the top card of the deck");
                println!("shuffle\t\tshuffles the deck randomly");
                println!("overhand\toverhand shuffle the deck");
                println!("riffle\t\triffle shuffle the deck");
                println!("quit\t\texits the program");
            }
            Pop { amount } => {
                for _ in 0..amount {
                    let card = instance.deck.pop();
                    match card {
                        Some(card) => {
                            println!("the top card was {card}.");
                            instance.deck.put(card);
                        },
                        None => {
                            println!("the deck is empty.");
                            break;
                        }
                    }
                }
            }
            Peek => {
                let card = instance.deck.peek();
                match card {
                    Some(card) => println!("the top card is {card}."),
                    None => println!("the deck is empty.")
                }
            }
            Shuffle => {
                instance.deck.shuffle();
                println!("the deck has been shuffled.");
            }
            Overhand => {
                instance.deck.overhand();
                println!("the deck has been overhand shuffled.");
            }
            Riffle => {
                instance.deck.riffle();
                println!("the deck has been riffle shuffled.");
            }
            Reset => {
                instance= Instance::new();
                println!("the instace has been reset.");
            }
            Save => {
                match save_instance(&instance) {
                    Ok(_) => println!("saved instance to disk."),
                    Err(_) => println!("failed to save instance to disk."),
                }
            }
            Load => {
                match load_instance() {
                    Ok(deserialized) => {
                        instance = deserialized;
                        println!("loaded instance from disk.");
                    }
                    Err(_) => println!("failed to load instance from disk (maybe no instance have been saved beforehand?)."),
                }
            }
            Quit => break,
            Other => println!("invalid input, try typing help.")
        }
    }
}
