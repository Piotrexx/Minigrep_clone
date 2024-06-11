use std::env;
use std::process;
use cli_app::Config;



fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("File Path: {}", config.filepath);
    println!("-----------------------------------------------------------------");
    if let Err(e) = cli_app::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
}

