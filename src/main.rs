#![feature(test)]

mod utils;
mod days;

#[derive(Debug)]
struct UserInput {
    day: u32,
    part: utils::Part,
    input: String,
}

fn main() {
    let input = get_inputs();
    if input.is_ok() {
        run(input.unwrap());
    } else {
        println!("Error: {}", input.unwrap_err());
    }
}

/// Loads all required inputs from the user
fn get_inputs() -> Result<UserInput, String> {
    println!("Please enter a day:");
    let day = utils::read_u32().ok_or("Please enter a valid number!".to_string())?;
    println!("Please enter a part:");
    let part = utils::read_part().ok_or("Please enter a valid part number!".to_string())?;
    println!("Please enter input:");
    let input = utils::read_line().ok_or("Failed to read input!".to_string())?;
    Ok(UserInput {
        day,
        part,
        input,
    })
}

/// Runs the given solution from user input
fn run(input: UserInput) {
    let result = days::run(input.day, input.part, &input.input);
    if result.is_ok() {
        println!("Result: {}", result.unwrap());
    } else {
        println!("Error: {}", result.unwrap_err());
    }
}
