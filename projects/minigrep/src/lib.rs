use std::fs;
use std::error::Error;
pub struct Config {
  query: String,
  file_name: String,
}

impl Config {
  pub fn get_query(&self) -> &str {
      &self.query
  }

  pub fn get_file_name(&self) -> &str {
      &self.file_name
  }

  pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("Not enough arguments");
      }

      let query = args[1].clone();
      let file_name = args[2].clone();

      Ok(Config { query, file_name })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.get_file_name())?;

  println!("with text: {contents}");
  Ok(())
}