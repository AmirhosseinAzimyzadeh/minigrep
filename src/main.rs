use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    // accepting arguments
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|error| {
        println!("Problem in parsing arguments: \n\t - {}", error);
        process::exit(1);
    });

    println!(
        "searching for \"{}\" in file: \"{}\" ...",
        config.query, config.filename
    );

    if let Err(error) = run(config) {
        println!("Something went wrong: \n\t - {}", error);
        process::exit(1);
    }
}

