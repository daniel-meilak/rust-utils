use regex::Regex;
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::{Add, Rem};
use std::str::FromStr;

//================================================================
// Split and filter strings
//================================================================

pub fn filter_by_regex(input: &str, regex_pattern: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(regex_pattern)?;

    Ok(regex.replace_all(input, "").to_string())
}

pub fn split_by_regex<'a>(input: &'a str, regex_pattern: &str) -> Result<Vec<&'a str>, regex::Error> {
    let regex = Regex::new(regex_pattern)?;

    Ok(regex.split(input).collect())
}

pub fn split_2d_by_regex<'a>(input: &'a str, regex_pattern: &str) -> Result<Vec<Vec<&'a str>>, regex::Error> {
    let regex = Regex::new(regex_pattern)?;

    Ok(input.lines().map(|line| regex.split(line).collect()).collect())
}

//================================================================
// Read, split & filter input file contents
//================================================================

pub fn filter_input(file_name: &str, regex_pattern: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = read_to_string(file_name)?;

    Ok(filter_by_regex(&contents, regex_pattern)?)
}

pub fn split_input(file_name: &str, regex_pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contents = read_to_string(file_name)?;

    Ok(split_by_regex(&contents, regex_pattern)?.into_iter().map(|s| s.to_string()).collect())
}

pub fn split_2d_input(file_name: &str, regex_pattern: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let contents = read_to_string(file_name)?;

    Ok(split_2d_by_regex(&contents, regex_pattern)?
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .collect())
}

//================================================================
// Convert to integers
//================================================================

pub fn to_numeric<S, T>(input: &[S]) -> Result<Vec<T>, T::Err>
where
    S: AsRef<str>,
    T: FromStr,
{
    input.iter().map(|s| s.as_ref().parse::<T>()).collect()
}

pub fn to_2d_numeric<A, B, S, T>(input: A) -> Result<Vec<Vec<T>>, T::Err>
where
    A: AsRef<[B]>,
    B: AsRef<[S]>,
    S: AsRef<str>,
    T: FromStr,
{
    input.as_ref().iter().map(|v| to_numeric::<S, T>(v.as_ref())).collect()
}

//================================================================
// Grid utilities
//================================================================

pub fn sum_column<A, B, T>(grid: A, n: usize) -> Option<T>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Sum + Copy,
{
    let grid_ref = grid.as_ref();
    if grid_ref.is_empty() || n >= grid_ref[0].as_ref().len() {
        return None;
    }

    Some(grid_ref.iter().map(|row| row.as_ref()[n]).sum())
}

pub fn sum_row<A, B, T>(grid: A, n: usize) -> Option<T>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Sum + Copy,
{
    Some(grid.as_ref().get(n)?.as_ref().iter().copied().sum())
}

pub fn min_column<A, B, T>(grid: A, n: usize) -> Option<T>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy,
{
    let grid_ref = grid.as_ref();
    if grid_ref.is_empty() || n >= grid_ref[0].as_ref().len() {
        return None;
    }
    grid_ref.iter().map(|row| row.as_ref()[n]).min()
}

pub fn max_column<A, B, T>(grid: A, n: usize) -> Option<T>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy,
{
    let grid_ref = grid.as_ref();
    if grid_ref.is_empty() || n >= grid_ref[0].as_ref().len() {
        return None;
    }
    grid_ref.iter().map(|row| row.as_ref()[n]).max()
}

pub fn min_row<A, B, T>(grid: A, n: usize) -> Option<T>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy,
{
    grid.as_ref().get(n)?.as_ref().iter().copied().min()
}

pub fn max_row<A, B, T>(grid: A, n: usize) -> Option<T>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Ord + Copy,
{
    grid.as_ref().get(n)?.as_ref().iter().copied().max()
}

pub fn rotate<A, B, T>(grid: A) -> Vec<Vec<T>>
where
    A: AsRef<[B]>,
    B: AsRef<[T]>,
    T: Copy,
{
    let max_column_size = grid.as_ref().iter().map(|row| row.as_ref().len()).max().unwrap_or(0);

    let mut rotated: Vec<Vec<T>> = vec![Vec::new(); max_column_size];

    for row in grid.as_ref() {
        for (i, &element) in row.as_ref().iter().enumerate() {
            rotated[i].push(element);
        }
    }

    rotated
}

