use std::env;
use rum::rumload;
use rum::rumdis;
use rum::memory::Memory;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());

    let mut memory = Memory::init();

    println!("{} instructions", instructions.len());

    for instruction in instructions {
        println!("Instruction # {}", instruction);
        rumdis::execute(instruction, &mut memory);
    }
}