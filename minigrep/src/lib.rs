use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
     
    let contents = fs::read_to_string(config.filename)?;

    let lines = if config.case_sensitive {
        search(&config.query, &contents) 
    } else {
        search_case_off(&config.query, &contents)
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

// iterate over each line of the contents
// check if line contains query string
// add to list of values to return
// return list
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_off<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l
                .to_lowercase()
                .contains(&query.to_lowercase())
                )
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query found in command"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("No filename found in command"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_1() {
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
        assert_eq!(vec!["Rust:", "Trust me."], search_case_off(query, contents));
    }
}
