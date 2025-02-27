// Standard library
use std::env; // Get command-line arguments
use std::process; // For exiting

// Custom library
use simple_grep::Config; // Import Config struct from lib.rs


fn main() {
    
    // Command-line arguments
    let args: Vec<String> = env::args().collect(); // Assumes inputs are valid Unicode

    let config = Config::make(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }); // Parse command-line arguments

    // Handle error
    if let Err(e) = simple_grep::read_file_and_print_matches(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
