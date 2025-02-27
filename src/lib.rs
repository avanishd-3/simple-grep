use std::error::Error; // For error handling
use std::fs; // For file stuff

// Parse command-line arguments

#[derive(PartialEq)]
#[derive(Debug)] // To check if Config properly makes errors if there are not enough arguments
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn make(args: &Vec<String>) -> Result<Config, &'static str> {
        // Error string is literal, so static should be fine here

        // Check for correct number of arguments -> ignore extras (they don't matter)
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Remove clone calls later
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn read_file(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.filename)?; // Return error (dynamic) for caller to handle

    println!("With text:\n{}", contents);

    Ok(()) // Ok if sucessful
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_config_successfully() {
        let args = vec![
            String::from("simple_grep"),
            String::from("query"),
            String::from("filename"),
        ];

        let config = Config::make(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn test_make_config_not_enough_args() {
        let args = vec![String::from("simple_grep")];

        let config = Config::make(&args);

        assert_eq!(config, Err("Not enough arguments"));
    }

    #[test]
    fn test_make_config_ignores_extra_args() {
        let args = vec![
            String::from("simple_grep"),
            String::from("query"),
            String::from("filename"),
            String::from("extra"),
        ];

        let config = Config::make(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn test_read_file_success() {
        let config = Config {
            query: String::from("query"),
            filename: String::from("./tests/test_poem.txt"), // Path is based on cwd (not executable location)
        };

        let result = read_file(config);

        assert!(result.is_ok());
    }

    #[test]
    fn test_read_file_error() {
        let config = Config {
            query: String::from("query"),
            filename: String::from("nonexistent_file.nonsense"),
        };

        let result = read_file(config);

        assert!(result.is_err());
    }
}