#![warn(clippy::all, clippy::pedantic)] // Linting


// Standard library
use std::process; // For exiting
// use std::path::PathBuf; // For file paths

// External creates
use clap::Parser;

// My stuff
use simple_grep::Argument; // Import Config struct from lib.rs


fn main() {
    
    let config = Argument::parse(); // Parse command-line arguments w/ clap

    match &config.recursive {
        true => {
            // Handle error
            if let Err(e) = simple_grep::read_dir_and_print_matches(&config) {
                eprintln!("Application error: {e}"); // Print to stderr

                process::exit(1);
            }
        },
        false => {
            // Handle error
            if let Err(e) = simple_grep::read_file_and_print_matches(&config) {
                eprintln!("Application error: {e}"); // Print to stderr

                process::exit(1);
            }
        }
    }
}
