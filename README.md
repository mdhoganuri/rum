# A6 - Rust Universal Machine (RUM)

## Profiling & Optimization Results

The original version of our rum assignment with no additional optimization ran midmark at 13.16s and sandmark at 354.88s. After looking through our code and analyzing instructions, we came to the conclusion that our load program opcode had redundant instructions that were slowing down our machine. We added a simple condition to check whether register B held the value 0. If 0 was held in register B, then the machine would skip the step of copying a memory segment. This simple change completely changed our runtimes for midmark and sandmark. Our new optimized time for midmark was 0.55s and our optimized time for sandmark was 12.95s.  

## Time Consumption

- Estimated hours spent analyzing the problems posed in the assignment: 5h
- Estimated hours spent solving the problems after our analysis: 1h

## Contributors

Mike Cavallaro  
Matt Hogan  
Prof. Daniels (rumload.rs)  
