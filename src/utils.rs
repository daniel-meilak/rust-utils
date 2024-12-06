use core::fmt::Debug;
use std::result;
use regex::Regex;
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::{Add, Rem};
use std::str::FromStr;

//================================================================
// Split and filter strings
//================================================================

pub fn filter_by_regex(input: &str, regex_pattern: &str) -> String {
    let regex = Regex::new(regex_pattern).expect("Failed to create regex pattern");

    regex.replace_all(input, "").to_string()
}

pub fn split_by_regex(input: &str, regex_pattern: &str) -> Vec<String> {
    let regex = Regex::new(regex_pattern).expect("Failed to parse regex pattern");

    regex.split(input).map(|s| s.to_string()).collect()
}

pub fn split_2d_by_regex(input: &str, regex_pattern: &str) -> Vec<Vec<String>> {
    let regex = Regex::new(regex_pattern).expect("Failed to parse regex pattern");

    input
        .lines()
        .map(|line| regex.split(line).map(|s| s.to_string()).collect())
        .collect()
}

//================================================================
// Read, split & filter input file contents
//================================================================

pub fn filter_input(file_name: &str, regex_pattern: &str) -> String {
    let contents = read_to_string(file_name).expect("Failed to read from input file");

    filter_by_regex(&contents, regex_pattern)
}

pub fn split_input(file_name: &str, regex_pattern: &str) -> Vec<String> {
    let contents = read_to_string(file_name).expect("Failed to read from input file");

    split_by_regex(&contents, regex_pattern)
}

pub fn split_2d_input(file_name: &str, regex_pattern: &str) -> Vec<Vec<String>> {
    let contents = read_to_string(file_name).expect("Failed to read from input file");

    split_2d_by_regex(&contents, regex_pattern)
}

//================================================================
// Convert to integers
//================================================================

pub fn to_numeric<S, T>(input: &[S]) -> Vec<T>
where
    S: AsRef<str>,
    T: FromStr,
    T::Err: Debug,
{
    input
        .iter()
        .map(|s| {
            s.as_ref()
                .parse::<T>()
                .expect("Failed to parse string as integer")
        })
        .collect()
}

pub fn to_2d_numeric<A, B, S, T>(input: A) -> Vec<Vec<T>>
where
    A: AsRef<[B]>,
    B: AsRef<[S]>,
    S: AsRef<str>,
    T: FromStr,
    T::Err: Debug,
{
    input
        .as_ref()
        .iter()
        .map(|v| to_numeric::<S, T>(v.as_ref()))
        .collect()
}

//================================================================
// Grid utilities
//================================================================

pub fn sum_column<A, B, T>(grid: A, n: usize) -> T
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Sum + Copy,
{
    grid.as_ref().iter().map(|row| row.as_ref()[n]).sum()
}

pub fn sum_row<A, B, T>(grid: A, n: usize) -> T
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Sum + Copy,
{
    grid.as_ref()[n].as_ref().iter().copied().sum()
}

pub fn min_column<A, B, T>(grid: A, n: usize) -> T
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy
{
    grid.as_ref()
        .iter()
        .map(|row| row.as_ref()[n])
        .min()
        .expect("Cannot find minimum of empty array")
}

pub fn max_column<A, B, T>(grid: A, n: usize) -> T
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy
{
    grid.as_ref()
        .iter()
        .map(|row| row.as_ref()[n])
        .max()
        .expect("Cannot find maximum of empty array")
}

pub fn min_row<A, B, T>(grid: A, n: usize) -> T
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy
{
    grid.as_ref()[n]
        .as_ref()
        .iter()
        .copied()
        .min()
        .expect("Cannot find minimum of empty array")
}

pub fn max_row<A, B, T>(grid: A, n: usize) -> T
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy
{
    grid.as_ref()[n]
        .as_ref()
        .iter()
        .copied()
        .max()
        .expect("Cannot find maximum of empty array")
}

pub fn rotate<A, B, T>(grid: A) -> Vec<Vec<T>>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Copy
{
    let max_column_size = grid
        .as_ref()
        .iter()
        .map(|row| row.as_ref().len())
        .max()
        .unwrap_or(0);

    let mut rotated: Vec<Vec<T>> = vec![Vec::new(); max_column_size];

    for row in grid.as_ref() {
        for (i, &element) in row.as_ref().iter().enumerate() {
            rotated[i].push(element);
        }
    }

    rotated
}

