use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use cursive::theme::Style;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use strum_macros::Display;
use tarotrs::deck::Deck;
use tarotrs::shuffle::*;
use tarotrs::Instance;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Display)]
enum LoadInstanceError {
    FileReadError(std::io::Error),
    DeserializeError(toml::de::Error),
}

#[derive(Debug, Display)]
enum SaveInstanceError {
    FileWriteError(std::io::Error),
    SerializeError(toml::ser::Error),
}

impl std::error::Error for LoadInstanceError {}
impl std::error::Error for SaveInstanceError {}

impl From<std::io::Error> for LoadInstanceError {
    fn from(err: std::io::Error) -> Self {
        LoadInstanceError::FileReadError(err)
    }
}

impl From<toml::de::Error> for LoadInstanceError {
    fn from(err: toml::de::Error) -> Self {
        LoadInstanceError::DeserializeError(err)
    }
}

impl From<std::io::Error> for SaveInstanceError {
    fn from(err: std::io::Error) -> Self {
        SaveInstanceError::FileWriteError(err)
    }
}

impl From<toml::ser::Error> for SaveInstanceError {
    fn from(err: toml::ser::Error) -> Self {
        SaveInstanceError::SerializeError(err)
    }
}

fn load_instance() -> Result<Instance, LoadInstanceError> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/instance.toml");
    let contents = fs::read_to_string(file_path)?;
    Ok(Instance::deserialize(contents.as_str())?)
}

fn save_instance(instance: &Instance) -> Result<(), SaveInstanceError> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/");
    fs::create_dir_all(&file_path)?;
    file_path.push("instance.toml");

    let serialized = instance.serialize()?;

    let mut file = File::create(file_path.as_path())?;
    Ok(file.write_all(serialized.as_bytes())?)
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
        .title(format!("tarotrs tui {VERSION}")),
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
