use std::env;

fn main() {
    // accepting arguments
    let arguments: Vec<String> = env::args().collect();
    
    let query = &arguments[1];
    let filename = &arguments[2];

    println!("searching for \"{}\" in file: \"{}\" ...", query, filename)
}
