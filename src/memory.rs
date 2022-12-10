type Umi = u32;

pub struct Memory {
    memory: Vec<Vec<Umi>>,
    unmapped: Vec<Umi>,
    registers: Vec<Umi>,
    counter: u32
}

impl Memory {
    /*
    Constructor */
    pub fn init () -> Self {
        Memory {
            memory: vec![vec![]],
            unmapped: vec![],
            registers: vec![0; 8],
            counter: 0
        }
    }

    /* 
    Allocates the necessary space in memory,
    checking for unmapped segments before doing so. */
    pub fn allocate_memory (&mut self, input: u32) -> u32 {
        if self.unmapped.len() != 0 {
            self.unmapped.pop().unwrap()
        } else {
            self.memory.push(vec![0; input as usize]);
            (self.memory.len() - 1) as u32
        }
    }

    /*
    Unmaps a segment in memory */
    pub fn deallocate_memory (&mut self, index: u32) {
        self.memory.get_mut(index as usize).unwrap().clear();
        self.unmapped.push(index);
    }

    /*
    Sets a segment in memory. */
    pub fn set_segment (&mut self, index: Umi, input: Vec<Umi>) {
        self.memory[index as usize] = input.to_vec();
    }

    /*
    Gets a segment in memory. */
    pub fn get_segment (&mut self, index: Umi) -> Vec<Umi> {
        self.memory[index as usize].clone()
    }

    /*
    Sets the value of a register. */
    pub fn set_register (&mut self, index: Umi, input: Umi) {
        self.registers[index as usize] = input
    }

    /*
    Gets the value of a register. */
    pub fn get_register (&mut self, index: Umi) -> Umi {
        self.registers[index as usize]
    }

    /*
    Gets the value of the program counter. */
    pub fn get_counter (&mut self) -> u32 {
        self.counter
    }

    /*
    Sets the value of the program counter. */
    pub fn set_counter (&mut self, input: u32) {
        self.counter = input;
    }

    /*
    Increments the value of the program counter
    by one. */
    pub fn increment_counter (&mut self) {
        self.counter += 1;
    }
}