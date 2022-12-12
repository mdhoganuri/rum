# A6 - Rust Universal Machine (RUM)

## Implementation Review

Although not without difficulty, we are happy to say that our Rust Universal Machine (RUM) is fully implemented and with a fairly nice benchmark time of 18.64s on midmark.um.

Our only real difficulty on this particular program was in terms of interpretation. At first, it took some time to wrap our heads aorund how we would deal with representing memory. Once we figured this out, it took some time to debug our instructions and we ran into the issue of our instructions vector being in the wrong location. Thankfully, we were able to receive some words of advice from a TA and resolve this issue propmptly.

## Solution Architecture

### main.rs

The main file is simply responsible for calling our loader (rumload.rs) and pushing the loaded instructions into the memory manager (manager.rs). From here, instructions are run (instructions.rs) on the manager until the halt instruction is received.

### rumload.rs

Rumload is a module provided to us in the rumdump lab. Its purpose is to read and return a vector of instruction words from the provided file path.

### manager.rs

Manager is a module dedicated to the representation of memory in our universal machine. Specifically, it holds a struct which contains a two-dimensional u32 vector for memory segments, a u32 vector holding the indices of unmapped segments, a u32 vector for registers, and a u32 variable for the program counter.

### instructions.rs

The Instructions module acts as a library of pseduo-functions called by use of an opcode match statement. This opcode is attained by deconstructing the provided instruction word using different fields that define registers, opcodes, and values. If an invalid opcode/instruction is provided, the program will panic. Otherwise, it will execute the proper instruction.

## Time Consumption

- Estimated hours spent analyzing the problems posed in the assignment: 8h
- Estimated hours spent solving the problems after our analysis: 3h

## Contributors

Mike Cavallaro  
Matt Hogan  
Prof. Daniels (rumload.rs)  
