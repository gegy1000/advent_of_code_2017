//! Solution to [Day 4: High-Entropy Passphrases](http://adventofcode.com/2017/day/4) for Advent of Code

use std::collections::HashSet;
use std::iter::FromIterator;
use utils::Part;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    let parsed = parse_input(input);
    let count = parsed.iter().filter(|p| p.valid(part)).count();

    Ok(count.to_string())
}

/// Parses the given input to a vec of passphrases
fn parse_input(input: &String) -> Vec<Passphrase> {
    input.split(",").map(|r| parse_passphrase(&r.to_string())).collect::<Vec<Passphrase>>()
}

/// Parses a single passphrase
fn parse_passphrase(input: &String) -> Passphrase {
    let split: Vec<&str> = input.split_whitespace().collect();
    Passphrase {
        words: split.iter().map(|v| v.trim().to_string()).collect::<Vec<String>>(),
    }
}

/// Adapts the given word for the given part.
/// In part 1, nothing is done because words need to be compared directly.
/// In part 2, characters are ordered alphabetically.
fn adapt_word(word: String, part: Part) -> String {
    if part == Part::Two {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|b, a| b.cmp(a));
        String::from_iter(chars)
    } else {
        word
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Passphrase {
    words: Vec<String>,
}

impl Passphrase {
    /// Checks if this passphrase is valid
    fn valid(&self, part: Part) -> bool {
        let mut unique = HashSet::<String>::new();
        self.words.len() > 0 && self.words.iter().all(|v| {
            unique.insert(adapt_word(v.clone().to_string(), part))
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    #[test]
    fn test_parse_passphrase() {
        let passphrase = parse_passphrase(&"aa bb cc".to_string());
        assert_eq!(passphrase.words, vec!("aa".to_string(), "bb".to_string(), "cc".to_string()));
    }

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(&"aa bb cc, aa aa aa".to_string());
        assert_eq!(parsed[0], Passphrase {
            words: vec!("aa".to_string(), "bb".to_string(), "cc".to_string()),
        });
        assert_eq!(parsed[1], Passphrase {
            words: vec!("aa".to_string(), "aa".to_string(), "aa".to_string()),
        });
    }

    #[test]
    fn test_passphrase_valid_1() {
        assert!(parse_passphrase(&"aa bb cc".to_string()).valid(Part::One));
        assert!(!parse_passphrase(&"aa aa cc".to_string()).valid(Part::One));
    }

    #[test]
    fn test_passphrase_valid_2() {
        assert!(parse_passphrase(&"abcde fghij".to_string()).valid(Part::Two));
        assert!(!parse_passphrase(&"abcde xyz ecdab".to_string()).valid(Part::Two));
        assert!(parse_passphrase(&"a ab abc abd abf abj".to_string()).valid(Part::Two));
        assert!(parse_passphrase(&"iiii oiii ooii oooi oooo".to_string()).valid(Part::Two));
        assert!(!parse_passphrase(&"oiii ioii iioi iiio".to_string()).valid(Part::Two));
    }
}
