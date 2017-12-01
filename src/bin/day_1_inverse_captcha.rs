use std::io;

fn main() {
    let mut input = String::new();
    let part = check_part();

    if part == 1 || part == 2 {
        println!("Please enter the puzzle input:");

        if let Ok(_) = io::stdin().read_line(&mut input) {
            let mut total = 0;
            let parsed = parse_input(&input);

            let offset = if part == 2 { parsed.len() / 2 } else { 1 };

            for index in 0..parsed.len() {
                let next_index = (index + offset) % (parsed.len() - 1);

                let current = parsed[index];
                let next = parsed[next_index];

                if current == next {
                    total += current;
                }
            }

            println!("Result: {}", total);
        } else {
            println!("Failed to read input from stdin");
        }
    } else {
        println!("Please enter a valid part! Expected 1 or 2, got {}", part);
    }
}

fn check_part() -> u8 {
    println!("Please enter the part number to calculate for:");

    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let new_length = input.len() - 1;
        input.truncate(new_length);
        input.parse::<u8>().unwrap()
    } else {
        println!("Failed to read input from stdin");
        0
    }
}

fn parse_input(input: &String) -> Vec<u32> {
    let mut parsed: Vec<u32> = vec!();
    for c in input.chars() {
        parsed.push(c.to_digit(10).unwrap_or(0));
    }
    parsed
}
