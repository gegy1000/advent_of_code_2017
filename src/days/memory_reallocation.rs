///! Solution to [Day 6: Memory Reallocation](http://adventofcode.com/2017/day/6) for Advent of Code

use utils::Part;
use std::collections::HashSet;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    let parsed = parse_input(input);

    let mut current = parsed.clone();
    let mut history = ReallocationHistory::new();
    let mut count: u32 = 0;

    while history.add(&current) {
        let reallocator = select_reallocator(&current);
        let value = current[reallocator];

        reallocate(&mut current, reallocator, value);

        count += 1;
    }

    let result = if part == Part::One {
        Some(count)
    } else {
        history.find_index(&current).map(|i| count - i as u32)
    };

    result.map(|v| v.to_string()).ok_or("Expected to find loop-back".to_string())
}

/// Reallocates the given blocks around the memory, starting after the given index
fn reallocate(memory: &mut Vec<u32>, reallocator: usize, amount: u32) {
    let mut blocks_left = amount;
    let mut current_index = reallocator;

    memory[current_index] = 0;

    while blocks_left > 0 {
        current_index = (current_index + 1) % memory.len();
        blocks_left -= 1;

        memory[current_index] += 1;
    }
}

/// Finds the highest element to perform reallocation
fn select_reallocator(memory: &Vec<u32>) -> usize {
    let mut max_index = 0;
    let mut max_value = 0;
    for i in 0..memory.len() {
        let value = memory[i];
        if value > max_value {
            max_index = i;
            max_value = value;
        }
    }
    max_index
}

/// Parses the given string input to a vector of `u32`
fn parse_input(input: &String) -> Vec<u32> {
    input.split_whitespace().filter_map(|v| v.trim().parse::<u32>().ok()).collect::<Vec<u32>>()
}

/// Holds the history of reallocation
struct ReallocationHistory {
    history_set: HashSet<Vec<u32>>,
    history_vec: Vec<Vec<u32>>,
}

impl ReallocationHistory {
    fn new() -> Self {
        ReallocationHistory {
            history_set: HashSet::new(),
            history_vec: vec!(),
        }
    }

    /// Adds a unique element to this set, returns false if given is already present
    fn add(&mut self, input: &Vec<u32>) -> bool {
        self.history_vec.push(input.clone());
        self.history_set.insert(input.clone())
    }

    /// Finds the index of the given input
    fn find_index(&self, input: &Vec<u32>) -> Option<usize> {
        for (index, element) in self.history_vec.iter().enumerate() {
            if input.eq(element) {
                return Some(index);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(&"1 2 3 4".to_string()), vec!(1, 2, 3, 4))
    }

    #[test]
    fn test_unique_history() {
        let mut history = ReallocationHistory::new();
        assert_eq!(history.add(&vec!(1, 2, 3, 4)), true);
        assert_eq!(history.add(&vec!(5, 6, 7, 8)), true);
        assert_eq!(history.add(&vec!(4, 3, 2, 1)), true);
        assert_eq!(history.add(&vec!(1, 2, 3, 4)), false);
    }

    #[test]
    fn test_select_reallocator() {
        assert_eq!(select_reallocator(&vec!(1, 2, 3, 4)), 3);
        assert_eq!(select_reallocator(&vec!(1, 2, 2, 1)), 1);
    }

    #[test]
    fn test_reallocate() {
        let mut memory = vec!(0, 2, 7, 0);

        reallocate(&mut memory, 2, 7);
        assert_eq!(memory, vec!(2, 4, 1, 2));

        reallocate(&mut memory, 1, 4);
        assert_eq!(memory, vec!(3, 1, 2, 3))
    }

    #[test]
    fn test_given_1() {
        assert_eq!(run(Part::One, &"0 2 7 0".to_string()), Ok("5".to_string()))
    }

    #[test]
    fn test_given_2() {
        assert_eq!(run(Part::Two, &"0 2 7 0".to_string()), Ok("4".to_string()))
    }
}
