use std::env;
use rum::{rumload, rumstate};
use rum::rumstate::Um;

fn main() {

   
    let input = env::args().nth(1);
   
    //using the load function we read the machine instructions from the input file & store them in a vector
    let instructions = rumload::load(input.as_deref());
   
    //this will create a new instence from our UM structure
    let mut um = Um::new();
    
    //we want to store our instructions into our memory as "segments"
    um.memory.push(instructions.clone());
   
    // This loop will run the machine until it is terminated
    loop{
        let instruction = um.memory[0][um.prog_count as usize];
        rumstate::Um::execute_instruction(&mut um, instruction)
    }
}
