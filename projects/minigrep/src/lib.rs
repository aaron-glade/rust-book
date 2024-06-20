use std::error::Error;
use std::fs;
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
    for line in search(config.get_query(), &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut contents_iter = contents.lines();
    let mut search_results: Vec<&str> = vec![];

    while let Some(line) = contents_iter.next() {
        if line.contains(query) {
            search_results.push(line);
        }
    }
    search_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
