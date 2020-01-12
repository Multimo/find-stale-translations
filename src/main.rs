extern crate serde_json;
extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let keys_path = &args[1];
    if keys_path == "" {
        panic!("No keys path argument supplied");
    }
    let translation_keys = parse_keys_path(&keys_path);

    let search_path = &args[2];
    if search_path == "" {
        panic!("No search_path argument supplied");
    }

    let mut missing_translation_keys: Vec<String> = Vec::new();

    WalkDir::new(search_path)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|file| {
            let file_path = file.path();
            println!("{}", file_path.display());
            let found_translations = check_file_for_translation_keys(file_path, &translation_keys);

            for translation in found_translations.into_iter() {
                missing_translation_keys.push(translation)
            }
        });

    write_results_to_file(missing_translation_keys)
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn parse_keys_path(key_path: &String) -> Vec<String> {
    let file = File::open(Path::new(key_path)).expect("Unable to open keys file");
    let translations_json: HashMap<String, String> =
        serde_json::from_reader(&file).expect("JSON was not well-formatted");

    return translations_json.keys().map(|key| key.clone()).collect();
}

fn check_file_for_translation_keys(
    file_path: &Path,
    translation_keys: &Vec<String>,
) -> Vec<String> {
    let mut file = File::open(Path::new(file_path)).expect("Unable to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Unable to open file");

    return translation_keys
        .into_iter()
        .filter(|key| file_contents.contains(*key))
        .map(|key| key.clone())
        .collect();
}

fn write_results_to_file(found_translations: Vec<String>) {
    let mut f = File::create("output.txt").expect("Could not create file to output results");

    for translation in &found_translations {
        write!(f, "{}", translation).expect("Could not write to file to output results");
    }
}
