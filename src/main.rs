use std::env;

use rum::rumload;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    println!("{} instructions", instructions.len());
    for instruction in instructions {
        println!("Instruction # {}", instruction);

        

        // Call the proper function.
    }
}