use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String>= env::args().collect();
    
    let config= Config::new(&args).unwrap_or_else(|err|{
        println!("{}",err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In File {}", config.file_path);
    
    if let Err(e)=read_file(&config.file_path){
        println!("Application Error {e}");
        process::exit(1);
    }
}

struct Config{
    query:String,
    file_path:String
}

impl  Config {
    fn new(args:&[String])->Result<Config, &'static str>{

        if args.len()<3{
            return Err("Not enough")
            
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

         Ok(Config{query,file_path})
    }
}


fn read_file(file_path:&str)-> Result<(), Box<dyn Error>>{
    let content=fs::read_to_string(file_path).map_err(|_| format!("Fail to read file"))?;
    

    
    println!("With text :\n {content}");
    Ok(())
}