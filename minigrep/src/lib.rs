use std::{error::Error, fs};

pub struct Config{
   pub query:String,
    pub file_path:String
}


///CONFIG READING ARGS CLI
impl  Config {
    pub fn new(
        mut args: impl Iterator<Item = String>
    )->Result<Config, &'static str>{

        args.next();

        let query = match args.next() {
            Some(arg)=>arg,
            None=>return Err("Didn't get a query thing")
        };

        let file_path = match args.next() {
            Some(arg)=>arg,
            None=>return Err("Didn't get a file path")
        };

         Ok(Config{query,file_path})
    }
}

/// READ FILE
/// 
pub fn read_file(file_path:&str)-> Result<(), Box<dyn Error>>{
    let content=fs::read_to_string(file_path).map_err(|_| format!("Fail to read file {file_path}"))?;
    

    
    println!("With text :\n {content}");
    Ok(())
}