use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

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

    fn parse<'a>(args: &'a [String]) -> Result<(&'a str, &'a str, bool), &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let mut count = 0;

        let mut q_flag = false;
        let mut f_flag = false;

        let mut query: &str = &args[1];
        let mut filename: &str = &args[2];
        let mut case_sensitive = true;

        for arg in args {
            count += 1;
            if count == 1 {
                continue;
            }
            if arg.starts_with("--case-insensitive") {
                case_sensitive = false;
                continue;
            }
            if arg.starts_with("--") {
                continue;
            }
            if !q_flag {
                query = arg;
                q_flag = true;

                continue;
            }
            if !f_flag {
                filename = arg;
                f_flag = true;
            }
        }
        if !q_flag || !f_flag {
            return Err("not enough arguments");
        }

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
        let args: &[String] = &[
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(
            Ok((args[1].as_str(), args[2].as_str(), true)),
            Config::parse(args)
        );

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(
            Ok((args[2].as_str(), args[3].as_str(), false)),
            Config::parse(args)
        );

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=1".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(
            Ok((args[2].as_str(), args[3].as_str(), false)),
            Config::parse(args)
        );

        let args: &[String] = &[
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
            "--case-insensitive".to_string(),
        ];
        assert_eq!(
            Ok((args[1].as_str(), args[2].as_str(), false)),
            Config::parse(args)
        );

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=0".to_string(),
            "to".to_string(),
        ];
        assert_eq!(Err("not enough arguments"), Config::parse(args));

        let args: &[String] = &["minigrep".to_string(), "to".to_string()];
        assert_eq!(Err("not enough arguments"), Config::parse(args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=0".to_string(),
            "--other=any".to_string(),
            "to".to_string(),
        ];
        assert_eq!(Err("not enough arguments"), Config::parse(args));
    }
}