pub fn pad(input: &str, filler: char) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();

    let padded_lines: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(0, filler); // Add 'X' at the beginning
            chars.push(filler); // Add 'X' at the end
            chars
        })
        .collect();

    let border_width = padded_lines[0].len();
    let border: Vec<char> = std::iter::repeat(filler).take(border_width).collect();

    let mut result = vec![border.clone()];
    result.extend(padded_lines);
    result.push(border);

    result
}

//================================================================
// Other utilities
//================================================================

pub fn modulus<T>(lhs: T, rhs: T) -> T
where
    T: Rem<Output = T> + Add<Output = T> + Copy,
{
    ((lhs % rhs) + rhs) % rhs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, write};

    const DELIMITERS: &str = ",|\\.|\\|| ";
    const SINGLE_LINE: &str = "1,2.3|4 5";
    const FILTERED_SINGLE_LINE: &str = "12345";
    const SPLIT_SINGLE_LINE: [&str; 5] = ["1", "2", "3", "4", "5"];
    const NUMERIC_SINGLE_LINE: [i32; 5] = [1, 2, 3, 4, 5];
    const MULTI_LINE: &str = "1,1\n2.2\n3 3";

    fn create_input(path: &str, contents: &str) {
        let test_file = path;
        write(test_file, contents).expect("Failed to create to input file");
    }

    fn remove_input(path: &str) {
        remove_file(path).expect("Failed to delete input file");
    }

    #[test]
    fn filter_string() {
        assert_eq!(
            filter_by_regex(SINGLE_LINE, DELIMITERS),
            FILTERED_SINGLE_LINE
        );
    }

    #[test]
    fn split_string() {
        assert_eq!(
            split_by_regex(SINGLE_LINE, DELIMITERS),
            SPLIT_SINGLE_LINE.to_vec()
        );
    }

    #[test]
    fn filter_text_file() {
        let path = "filter.txt";
        create_input(path, SINGLE_LINE);
        assert_eq!(filter_input(path, DELIMITERS), FILTERED_SINGLE_LINE);
        remove_input(path);
    }

    #[test]
    fn split_text_file() {
        let path = "split.txt";
        create_input(path, SINGLE_LINE);
        assert_eq!(split_input(path, DELIMITERS), SPLIT_SINGLE_LINE.to_vec());
        remove_input(path);
    }

    #[test]
    fn split_2d_text_file() {
        let path = "split_2d.txt";
        create_input(path, MULTI_LINE);
        assert_eq!(
            split_2d_input(path, DELIMITERS),
            vec![vec!["1", "1"], vec!["2", "2"], vec!["3", "3"]]
        );
        remove_input(path);
    }

    #[test]
    fn numeric_conversions() {
        assert_eq!(
            to_numeric::<&str, i32>(&SPLIT_SINGLE_LINE),
            NUMERIC_SINGLE_LINE.to_vec()
        );
        assert_eq!(
            to_2d_numeric::<&[&[&str]], &[&str], &str, i32>(&[
                &["1", "1"],
                &["2", "2"],
                &["3", "3"]
            ]),
            &[vec![1, 1], vec![2, 2], vec![3, 3]]
        );
    }

    #[test]
    fn grid_summation() {
        let grid = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

        assert_eq!(sum_column(grid, 0), 12);
        assert_eq!(sum_column(grid, 1), 15);
        assert_eq!(sum_column(grid, 2), 18);

        assert_eq!(sum_row(grid, 0), 6);
        assert_eq!(sum_row(grid, 1), 15);
        assert_eq!(sum_row(grid, 2), 24);
    }

    #[test]
    fn grid_min_max() {
        let grid = &[&[1, 2], &[5, 4]];

        assert_eq!(min_column(grid, 0), 1);
        assert_eq!(max_column(grid, 0), 5);

        assert_eq!(min_row(grid, 0), 1);
        assert_eq!(max_row(grid, 0), 2);
    }

    #[test]
    fn grid_rotate() {
        let grid = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

        assert_eq!(rotate(grid), vec![vec![1,4,7],vec![2,5,8],vec![3,6,9]])
    }

    #[test]
    fn mathematical_modulus() {
        let a = 7;
        let b = -7;

        assert_eq!(modulus(a, 5), 2);
        assert_eq!(modulus(b, 5), 3);
    }

    #[test]
    fn padding() {
        assert_eq!(pad("aa\naa", '#'), vec!["####","#aa#","#aa#","####"]);
    }
}
