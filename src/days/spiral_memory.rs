//! Solution to [Day 3: Spiral Memory](http://adventofcode.com/2017/day/3) for Advent of Code
//! Part 2 solution not included.

use utils::Part;

/// Runs this solution
pub fn run(_: Part, input: &String) -> Result<String, String> {
    let parsed_input = input.parse::<u32>().map_err(|_| "Invalid input")?;
    let point = get_memory_location(parsed_input);
    let steps = get_steps(point);

    Ok(steps.to_string())
}

fn get_memory_location(index: u32) -> (i32, i32) {
    if index == 1 {
        return (0, 0);
    }

    // I'm too lazy to figure out how to calculate this properly. I'm so terribly sorry to anyone reading this.
    let mut ring = 1;
    let mut current = 2u32;

    loop {
        let size = get_ring_size(ring);
        let half_size = size / 2;

        let mut move_x = 0;
        let mut move_y = -1;

        let mut x = half_size;
        let mut y = 0;

        loop {
            if current == index {
                return (x, y);
            }

            current += 1;
            x += move_x;
            y += move_y;

            if y == -half_size && move_y < 0 {
                move_y = 0;
                move_x = -1;
            }
            if x == -half_size && move_x < 0 {
                move_x = 0;
                move_y = 1;
            }
            if y == half_size && move_y > 0 {
                move_y = 0;
                move_x = 1;
            }
            if x == half_size && move_x > 0 {
                move_x = 0;
                move_y = -1;
            }

            if x == half_size && y == 0 {
                break;
            }
        }

        current += 1;
        ring += 1;
    }
}

fn get_steps(point: (i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}

fn get_ring_size(ring: i32) -> i32 {
    2 * ring + 1
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[test]
    fn test_given_locations_1() {
        assert_eq!(get_memory_location(1), (0, 0));
        assert_eq!(get_memory_location(2), (1, 0));
        assert_eq!(get_memory_location(3), (1, -1));
        assert_eq!(get_memory_location(4), (0, -1));
        assert_eq!(get_memory_location(12), (2, -1));
    }

    #[test]
    fn test_given_1() {
        assert_eq!(run(Part::One, &"1".to_string()), Ok("0".to_string()));
        assert_eq!(run(Part::One, &"12".to_string()), Ok("3".to_string()));
        assert_eq!(run(Part::One, &"23".to_string()), Ok("2".to_string()));
        assert_eq!(run(Part::One, &"1024".to_string()), Ok("31".to_string()));
    }
}
