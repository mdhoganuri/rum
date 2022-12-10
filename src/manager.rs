pub struct Manager {
    pub memory: Vec<Vec<u32>>,
    pub unmapped: Vec<u32>,
    pub registers: Vec<u32>,
    pub counter: u32
}

impl Manager {
    /*
    Constructor */
    pub fn new () -> Self {
        Manager {
            memory: vec![vec![0; 32]; 1],
            unmapped: vec![],
            registers: vec![0; 8],
            counter: 0
        }
    }
}