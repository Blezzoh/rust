use std::{env,process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        // print to the standard error
        eprintln!("problem occured: {}",err);
        process::exit(1);
    });
 

    println!("query: {}", &config.query);
    println!("file: {}", &config.filename);
    if let Err(e) = minigrep::run(config){
        // print to the standard error
        eprintln!("{}", e);
        process::exit(1);
    }
}
