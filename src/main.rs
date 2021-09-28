use std::env;
use std::fs;

fn main() {
    // accepting arguments
    let arguments: Vec<String> = env::args().collect();

    let query = &arguments[1];
    let filename = &arguments[2];

    println!("searching for \"{}\" in file: \"{}\" ...", query, filename);

    let file_contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("____________________");
    println!("With text: {}", file_contents);
}
