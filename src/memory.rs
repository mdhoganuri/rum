type Umi = u32;

pub struct Memory {
    memory: Vec<Vec<Umi>>,
    unmapped: Vec<Umi>,
    registers: Vec<Umi>,
}

impl Memory {
    /*
    Constructor */
    pub fn init () -> Self {
        Memory {
            memory: vec![vec![]],
            unmapped: vec![],
            registers: vec![0; 8]
        }
    }

    /* 
    Allocates the necessary space in memory,
    checking for unmapped segments before doing so. */
    pub fn allocate_memory (&mut self) -> Option<&Vec<u32>> {
        if self.unmapped.len() == 0 {
            return self.reclaim_memory();
        }

        self.memory.push(vec![0; 32]);
        self.memory.last()
    }

    /*
    Reclaims space in memory from an unmapped
    segment in order to prevent the unnecesdary
    allocation of more memory. */
    fn reclaim_memory (&mut self) -> Option<&Vec<u32>> {
        return self.memory.get(self.unmapped.pop().unwrap() as usize);
    }

    /*
    Unmaps a segment in memory */
    pub fn deallocate_memory (&mut self, index: u32) {
        self.memory.get_mut(index as usize).unwrap().clear();
        self.unmapped.push(index);
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
}