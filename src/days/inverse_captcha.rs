///! Solution to [Day 1: Inverse Captcha](http://adventofcode.com/2017/day/1) for Advent of Code

use utils::Part;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    if let Ok(parsed) = parse_input(&input) {
        let mut total = 0;
        let offset = get_offset(part, &parsed);

        for index in 0..parsed.len() {
            // Get the wrapped partner for this index
            let partner_index = (index + offset) % (parsed.len() - 1);

            let current = parsed[index];
            let partner = parsed[partner_index];

            if current == partner {
                total += current;
            }
        }

        Ok(total.to_string())
    } else {
        Err("Failed to parse input".to_string())
    }
}

/// Gets the partner offset for the given part and parsed input
fn get_offset(part: Part, parsed: &Vec<u32>) -> usize {
    if part == Part::Two {
        parsed.len() / 2
    } else {
        1
    }
}

/// Parses a string of digits into a [Vec<u32>]
fn parse_input(input: &String) -> Result<Vec<u32>, ()> {
    let mut parsed: Vec<u32> = vec!();
    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            parsed.push(digit);
        } else {
            return Err(())
        }
    }
    Ok(parsed)
}
