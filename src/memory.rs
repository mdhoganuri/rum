pub struct MemoryManager {
    memory: Vec<Vec<u32>>,
    unmapped: Vec<u32>,
    registers: Vec<u32>,
}

impl MemoryManager {
    /*
    Constructor */
    pub fn init () -> Self {
        MemoryManager {
            memory: vec![vec![]],
            unmapped: vec![],
            registers: vec![]
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
}