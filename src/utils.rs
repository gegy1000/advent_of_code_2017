use std::io;

/// Reads a u32 from stdin
pub fn read_u32() -> Option<u32> {
    read_line().map(|input| input.parse::<u32>().unwrap())
}

/// Reads a day part from stdin
pub fn read_part() -> Option<Part> {
    read_line().and_then(|input| parse_part(&input))
}

/// Reads a line from stdin and truncates the newline character
pub fn read_line() -> Option<String> {
    // Read a string from stdin
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        // Truncate the newline character
        let new_length = input.len() - 1;
        input.truncate(new_length);

        Some(input)
    } else {
        None
    }
}

fn parse_part(input: &String) -> Option<Part> {
    if let Ok(parsed) = input.parse::<u8>() {
        match parsed {
            1 => Some(Part::One),
            2 => Some(Part::Two),
            _ => None
        }
    } else {
        None
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Part {
    One,
    Two,
}
