// Standard library
use std::error::Error; // For error handling
use std::fs; // For file stuff

// External crates
use clap::Parser; // For command-line argument parsing

#[derive(Parser, Debug)]
#[command(version)]
pub struct Argument {
    /// The string to search for
    query: String,

    /// The file to search in
    filename: String,

    /// Use case insensitive matching
    #[arg(default_value_t=false, short, long)] // Short and long refer to -i and --insensitive
    insensitive: bool,

    /// Print count of matching lines in file
    #[arg(default_value_t=false, short, long)] 
    count: bool,

    /// Match whole word
    #[arg(default_value_t=false, short, long)]
    word: bool,
}


pub fn read_file_and_print_matches(arg: Argument) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents = fs::read_to_string(arg.filename)?; // Return error (dynamic) for caller to handle

    // Print matching file contents

    if arg.count {
        let count = match arg.insensitive {
            true => case_insensitive_line_matching(&arg.query, &contents, &arg.word).len(),
            false => case_sensitive_line_matching(&arg.query, &contents, &arg.word).len(),
        };

        println!("{count}");
        return Ok(());
    }

    else {
        match arg.insensitive {
            true => case_insensitive_line_matching(&arg.query, &contents, &arg.word),
            false => case_sensitive_line_matching(&arg.query, &contents, &arg.word),
        }
        .iter()
        .for_each(|line| 
            // Make matching lines bold red

            if arg.insensitive {
                // Bold red all occurrences regardless of case

                let mut result = String::from(*line);
                let lowercase_line = line.to_lowercase();
                let lowercase_query = arg.query.to_lowercase();

                // Find all occurrences of query in line
                let mut start = 0;

                while let Some(index) = lowercase_line[start..].find(&lowercase_query) {
                    let index = index + start;
                    let end = index + arg.query.len();

                    // Replace query with bold red query
                    result = result.replace(&line[index..end], &format!("\x1b[1;31m{}\x1b[0m", &line[index..end]));

                    // Move start to end of query
                    start = end;
                }
                
                println!("{result}");
            }
            else {
        
                println!("{}", line.replace(&arg.query, &format!("\x1b[1;31m{}\x1b[0m", &arg.query)));
            }
        
        
        );
    }

    Ok(()) // Ok if sucessful
}

fn case_sensitive_line_matching<'a> (query: &str, contents: &'a str, whole_word: &bool) -> Vec<&'a str> {

    // If whole word matching is enabled, only match if query is a whole word in the line
    if *whole_word {
        contents
            .lines()
            .filter(|line| line.split_whitespace().any(|word| word == query))
            .collect()
    }

    else {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
}

fn case_insensitive_line_matching<'a> (query: &str, contents: &'a str, whole_word: &bool) -> Vec<&'a str> {

    // If whole word matching is enabled, only match if query is a whole word in the line
    if *whole_word {
        contents
            .lines()
            .filter(|line| line.to_lowercase().split_whitespace().any(|word| word == query.to_lowercase()))
            .collect()
    }

    else {
        contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Test read file and print matches */
    #[test]
    fn test_read_file_success() {
        let arg = Argument {
            query: String::from("query"),
            filename: String::from("./tests/test_poem.txt"),
            insensitive: false, // Path is based on cwd (not executable location)
            count: false,
            word: false,
        };

        let result = read_file_and_print_matches(arg);

        assert!(result.is_ok());
    }

    #[test]
    fn test_read_file_error() {
        let arg = Argument {
            query: String::from("query"),
            filename: String::from("nonexistent_file.nonsense"),
            insensitive: false,
            count: false,
            word: false,
        };

        let result = read_file_and_print_matches(arg);

        assert!(result.is_err());
    }

    /* Test case sensitive line matching */

    #[test]
    fn test_case_sensitive_line_matching_is_case_sensitive() {
        let query = "the";
        let contents = "The quick brown fox\nJumps over the lazy dog\n";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["Jumps over the lazy dog"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_no_matching_lines() {
        let query = "the";
        let contents = "No\nmatches";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_sensitive_line_matching_multiple_matching_lines() {
        let query = "the";
        let contents = "The quick brown fox\nJumps over the lazy dog\nthe end\n";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["Jumps over the lazy dog", "the end"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_one_word_per_line() {
        let query = "hello";
        let contents = "hello\nthere";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_duplicate_lines() {
        let query = "the";
        let contents = "the\nthe\nthe\n";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["the", "the", "the"]);
    }

    #[test]
    fn test_case_sensitive_line_matching_empty_contents() {
        let query = "the";
        let contents = "";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_sensitive_line_matching_empty_query() {
        let query = "";
        let contents = "The quick brown fox\nJumps over the lazy dog\n";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, contents.lines().collect::<Vec<&str>>());
    }

    #[test]
    fn test_case_sensitive_line_matching_empty_query_and_contents() {
        let query = "";
        let contents = "";

        let result = case_sensitive_line_matching(query, contents, &false);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_sensitive_whole_word_matching_works() {
        let query = "the";
        let contents = "the quick brown fox\nthere there\n";

        let result = case_sensitive_line_matching(query, contents, &true);

        assert_eq!(result, vec!["the quick brown fox"]);
    }

    /* Test case insensitive line matching */

    #[test]
    fn test_case_insensitive_line_matching_is_case_insensitive() {
        let query = "the";
        let contents = "The quick brown fox\nJumps over the lazy dog\n";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["The quick brown fox", "Jumps over the lazy dog"]);
    }

    #[test]
    fn test_case_insensitive_line_matching_no_matching_lines() {
        let query = "the";
        let contents = "No\nmatches";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_insensitive_line_matching_multiple_matching_lines() {
        let query = "the";
        let contents = "The quick brown fox\nJumps over the lazy dog\nthe end\n";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["The quick brown fox","Jumps over the lazy dog", "the end"]);
    }

    #[test]
    fn test_case_insensitive_line_matching_one_word_per_line() {
        let query = "hello";
        let contents = "hello\nthere";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_case_insensitive_line_matching_duplicate_lines() {
        let query = "the";
        let contents = "the\nthe\nThe\n";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, vec!["the", "the", "The"]);
    }

    #[test]
    fn test_case_insensitive_line_matching_empty_contents() {
        let query = "the";
        let contents = "";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_insensitive_line_matching_empty_query() {
        let query = "";
        let contents = "The quick brown fox\nJumps over the lazy dog\n";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, contents.lines().collect::<Vec<&str>>());
    }

    #[test]
    fn test_case_insensitive_line_matching_empty_query_and_contents() {
        let query = "";
        let contents = "";

        let result = case_insensitive_line_matching(query, contents, &false);

        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_case_insensitive_whole_word_matching_works() {
        let query = "the";
        let contents = "The quick brown fox\nthere there\n";

        let result = case_insensitive_line_matching(query, contents, &true);

        assert_eq!(result, vec!["The quick brown fox"]);
    }
}