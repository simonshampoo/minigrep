use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    needle: String,
    haystack: String,
    case_insensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].to_string();
        let file_path = args[2].to_string();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            needle: query,
            haystack: file_path,
            case_insensitive: ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.haystack)?;
    let results = if config.case_insensitive {
        search_case_insensitive(&config.needle, &contents)
    } else {
        search(&config.needle, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
