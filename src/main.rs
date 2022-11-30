use std::{env};
mod lib;
use lib::{read_input, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments. Usage: get_inputs <day>");
    }
    let day = args[1].parse::<i32>().expect("Argument 1 isn't an int");
    read_input(day)?;
    Ok(())
}
