use std::error::Error;
use std::{fs, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 如果产生异常，将异常抛出给上层调用者
    let content= fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return Err("not enough arguments");
        }
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("did not get query string"),
        }  ;
        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("did not get query string"),
        } ;
        Ok(Config { query, filename })
    }
}

pub fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str>{
    // let mut results = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    content.lines().filter(|line| {line.contains(query)}).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}


