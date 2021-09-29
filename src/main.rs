use std::env;
use std::fs;
use std::process;

fn main() {
    // accepting arguments
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|error| {
        println!("Problem in parsing arguments: \n\t - {}", error);
        process::exit(1);
    });

    println!("searching for \"{}\" in file: \"{}\" ...", config.query, config.filename);
    run(config);
}

fn run(config: Config) {
    let file_contents = fs::read_to_string(config.filename)
        .expect("something went wrong in reading file");
    
    println!("file content: {}", file_contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
