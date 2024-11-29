use core::fmt::Debug;
use regex::Regex;
use std::fs::read_to_string;
use std::str::FromStr;

//================================================================
// Read & filter inputs of different forms
//================================================================

fn filter_regex(input: &str, regex_pattern: &str) -> Vec<String> {
    let regex = Regex::new(regex_pattern).expect("Failed to parse regex pattern");

    regex.split(input)
         .map(|s| s.to_string())
         .collect()
}

pub fn filter_input(file_name: &str, regex_pattern: &str) -> String {
    let contents = read_to_string(file_name).expect("Failed to read from input file");

    let regex = Regex::new(regex_pattern).expect("Failed to create regex pattern");

    regex.replace_all(&contents, "").to_string()
}

pub fn split_input(file_name: &str, regex_pattern: &str) -> Vec<String> {
    let contents = read_to_string(file_name).expect("Failed to read from input file");

    filter_regex(&contents, regex_pattern)
}

pub fn split_2d_input(file_name: &str, regex_pattern: &str) -> Vec<Vec<String>> {
    let contents = read_to_string(file_name).expect("Failed to read from input file");

    contents.lines()
            .map(|line| filter_regex(line, regex_pattern))
            .collect()
}

//================================================================
// Convert to integers
//================================================================

pub fn to_numeric<T>(input: Vec<String>) -> Vec<T>
where T:FromStr, T::Err: Debug {
    input.iter()
         .map(|s| s.parse::<T>().expect("Failed to parse string as integer"))
         .collect()
}

pub fn to_2d_numeric<T>(input: Vec<Vec<String>>) -> Vec<Vec<T>>
where T:FromStr, T::Err: Debug {
    input.into_iter()
         .map(|v| to_numeric::<T>(v))
         .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        let line = "1,2.3|4 5";
        let delimiters = r",|\.|\|| ";

        assert_eq!(filter_regex(line, delimiters), vec!["1","2","3","4","5"]);
    }
}