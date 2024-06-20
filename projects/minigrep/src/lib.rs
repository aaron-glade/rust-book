use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    query: String,
    file_name: String,
    ignore_case: bool,
}

impl Config {
    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let mut args_iter = args.iter();
        
        // Skip the first arg because it's just the name of the binary
        args_iter.next();

        let query = args_iter
            .next()
            .expect("Should be a value here, since args.len >= 3")
            .clone();
        let file_name = args_iter
            .next()
            .expect("Should be a value here, since args.len >= 3")
            .clone();

        let ignore_case = match args_iter.next() {
            Some(word) => word == "ignore",
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_name,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.get_file_name())?;

    let search_results = if config.get_ignore_case() {
        search_case_insensitive(config.get_query(), &contents)
    } else {
        search(config.get_query(), &contents)
    };

    for line in search_results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();
    let mut contents_iter = contents.lines();
    let mut results: Vec<&str> = vec![];

    while let Some(line) = contents_iter.next() {
        if line.to_lowercase().contains(&lower_query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
