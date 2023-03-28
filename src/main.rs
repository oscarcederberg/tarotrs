use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use cursive::theme::Style;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use tarotrs::Instance;

#[allow(dead_code)]
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(dead_code)]
fn load_instance() -> Result<Instance, toml::de::Error> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/instance.toml");
    println!("loading from {}...", file_path.display());
    match fs::read_to_string(file_path) {
        Ok(contents) => Instance::deserialize(contents.as_str()),
        Err(_) => panic!("couldn't load instance"),
    }
}

#[allow(dead_code)]
fn save_instance(instance: &Instance) -> Result<(), toml::ser::Error> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/");
    fs::create_dir_all(&file_path).unwrap();
    file_path.push("instance.toml");
    println!("saving to {}...", file_path.display());

    let serialized = instance.serialize()?;

    let mut file = File::create(file_path.as_path()).expect("couldn't create file");
    file.write_all(serialized.as_bytes())
        .expect("couldn't wrie to file");
    Ok(())
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Command {
    Help,
    Pop,
    Peek,
    Shuffle,
    Strip,
    Riffle,
    Save,
    Load,
    Reset,
    Quit,
    Other,
}

fn main() {
    use Command::*;

    let mut siv = cursive::default();
    let _instance = Instance::new();
    let action_select = SelectView::new()
        .item("pull top card", Pop)
        .item("peek top card", Peek)
        .item("quit", Quit)
        .on_submit(|siv, selected| match selected {
            Pop => {}
            Peek => {}
            Quit => Cursive::quit(siv),
            _ => (),
        });

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
