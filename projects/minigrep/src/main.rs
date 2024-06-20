use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn get_query(&self) -> &str {
        &self.query
    }

    fn get_file_name(&self) -> &str {
        &self.file_name
    }

    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for query: {} on file: {}",
        config.get_query(),
        config.get_file_name()
    );

    let contents = fs::read_to_string(config.get_file_name())
        .expect("Should have been able to read provided file");

    println!("with text: {contents}");
}
