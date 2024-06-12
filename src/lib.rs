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

    let results = search(&config.query, &contents);

    if !results.is_empty(){
        for line in results {
            for word in line.split_whitespace(){
                if word.contains(&config.query){
                    print!("{} ", word.red());
                    continue;
                }
                print!("{} ", word);
            }
            println!("");
        }    
    }else{println!("Nothing Found :(")}
    
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