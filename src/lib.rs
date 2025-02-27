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

pub fn read_file_and_print_matches(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(config.filename)?; // Return error (dynamic) for caller to handle

    // Print matching file contents
    case_sensitive_line_matching(&config.query, &contents)
        .iter()
        .for_each(|line| println!("{line}"));

    Ok(()) // Ok if sucessful
}

pub fn case_sensitive_line_matching<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Test config */
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

    /* Test read file and print matches */
    #[test]
    fn test_read_file_success() {
        let config = Config {
            query: String::from("query"),
            filename: String::from("./tests/test_poem.txt"), // Path is based on cwd (not executable location)
        };

        let result = read_file_and_print_matches(config);

        assert!(result.is_ok());
    }

    #[test]
    fn test_read_file_error() {
        let config = Config {
            query: String::from("query"),
            filename: String::from("nonexistent_file.nonsense"),
        };

        let result = read_file_and_print_matches(config);

        assert!(result.is_err());
    }

    // TODO -> Redirect stdout to check if output is correct

    /* Test case sensitive line matching */

    #[test]
    fn test_case_sensitive_line_matching_is_case_sensitive() {
        let query = "the";
        let contents = "The quick brown fox\nJumps over the lazy dog\n";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, vec!["Jumps over the lazy dog"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_no_matching_lines() {
        let query = "the";
        let contents = "No\nmatches";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_sensitive_line_matching_multiple_matching_lines() {
        let query = "the";
        let contents = "The quick brown fox\nJumps over the lazy dog\nthe end\n";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, vec!["Jumps over the lazy dog", "the end"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_one_word_per_line() {
        let query = "hello";
        let contents = "hello\nthere";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_duplicate_lines() {
        let query = "the";
        let contents = "the\nthe\nthe\n";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, vec!["the", "the", "the"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_empty_contents() {
        let query = "the";
        let contents = "";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_sensitive_line_matching_empty_query() {
        let query = "";
        let contents = "The quick brown fox\nJumps over the lazy dog\n";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, contents.lines().collect::<Vec<&str>>());
    }

    #[test]
    fn test_case_sensitive_line_matching_empty_query_and_contents() {
        let query = "";
        let contents = "";

        let result = case_sensitive_line_matching(query, contents);

        assert_eq!(result, Vec::<&str>::new());
    }
}