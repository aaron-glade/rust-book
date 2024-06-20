use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(()) => {"minigrep ran successfully";},
        Err(e) => {
            println!("application error: {e}");
            process::exit(1);
        }
    }
}
