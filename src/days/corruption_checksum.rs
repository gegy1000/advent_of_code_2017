///! Solution to [Day 2: Corruption Checksum](http://adventofcode.com/2017/day/2) for Advent of Code

use std::u32;
use utils::Part;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    let parsed = parse_input(input);
    let row_bounds = parsed.iter().map(|r| get_row_result(part, r)).collect::<Vec<(u32, u32)>>();
    let checksum = get_checksum(part, &row_bounds);

    Ok(checksum.to_string())
}

/// Gets the checksum for the given row bounds
fn get_checksum(part: Part, row_bounds: &Vec<(u32, u32)>) -> u32 {
    if part == Part::One {
        row_bounds.iter()
            .fold(0, |acc, i| acc + (i.1 - i.0))
    } else {
        row_bounds.iter()
            .fold(0, |acc, i| acc + (i.0 / i.1))
    }
}

/// Gets the input for the given row and part
fn get_row_result(part: Part, row: &Vec<u32>) -> (u32, u32) {
    if part == Part::One { find_bounds(row) } else { find_divisible(row) }
}

/// Finds the lower and upper bounds in a tuple of (lower, upper)
fn find_bounds(row: &Vec<u32>) -> (u32, u32) {
    let mut lower = u32::MAX;
    let mut upper = u32::MIN;
    for v in row {
        if *v < lower {
            lower = *v;
        }
        if *v > upper {
            upper = *v;
        }
    }
    (lower, upper)
}

/// Finds the two numbers in the given row that are divisible by each other
fn find_divisible(row: &Vec<u32>) -> (u32, u32) {
    for first in row {
        for second in row {
            if first != second && *first % *second == 0 {
                return if *first > *second { (*first, *second)} else { (*second, *first) }
            }
        }
    }
    (1, 1)
}

/// Parses the given row/column input to a 2D vector of u32
fn parse_input(input: &String) -> Vec<Vec<u32>> {
    input.split(",").map(|r| {
        let split: Vec<&str> = r.split_whitespace().collect();
        split.iter().filter_map(|v| v.trim().parse::<u32>().ok()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(&"1 2 3, 4 5 6, 7 8 9".to_string());
        let expected = vec!(vec!(1, 2, 3), vec!(4, 5, 6), vec!(7, 8, 9));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_find_bounds() {
        let row = vec!(1, 6, 2, 9);
        assert_eq!(find_bounds(&row), (1, 9));
    }

    #[test]
    fn test_find_divisible() {
        let row = vec!(5, 9, 2, 8);
        assert_eq!(find_divisible(&row), (8, 2));
    }

    #[test]
    fn test_get_checksum_1() {
        let row_bounds = vec!((1, 9), (4, 5));
        assert_eq!(get_checksum(Part::One, &row_bounds), 8 + 1)
    }

    #[test]
    fn test_get_checksum_2() {
        let row_divisibility = vec!((8, 2), (9, 3));
        assert_eq!(get_checksum(Part::Two, &row_divisibility), 4 + 3)
    }

    #[test]
    fn test_given_1() {
        let input = "5 1 9 5, 7 5 3, 2 4 6 8".to_string();
        assert_eq!(run(Part::One, &input), Ok("18".to_string()))
    }

    #[test]
    fn test_given_2() {
        let input = "5 9 2 8, 9 4 7 3, 3 8 6 5".to_string();
        assert_eq!(run(Part::Two, &input), Ok("9".to_string()))
    }

    #[test]
    fn test_multi_digit() {
        let input = "12 10 4, 5 2 12, 15 8 5".to_string();
        assert_eq!(run(Part::One, &input), Ok("28".to_string()))
    }
}
