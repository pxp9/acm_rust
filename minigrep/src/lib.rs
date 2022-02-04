//! Command to grep lines in a file
//!
//! This module contains the logic and execution for Rust program grep
//!
//! You could call it with an env variable CASE_INSENSITIVE in order to
//! get lines without case insensitive
use std::env;
use std::error::Error;
use std::fs;

#[inline]
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

/// Struct to config the grep
/// in order to what should do the command
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::iter::Skip<env::Args>) -> Result<Config, Box<dyn Error>> {
        if args.len() != 2 {
            let error: Box<dyn Error> =
                String::from("Should be 2 arguments ( query filename )").into();
            return Err(error);
        }
        let query = match args.next() {
            Some(arg) => arg,
            None => {
                let error: Box<dyn Error> = String::from("Query argument is wrong").into();
                return Err(error);
            }
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                let error: Box<dyn Error> = String::from("Filename argument is wrong").into();
                return Err(error);
            }
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
#[inline]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[inline]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: &str = &(query.to_lowercase());
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
