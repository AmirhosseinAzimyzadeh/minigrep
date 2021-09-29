use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(config.filename).expect("something went wrong in reading file");

    let result = search(&config.query, &file_contents);
    let number_of_result = result.len();

    println!("{} results found !", number_of_result);
    for line in result {
      println!("\t{} - {}", line.line_number, line.content);
    }

    Ok(())
}

fn search<'a>(term: &'a str, content: &'a str) -> Vec<LineResult<'a>> {
  let mut result_line: Vec<LineResult> = Vec::new();

  let mut line_count: u32 = 0;
  for line in content.lines() {
    line_count += 1;
    if line.contains(term) {
      result_line.push(LineResult { content: line, line_number: line_count });
    }
  }

  result_line
}


struct LineResult<'a> {
  content: &'a str,
  line_number: u32,
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
    assert_eq!("this is a test sentence", search(test_query, test_content)[0].content)
  }
}