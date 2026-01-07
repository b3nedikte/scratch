use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    query: String,
    file_path: String,
    pub ignore_case: bool,
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // first arg is the program name, iter over

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
