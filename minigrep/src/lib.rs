use std::error::Error;
use std::env;
use std::fs;

// Function returns Unit type - "()"
// Box<dyn Error> is a trait object
// It means the function will return a type that implements the Error trait
// but we don't have to specify what particular type the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // "?" will return teh error value from the current function for the caller to handle
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // Idiomatic way to indicate that we're calling "run" for its side effects only
    // it doesn't return a value we need
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    // " &'static " is the type of string literals
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("Looking for: {}", query);
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
Pick three";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}