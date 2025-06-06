use std::fs;

extern crate serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Joke {
    id: String,
    whos_there: String,
    answer_who: String,
    tags: Vec<String>,
    source: String,
}

fn main() {
    let mut jokes = Vec::new();
    for path in fs::read_dir("jokes").unwrap() {
        let path = path.unwrap().path();
        let file = fs::File::open(&path).unwrap();
        let joke: Joke = match serde_json::from_reader(file) {
            Ok(joke) => joke,
            Err(e) => {
                eprintln!("{}: {}", path.display(), e);
                continue;
            }
        };
        jokes.push(joke);
    }
    let out_file = fs::File::create("jokes.json").unwrap();
    serde_json::to_writer(out_file, &jokes).unwrap();
}
