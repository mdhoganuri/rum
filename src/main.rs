use std::env;
use rum::rumload;
use rum::instruct;
use rum::memory::Memory;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());

    let mut memory = Memory::init();

    println!("{} instructions", instructions.len());

    loop {
        println!("Instruction # {}", memory.get_counter());
        instruct::execute(*instructions.get(memory.get_counter() as usize).unwrap(), &mut memory);
    }
}