use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let (query, filename) = Config::parse(args)?;

        let query = query.to_string();
        let filename = filename.to_string();
        let case_sensitive = Config::is_case_sensitive(args);

        Ok(Config {query, filename, case_sensitive})
    }

    fn parse<'a>(args: &'a [String]) -> Result<(&'a str, &'a str), &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let mut count = 0;
        let mut q_flag = false;
        let mut f_flag = false;
        let mut query: &str = &args[1];
        let mut filename: &str = &args[2];
        for arg in args {
            count += 1;
            if count == 1 || arg.starts_with("--") {
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
            return Err("not enough arguments")
        }
        Ok((query, filename))
    }

    fn is_case_sensitive(args: &[String]) -> bool {
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        for arg in args {
            if arg.starts_with("--case-insensitive=") {
                let ci:Vec<&str> = arg.split("=").collect();
                if ci[1] != "0" && ci[1] != "false" && ci[1] != "no" {
                    case_sensitive = false;
                }
            }
        }

        case_sensitive
    }
}

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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
            vec![ "Rust:", "Trust me." ],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn is_case_sensitive() {
        let args: &[String] = &[
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(true, Config::is_case_sensitive(&args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
            "--case-insensitive=1".to_string(),
        ];
        assert_eq!(false, Config::is_case_sensitive(&args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=1".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(false, Config::is_case_sensitive(&args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=yes".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(false, Config::is_case_sensitive(&args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=true".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(false, Config::is_case_sensitive(&args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=no".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(true, Config::is_case_sensitive(&args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=false".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(true, Config::is_case_sensitive(&args));
    }

    #[test]
    fn parse() {
        let args: &[String] = &[
            "minigrep".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(Ok((args[1].as_str(), args[2].as_str())), Config::parse(args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=false".to_string(),
            "to".to_string(),
            "target.txt".to_string(),
        ];
        assert_eq!(Ok((args[2].as_str(), args[3].as_str())), Config::parse(args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=false".to_string(),
            "to".to_string(),
        ];
        assert_eq!(Err("not enough arguments"), Config::parse(args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "to".to_string(),
        ];
        assert_eq!(Err("not enough arguments"), Config::parse(args));

        let args: &[String] = &[
            "minigrep".to_string(),
            "--case-insensitive=false".to_string(),
            "--other=any".to_string(),
            "to".to_string(),
        ];
        assert_eq!(Err("not enough arguments"), Config::parse(args));
    }
}
