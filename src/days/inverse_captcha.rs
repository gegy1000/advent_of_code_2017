///! Solution to [Day 1: Inverse Captcha](http://adventofcode.com/2017/day/1) for Advent of Code

use utils::Part;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    let parsed = parse_input(&input);
    let mut total = 0;
    let offset = get_offset(part, parsed.len());

    for index in 0..parsed.len() {
        // Get the wrapped partner for this index
        let partner_index = (index + offset) % parsed.len();

        let current = parsed[index];
        let partner = parsed[partner_index];

        if current == partner {
            total += current;
        }
    }

    Ok(total.to_string())
}

/// Gets the partner offset for the given part and parsed input
fn get_offset(part: Part, parsed_length: usize) -> usize {
    if part == Part::Two {
        parsed_length / 2
    } else {
        1
    }
}

/// Parses a string of digits into a [Vec<u32>]
fn parse_input(input: &String) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[test]
    fn test_parse_input() {
        let input = &"1234".to_string();
        assert_eq!(parse_input(input), vec!(1, 2, 3, 4));
    }

    #[test]
    fn test_offset_part_1() {
        let offset = get_offset(Part::One, 10);
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_offset_part_2() {
        let offset = get_offset(Part::Two, 10);
        assert_eq!(offset, 5);
    }

    #[test]
    fn test_given_part_1() {
        assert_eq!(run(Part::One, &"1122".to_string()).unwrap(), "3");
        assert_eq!(run(Part::One, &"1111".to_string()).unwrap(), "4");
        assert_eq!(run(Part::One, &"1234".to_string()).unwrap(), "0");
        assert_eq!(run(Part::One, &"91212129".to_string()).unwrap(), "9");
    }

    #[test]
    fn test_given_part_2() {
        assert_eq!(run(Part::Two, &"1212".to_string()).unwrap(), "6");
        assert_eq!(run(Part::Two, &"1221".to_string()).unwrap(), "0");
        assert_eq!(run(Part::Two, &"123425".to_string()).unwrap(), "4");
        assert_eq!(run(Part::Two, &"123123".to_string()).unwrap(), "12");
        assert_eq!(run(Part::Two, &"12131415".to_string()).unwrap(), "4");
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| {
            let input = test::black_box("68376334".to_string());
            run(Part::One, &input)
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| {
            let input = test::black_box("68376334".to_string());
            run(Part::Two, &input)
        });
    }
}
