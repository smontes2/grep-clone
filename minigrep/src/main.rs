use std::{env, process}; // import env module
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // When using collect, make sure to explicitly annotate the type (:Vec<String>)

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        print!("Application error {e}");
        process::exit(1);
    }
}