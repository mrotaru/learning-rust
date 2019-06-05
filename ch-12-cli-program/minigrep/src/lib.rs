use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }
    Ok(())
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // first arg is program name; ignore it
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive_env = env::var("CASE_SENSITIVE").unwrap_or_default();
        Ok(Config { query, filename, case_sensitive: case_sensitive_env.eq("true") })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "foo";
        let contents = "\
First line
Some foo king text
On multiple lines
Foo-starting line";
        assert_eq!(vec!["Some foo king text"], search(query, contents));
    }

    #[test]
    fn case_insenstive() {
        let query = "foo";
        let contents = "Foo";
        assert_eq!(vec!["Foo"], search_case_insensitive(query, contents));

        let query = "Foo";
        let contents = "foo";
        assert_eq!(vec!["foo"], search_case_insensitive(query, contents));

        let query = "fOO";
        let contents = "foo";
        assert_eq!(vec!["foo"], search_case_insensitive(query, contents));
    }
}