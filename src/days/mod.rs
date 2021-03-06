mod inverse_captcha;
mod corruption_checksum;
mod spiral_memory;
mod passphrases;
mod twisty_trampolines;
mod memory_reallocation;
mod recursive_circus;

use utils;

/// Runs the code for the given day
pub fn run(day: u32, part: utils::Part, input: &String) -> Result<String, String> {
    match day {
        1 => inverse_captcha::run(part, input),
        2 => corruption_checksum::run(part, input),
        3 => spiral_memory::run(part, input),
        4 => passphrases::run(part, input),
        5 => twisty_trampolines::run(part, input),
        6 => memory_reallocation::run(part, input),
        7 => recursive_circus::run(part, input),
        _ => Err("Given day does not exist".to_string())
    }
}
