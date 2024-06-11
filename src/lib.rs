use std::{error::Error, fs};
use colored::Colorize;

pub struct Config {
    pub query: String,
    pub filepath: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 3 {
            return Err("Not enought arguments")
        }

        let query = args[1].clone();
        let filepath = args[2].clone();
    
        Ok(Config {query, filepath})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    for line in search(&config.query, &contents) {
        println!("Line: {}", line);
        println!("LINE AFTER: {:?}", line.split_whitespace().next());
        for word in line.split_whitespace(){
            print!(" {}", word);
            if word == config.query{
                println!("{}", word.red());
                continue;
            }
        }
        println!("{line}")
    }

    // println!("With text:\n{contents}");

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            result.push(line)
        }
    }

    result
}

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
}