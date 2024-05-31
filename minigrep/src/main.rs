use std::{env, process};
use minigrep::Config;
use minigrep;


// HOW TO RUN cargo run -- searchstring example-filename.txt
fn main() {
    let args= env::args();

    // println!("{:?}",&args);
    
    let config= Config::new(args).unwrap_or_else(|err|{
        println!("{}",err);
        process::exit(1);
    });

    

    println!("Searching for {}", config.query);
    println!("In File {}", config.file_path);
    
    if let Err(e)=minigrep::read_file(&config.file_path){
        println!("Application Error {e}");
        process::exit(1);
    }
}



