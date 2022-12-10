use std::{process::exit, io::Read};
use crate::manager::Manager;

pub struct Field { width: u32, lsb: u32 }

static OP: Field = Field {width: 4, lsb: 28};

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};

static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};

enum Opcode { CMov, Load, Store, Add, Mul, Div, NAND, HALT, MapSeg, UMapSeg, Out, In, LP, LV }

fn mask (bits: u32) -> u32 {
    (1 << bits) - 1
}

pub fn get (field: &Field, instruction: u32) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn execute (ins: u32, mem: &mut Manager) {
    let a_idx = get(&RA, ins) as usize;
    let b_idx = get(&RB, ins) as usize;
    let c_idx = get(&RC, ins) as usize;

    match get(&OP, ins) {
        // CMove Instruction
        o if o == Opcode::CMov as u32 => {
            if mem.registers[c_idx] != 0 {
                mem.registers[a_idx] = mem.registers[b_idx];
            }

            mem.counter += 1;
        },
        // Load Instruction
        o if o == Opcode::Load as u32 => {
            mem.registers[a_idx] = mem.memory[b_idx][c_idx];

            mem.counter += 1;
        },
        // Store Instruction
        o if o == Opcode::Store as u32 => {
            mem.memory[a_idx][b_idx] = mem.registers[c_idx];

            mem.counter += 1;
        },
        // Add Instruction
        o if o == Opcode::Add as u32 => {
            mem.registers[a_idx] = mem.registers[b_idx].wrapping_add(mem.registers[c_idx]);

            mem.counter += 1;
        },
        // Mul Instruction
        o if o == Opcode::Mul as u32 => {
            mem.registers[a_idx] = mem.registers[b_idx].wrapping_mul(mem.registers[c_idx]);

            mem.counter += 1;
        },
        // Div Instruction
        o if o == Opcode::Div as u32 => {
            mem.registers[a_idx] = mem.registers[b_idx] / mem.registers[c_idx];

            mem.counter += 1;
        },
        // NAND Instruction
        o if o == Opcode::NAND as u32 => {
            mem.registers[a_idx] = !(mem.registers[b_idx] & mem.registers[c_idx]);

            mem.counter += 1;
        },
        // HALT Instruction
        o if o == Opcode::HALT as u32 => {
            exit(0);
        },
        // MapSeg Instruction
        o if o == Opcode::MapSeg as u32 => {
            let idx: u32;

            if mem.unmapped.len() > 0 {
                idx = mem.unmapped.pop().unwrap();
                mem.memory[idx as usize] = vec![0; mem.registers[c_idx] as usize];
            } else {
                idx = mem.memory.len() as u32;
                mem.memory.push(vec![0; mem.registers[c_idx] as usize]);
            }

            mem.registers[b_idx] = idx;

            mem.counter += 1;
        },
        // UMapSeg Instruction
        o if o == Opcode::UMapSeg as u32 => {
            mem.unmapped.push(mem.registers[c_idx]);
            mem.memory[c_idx].clear();

            mem.counter += 1;
        },
        // Out Instruction
        o if o == Opcode::Out as u32 => {
            print!("{}", char::from_u32(mem.registers[c_idx]).unwrap());

            mem.counter += 1;
        },
        // In Instruction
        o if o == Opcode::In as u32 => {
            let mut buf = [0_u8];
            let res = std::io::stdin().read(&mut buf).unwrap();

            if res == 0 {
                mem.registers[c_idx] = u32::MAX;
            } else {
                mem.registers[c_idx] = buf[0] as u32;
            }

            mem.counter += 1;
        },
        // LP Instruction
        o if o == Opcode::LP as u32 => {
            let seg_b = mem.memory[b_idx].clone();
            mem.memory[0] = seg_b;

            mem.counter = mem.registers[c_idx];
        },
        // LV Instruction
        o if o == Opcode::LV as u32 => {
            mem.registers[get(&RL, ins) as usize] = get(&VL, ins);

            mem.counter += 1;
        },
        _ => panic!()
    }
}