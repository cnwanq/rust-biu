use std::env;

use minigrep::Config;



fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[2];
    // let (query, file_path) = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {e}");
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
