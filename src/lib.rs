use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(config.filename).expect("something went wrong in reading file");

    println!("file content: {}", file_contents);
    Ok(())
}

fn search<'a>(term: &'a str, content: &'a str) -> Vec<&'a str> {
  vec!["test"]
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn one_result() {
    let test_query = "test";
    let test_content = "/
      this is a test sentence
    ";
    assert_eq!(vec!["this", "is", "a", "test", "sentence"], search(test_query, test_content))
  }
}