pub fn pad(input: &str, filler: char) -> Option<Vec<Vec<char>>> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() || lines.iter().all(|line| line.is_empty()) {
        return None;
    }

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
    let border: Vec<char> = std::iter::repeat_n(filler, border_width).collect();

    let mut result = vec![border.clone()];
    result.extend(padded_lines);
    result.push(border);

    Some(result)
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
        assert_eq!(filter_by_regex(SINGLE_LINE, DELIMITERS).unwrap(), FILTERED_SINGLE_LINE);
    }

    #[test]
    fn split_string() {
        assert_eq!(split_by_regex(SINGLE_LINE, DELIMITERS).unwrap(), SPLIT_SINGLE_LINE.to_vec());
    }

    #[test]
    fn filter_text_file() {
        let path = "filter.txt";
        create_input(path, SINGLE_LINE);
        assert_eq!(filter_input(path, DELIMITERS).unwrap(), FILTERED_SINGLE_LINE);
        remove_input(path);
    }

    #[test]
    fn split_text_file() {
        let path = "split.txt";
        create_input(path, SINGLE_LINE);
        assert_eq!(split_input(path, DELIMITERS).unwrap(), SPLIT_SINGLE_LINE.to_vec());
        remove_input(path);
    }

    #[test]
    fn split_2d_text_file() {
        let path = "split_2d.txt";
        create_input(path, MULTI_LINE);
        assert_eq!(
            split_2d_input(path, DELIMITERS).unwrap(),
            vec![vec!["1", "1"], vec!["2", "2"], vec!["3", "3"]]
        );
        remove_input(path);
    }

    #[test]
    fn numeric_conversions() {
        assert_eq!(to_numeric::<&str, i32>(&SPLIT_SINGLE_LINE).unwrap(), NUMERIC_SINGLE_LINE.to_vec());
        assert_eq!(
            to_2d_numeric::<&[&[&str]], &[&str], &str, i32>(&[&["1", "1"], &["2", "2"], &["3", "3"]]).unwrap(),
            &[vec![1, 1], vec![2, 2], vec![3, 3]]
        );
    }

    #[test]
    fn grid_summation() {
        let grid = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

        assert_eq!(sum_column(grid, 0), Some(12));
        assert_eq!(sum_column(grid, 1), Some(15));
        assert_eq!(sum_column(grid, 2), Some(18));

        assert_eq!(sum_row(grid, 0), Some(6));
        assert_eq!(sum_row(grid, 1), Some(15));
        assert_eq!(sum_row(grid, 2), Some(24));
    }

    #[test]
    fn grid_min_max() {
        let grid = &[&[1, 2], &[5, 4]];

        assert_eq!(min_column(grid, 0), Some(1));
        assert_eq!(max_column(grid, 0), Some(5));

        assert_eq!(min_row(grid, 0), Some(1));
        assert_eq!(max_row(grid, 0), Some(2));
    }

    #[test]
    fn grid_rotate() {
        let grid = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

        assert_eq!(rotate(grid), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]])
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
        assert_eq!(
            pad("aa\naa", '#'),
            Some(vec![
                vec!['#', '#', '#', '#'],
                vec!['#', 'a', 'a', '#'],
                vec!['#', 'a', 'a', '#'],
                vec!['#', '#', '#', '#']
            ])
        );
    }

    #[test]
    fn empty_grid() {
        let grid: &[&[i32]] = &[];
        assert_eq!(sum_column(grid, 0), None);
    }

    #[test]
    fn out_of_bounds() {
        let grid = &[&[1, 2]];
        assert_eq!(sum_column(grid, 5), None);
        assert_eq!(sum_row(grid, 5), None);
    }

    #[test]
    fn pad_empty() {
        assert_eq!(pad("", '#'), None);
    }
}
