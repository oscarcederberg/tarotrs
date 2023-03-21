use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use tarotrs::Instance;
use cursive::Cursive;
use cursive::views::{Dialog, TextView};

#[allow(dead_code)]
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(dead_code)]
fn load_instance() -> Result<Instance, ()> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/instance.toml");
    println!("loading from {}...", file_path.display());
    match fs::read_to_string(file_path) {
        Ok(contents) => Instance::deserialize(contents.as_str()),
        Err(_) => Err(()),
    }
}

#[allow(dead_code)]
fn save_instance(instance: &Instance) -> Result<(), ()> {
    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::from("~/.cache/"));
    file_path.push("tarotrs/");
    fs::create_dir_all(&file_path).unwrap();
    file_path.push("instance.toml");
    println!("saving to {}...", file_path.display());

    let serialized = instance.serialize()?;

    let mut file = File::create(file_path.as_path()).expect("couldn't write to file");
    match file.write_all(serialized.as_bytes()) {
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(TextView::new("hello world!"))
        .title(format!("tarotrs {}", VERSION))
        .button("quit", Cursive::quit));
    siv.run();
}