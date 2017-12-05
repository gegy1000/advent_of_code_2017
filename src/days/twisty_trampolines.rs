///! Solution to [Day 5: A Maze of Twisty Trampolines, All Alike](http://adventofcode.com/2017/day/5) for Advent of Code

use utils::Part;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    let mut instructions = parse_input(input);
    let mut pointer: i32 = 0;
    let mut steps: u32 = 0;
    while pointer >= 0 && pointer < instructions.len() as i32 {
        pointer = move_pointer(&mut instructions, pointer, part);
        steps += 1;
    }
    Ok(steps.to_string())
}

/// Moves the pointer with the given instruction and increments
fn move_pointer(instructions: &mut Vec<i32>, pointer: i32, part: Part) -> i32 {
    let index = pointer as usize;

    let instruction = instructions[index];

    if part == Part::One {
        instructions[index] += 1;
    } else {
        instructions[index] += if instruction >= 3 { -1 } else { 1 }
    }

    pointer + instruction
}

/// Parses the given input into a mutable vec of instructions
fn parse_input(input: &String) -> Vec<i32> {
    input.split_whitespace().filter_map(|v| v.parse::<i32>().ok()).collect::<Vec<i32>>()
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
    fn test_increment_pointer_1() {
        let mut instructions = vec!(0, 3, 0, 1, -3);
        let mut pointer: i32 = 0;
        pointer = move_pointer(&mut instructions, pointer, Part::One);
        assert_eq!(pointer, 0);
        pointer = move_pointer(&mut instructions, pointer, Part::One);
        assert_eq!(pointer, 1);
        pointer = move_pointer(&mut instructions, pointer, Part::One);
        assert_eq!(pointer, 4);
        pointer = move_pointer(&mut instructions, pointer, Part::One);
        assert_eq!(pointer, 1);
        pointer = move_pointer(&mut instructions, pointer, Part::One);
        assert_eq!(pointer, 5);
    }

    #[test]
    fn test_increment_pointer_2() {
        let mut instructions = vec!(0, 3, 0, 1, -3);
        let mut pointer: i32 = 0;
        pointer = move_pointer(&mut instructions, pointer, Part::Two);
        assert_eq!(pointer, 0);
        pointer = move_pointer(&mut instructions, pointer, Part::Two);
        assert_eq!(pointer, 1);
        pointer = move_pointer(&mut instructions, pointer, Part::Two);
        assert_eq!(pointer, 4);
        pointer = move_pointer(&mut instructions, pointer, Part::Two);
        assert_eq!(pointer, 1);
        pointer = move_pointer(&mut instructions, pointer, Part::Two);
        assert_eq!(pointer, 3);
    }

    #[test]
    fn test_given_1() {
        assert_eq!(run(Part::One, &"0 3 0 1 -3".to_string()), Ok("5".to_string()))
    }

    #[test]
    fn test_given_2() {
        assert_eq!(run(Part::Two, &"0 3 0 1 -3".to_string()), Ok("10".to_string()))
    }
}
