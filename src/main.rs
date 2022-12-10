use std::env;
use rum::rumload;
use rum::manager::Manager;
use rum::instructions;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut manager = Manager::new();

    loop {
        instructions::execute(instructions[manager.counter as usize], &mut manager)
    }
}