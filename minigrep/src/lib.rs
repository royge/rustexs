use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let (query, filename, mut case_sensitive) = Config::parse(args)?;

        if case_sensitive {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }

        let query = query.to_string();
        let filename = filename.to_string();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    fn parse(
        mut args: impl Iterator<Item = String>,
    ) -> Result<(String, String, bool), &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = match args.next() {
            Some(arg) => !arg.starts_with("--case-insensitive"),
            None => true,
        };

        Ok((query, filename, case_sensitive))
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
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
Duck tape.";

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

    #[test]
    fn config_parse() {
        let args = vec![
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(
            Ok((args[1].to_string(), args[2].to_string(), true)),
            Config::parse(args.into_iter())
        );

        let args = vec![
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
            "--case-insensitive".to_string(),
        ];
        assert_eq!(
            Ok((args[1].to_string(), args[2].to_string(), false)),
            Config::parse(args.into_iter())
        );

        let args = vec![
            "minigrep".to_string(),
            "to".to_string(),
        ];
        assert_eq!(Err("Didn't get a filename"), Config::parse(args.into_iter()));

        let args  = vec!["minigrep".to_string()];
        assert_eq!(Err("Didn't get a query string"), Config::parse(args.into_iter()));
    }
}
