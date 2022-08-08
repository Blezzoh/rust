use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str>{
        if args.len()< 3{
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
    
        Ok(Config {query: String::from(query), filename:String::from(filename), case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let text = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &text)
    } else{
        search_case_insensitive(&config.query, &text)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = vec![];
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

/* -------------- Tests ------------ */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

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