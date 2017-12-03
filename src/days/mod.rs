mod inverse_captcha;
mod corruption_checksum;
mod spiral_memory;

use utils;

/// Runs the code for the given day
pub fn run(day: u32, part: utils::Part, input: &String) -> Result<String, String> {
    match day {
        1 => inverse_captcha::run(part, input),
        2 => corruption_checksum::run(part, input),
        3 => spiral_memory::run(part, input),
        _ => Err("Given day does not exist".to_string())
    }
}
