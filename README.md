# Created and Implemented By:
    Amoy Scott and Scott Barber
## Help:
- Issac
- Cameron
- Ayman
- Will
## Implementation:
     We believe that we correctly implemented our operations that are used to execute each instruction. Although we do believe that our match statement that is used to connect our opcode with the instruction that is going to be performed could have been cleaner. We know that our implementation is not one hundred percent optimized seeing as some advent.umz and codex.umz did not perform well.
## Departure from our Design:
    Our file structure explained in our design doc is the same but our understanding of what goes into each structure is different. For example, we found out that having a type UMi = u32 was unnecessary. We also learned that main should not return any output at all. We learned that RA, RB, and RC are not the value of the registers but are, in fact, the register's position.
## Architecture:
    Using the Load function given to us by professor daniels in the lab handout, a binary file is read is extracted of its binary contents in order to get our instructions. The file this is located in is called Rumload and is used as a crate in our main in order to load each instruction into a vector which is used to help initialize our memory segment 0 created in a rumstate file. Our main is where a file is read in and holds a loop with a function that executes each instruction until terminated. This function is created in our rumstate file which also includes a struct 'Field' that has a 'width' field, used to represent a bit field in an instruction, and a field 'lsb', used to represent the least significant bit position of the field in a word. The file also holds a public struct Um which has several public fields: regs, memory, prog_count, and queue. The Um struct also has several methods. new is a constructor that initializes the Um struct's fields. mask is a private method that returns a bit mask with the specified number of bits set to 1. get is a public method that takes a reference to a field struct and an instruction word and returns the value of the specified field. The Um structs execute_instruction is the main function used for the loop in main It takes an instruction word as an argument and uses the get method to extract the opcode and register values. it uses an operations module to perform each operation that corresponds with a respective opcode and instruction. These operations are public functions created in a file called Operations.rs. Each function takes in the Um struct and a registers position in order to perform each action when necessary. This includes: cond_move, seg_load, seg_store, addition, mult, division, nand, map_seg, unmap_seg, output and input, and load_prog & load_val. We also created a lib.rs used to test each operation function before calling them in our execute_instruction function.
    
## execution of 50 million instructions:
    I believe our program would take a little over an hour. We tested this by piping a word count command on the command line when using our rumdump lab to test the binary files. our program outputs each instruction to the terminal so the command was able to count each line. Sandmark is around 11422 lines and Advent is around 312311 lines. Using our implementation, sandmark takes approximately a minute and a half to complete. Unfortunately, after 20-30 minutes of waiting, advent still had finally executed.
## Time Spent analyzing the Assignment:
     My partner and I spent a total of 3 hours analyzing the assignment. This includes us individuals re-reading the assignment if there was a place we got stuck.
## Time spent preparing our Design:
    My partner and I spent a total of 4 hours preparing our design.
## Time spent problem-solving:
    My partner and I spent a total of 5 hours implementing and problem-solving our implementation.