use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    //Our new function now returns a Result with a Config instance in the success case and a &'static str in the error cas
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(" Not enough argument");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run (config: Config)  -> Result<(), Box<dyn Error>>{ //dyn short for dynamic
    //Box<dyn Error> means the function will return a type that implements the Error trait,
    // but we don’t have to specify what particular type the return value will be.
    // This gives us flexibility to return error values that may be of different types in different error cases.
    //Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
    //Third, the run function now returns an Ok value in the success case
    let contents= fs::read_to_string(config.filename)?;


    println!("With text:\n{}",contents);

    Ok(())
}