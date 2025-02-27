// Standard library
use std::process; // For exiting
// use std::path::PathBuf; // For file paths

// External creates
use clap::Parser;

// My stuff
use simple_grep::Argument; // Import Config struct from lib.rs


fn main() {
    
    let config = Argument::parse(); // Parse command-line arguments w/ clap

    // Handle error
    if let Err(e) = simple_grep::read_file_and_print_matches(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
