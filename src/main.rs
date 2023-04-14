use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use cursive::theme::Style;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use tarotrs::deck::Deck;
use tarotrs::shuffle::*;
use tarotrs::Instance;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_instance() -> Result<Instance, toml::de::Error> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/instance.toml");
    match fs::read_to_string(file_path) {
        Ok(contents) => Instance::deserialize(contents.as_str()),
        Err(_) => panic!("couldn't load instance"),
    }
}

fn save_instance(instance: &Instance) -> Result<(), toml::ser::Error> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/");
    fs::create_dir_all(&file_path).unwrap();
    file_path.push("instance.toml");

    let serialized = instance.serialize()?;

    let mut file = File::create(file_path.as_path()).expect("couldn't create file");
    file.write_all(serialized.as_bytes())
        .expect("couldn't wrie to file");
    Ok(())
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Command {
    Draw,
    Peek,
    Shuffle,
    Reset,
    Quit,
}

fn main() {
    use Command::*;

    let mut siv = cursive::default();
    let instance = Rc::new(RefCell::new(load_instance().unwrap_or(Instance::new())));
    let action_select = SelectView::new()
        .item("draw top card", Draw)
        .item("peek top card", Peek)
        .item("shuffle the deck", Shuffle)
        .item("reset the deck", Reset)
        .item("quit", Quit)
        .on_submit(move |siv, selected| perform_action(siv, selected, Rc::clone(&instance)));

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("select an option").style(Style::title_primary()))
                .child(DummyView)
                .child(action_select),
        )
        .title(format!("tarotrs {VERSION}")),
    );

    siv.run();
}

fn perform_action(siv: &mut Cursive, selected: &Command, instance: Rc<RefCell<Instance>>) {
    use Command::*;

    match selected {
        Draw => {
            let mut instance = instance.borrow_mut();
            let card = instance.deck.draw().unwrap();
            show_popup(siv, "draw".to_owned(), format!("you drew\nThe {card}"));
            instance.deck.put(card);
        }
        Peek => {
            let instance = instance.borrow_mut();
            let card = instance.deck.peek().unwrap();
            show_popup(
                siv,
                "peek".to_owned(),
                format!("the top card is\nThe {card}"),
            );
        }
        Shuffle => {
            siv.add_layer(
                Dialog::text("shuffle the deck")
                    .title("shuffle")
                    .button("random", {
                        let instance = instance.clone();
                        move |_siv| {
                            RandomShuffler::shuffle(&mut instance.borrow_mut().deck);
                        }
                    })
                    .button("strip", {
                        let instance = instance.clone();
                        move |_siv| {
                            StripShuffler::shuffle(&mut instance.borrow_mut().deck);
                        }
                    })
                    .button("riffle", move |_siv| {
                        RiffleShuffler::shuffle(&mut instance.borrow_mut().deck);
                    })
                    .button("done", |siv| {
                        siv.pop_layer();
                    }),
            );
        }
        Reset => {
            instance.borrow_mut().deck = Deck::default();
            show_popup(
                siv,
                "reset".to_owned(),
                "the deck has been reset".to_owned(),
            );
        }
        Quit => match save_instance(&instance.borrow_mut()) {
            Ok(_) => siv.quit(),
            Err(_) => {
                siv.add_layer(
                    Dialog::text("unable to save instance to file")
                        .title("warning")
                        .button("OK", Cursive::quit),
                );
            }
        },
    }
}

fn show_popup(siv: &mut Cursive, title: String, text: String) {
    siv.add_layer(Dialog::text(text).title(title).button("OK", |siv| {
        siv.pop_layer();
    }));
}
