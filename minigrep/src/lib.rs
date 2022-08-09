use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // remember functions and method differ in the use of self
    // this is a function that instantiate a new Config struct. To be called Config::new()
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

        // search(&config.query, &text)
        search_using_iter(&config.query, &text)
    } else{
        search_case_insensitive(&config.query, &text)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// remember lifetime here
// lifetime have three rules. 
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

/*
- uses an iterator and a closure.
- Filter function has a close inside, this is almost like javascript arrow function syntax wise
- .lines() returns an iterator over the lines of a string
- iterators are lazy, performs their task in this case when we do a collect
- this search function is the same as the one above. Rust developers like iterators compared to some collections like arrays
because they come with additional perks; in this case a filter. 
*/
pub fn search_using_iter <'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
    .filter(|line| line.contains(query)) // the function inside filter is a closure
    .collect()
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