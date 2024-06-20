use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(()) => {"minigrep ran successfully";},
        Err(e) => {
            eprintln!("application error: {e}");
            process::exit(1);
        }
    }
}
