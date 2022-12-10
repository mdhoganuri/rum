use std::{process::exit, io::Read};
use crate::memory::Memory;

type Umi = u32;

enum Opcode { CMov, Load, Store, Add, Mul, Div, NAND, HALT, MapSeg, UMapSeg, Out, In, LP, LV }
pub struct Field { width: u32, lsb: u32 }

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

fn mask (bits: u32) -> u32 {
    (1 << bits) - 1
}

fn get (field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn execute (inst: Umi, mem: &mut Memory) {
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            // CMov
            let a = get(&RA, inst);
            let b = mem.get_register(get(&RB, inst));
            let c = mem.get_register(get(&RC, inst));

            if c != 0 {
                mem.set_register(a, b);
            }

            mem.increment_counter();
        },
        o if o == Opcode::Load as u32 => {
            // Load
            let mb = mem.get_segment(get(&RB, inst));

            mem.set_register( get(&RA, inst), mb[get(&RC, inst) as usize]);
            mem.increment_counter();
        },
        o if o == Opcode::Store as u32 => {
            // Store
            let mut ma = mem.get_segment(get(&RA, inst));

            ma[get(&RB, inst) as usize] = mem.get_register(get(&RC, inst));
            mem.increment_counter();
        },
        o if o == Opcode::Add as u32 => {
            // Add
            let a = get(&RA, inst);
            let b = mem.get_register(get(&RB, inst));
            let c = mem.get_register(get(&RC, inst));

            mem.set_register(a, (b + c) % 2u32.pow(32));
            mem.increment_counter();
        },
        o if o == Opcode::Mul as u32 => {
            // Mul
            let a = get(&RA, inst);
            let b = mem.get_register(get(&RB, inst));
            let c = mem.get_register(get(&RC, inst));

            mem.set_register(a, (b * c) % 2u32.pow(32));
            mem.increment_counter();
        },
        o if o == Opcode::Div as u32 => {
            // Div
            let a = get(&RA, inst);
            let b = mem.get_register(get(&RB, inst));
            let c = mem.get_register(get(&RC, inst));

            mem.set_register(a, b / c);
            mem.increment_counter();
        },
        o if o == Opcode::NAND as u32 => {
            // NAND
            let b = mem.get_register(get(&RA, inst));
            let c = mem.get_register(get(&RC, inst));

            mem.set_register( get(&RA, inst), !(b & c));
            mem.increment_counter();
        },
        o if o == Opcode::HALT as u32 => {
            // Halt
            exit(0);
        },
        o if o == Opcode::MapSeg as u32 => {
            // MapSeg
            let c = mem.get_register(get(&RC, inst));

            let index = mem.allocate_memory(c);
            mem.set_register(get(&RB, inst), index);
            mem.increment_counter()
        },
        o if o == Opcode::UMapSeg as u32 => {
            // UMapSeg
            mem.deallocate_memory(get(&RC, inst));
            mem.increment_counter();
        },
        o if o == Opcode::Out as u32 => {
            // Out
            print!("{}", mem.get_register(get(&RC, inst)));
            mem.increment_counter();
        },
        o if o == Opcode::In as u32 => {
            // In
            let mut buffer = [0; 1];     
            std::io::stdin().read(&mut buffer).expect("Error attempting to read a charcater.\n");

            mem.set_register(get(&RC, inst), buffer[0] as u32);
            mem.increment_counter();
        },
        o if o == Opcode::LP as u32 => {
            // LP
            let input = mem.get_segment(get(&RB, inst));
            mem.set_segment(0, input);

            let new_counter = mem.get_register(get(&RC, inst));
            mem.set_counter(new_counter);
        },
        o if o == Opcode::LV as u32 => {
            // LV
            mem.set_register(get(&RL, inst), get(&VL, inst));
            mem.increment_counter();
        },
        _ => panic!()
    }
}