use std::{env, error::Error, fs};
use colored::Colorize;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub line: usize
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 3 {
            return Err("Not enought arguments")
        }

        let query = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, filepath, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    if !results.is_empty(){
        for line in results {
            for word in line.content.split_whitespace(){
                if word.to_lowercase().contains(&config.query.to_lowercase()){
                    print!("{} ", word.red());
                    continue;
                }
                print!("{} ", word);
            }
            print!("Line: {}", line.line);
            println!("");
        }    
    }else{println!("Nothing Found :(")}
    
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Line> {

    let mut result = Vec::new();

    for (index,line) in contents.lines().enumerate() {
        if line.contains(query){
            let found_line = Line {
                content: line.to_string(), 
                line: index + 1
            };
            result.push(found_line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {

    let mut result = Vec::new();
    let query = query.to_lowercase();

    for (index,line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query){
            let found_line = Line {
                content: line.to_string(), 
                line: index + 1
            };
            result.push(found_line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "Rust:
safe, fast, productive.
Pick three.";

        // assert_eq!(Line{content: String::from("safe, fast, productive."), line: 5} ,search(query, contents));
        assert_eq!(vec![Line{content: String::from("safe, fast, productive."), line: 2}], search(query, contents))
    }
}