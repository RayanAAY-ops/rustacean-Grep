use std::env;
use minigrep::Config;

use std::process;
fn main () {
    let args: Vec<String> = env::args().collect(); //this function returns an iterator
    //println!("{:?}",args);
   // let query = &args[1]; // store argument values in variables
   // let filename = &args[2];
    //let (_query,filename) = parse_config(&args); 1,2
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Using unwrap_or_else allows us to define some custom, non-panic! error handling
        println!("Problem parsing arguments: {}", err);
        process::exit(1);

        //. The process::exit function will stop the program immediately and return the number that was passed as the exit status code
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }

    // Read the file

    //let contents = fs::read_to_string(filename)
    //    .expect(" Something went wrong reading the file");

  /*  fn run (config: Config) {
        let contents= fs::read_to_string(config.filename)
            .expect("Somerthing went wront reading the file");

        println!("With text:\n{}",contents);
        }
    }*/


}



/*fn parse_config(args: &[String]) -> (&str,&str) {
    let query = &args[1];
    let filename = &args[2];

    (query,filename)
}*/

/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }

}*/



