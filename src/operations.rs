use crate::rumstate::Um;
use std::io::{Write, Read};

//listed below are all public functions that are used to execute and perfrom the instruction when called from their respective Opcode In the Rum State File

//a function that perfroms the conditional move operation
pub fn cond_move(um: &mut Um, a: u32, b: u32, c: u32){
    if um.regs[c as usize] != 0 {
        um.regs[a as usize] = um.regs[b as usize];
    }
    um.prog_count += 1;
}

//a function that perfroms the Segement Load operation
pub fn seg_load(um: &mut Um, a: u32, b: u32, c: u32){
    let val = um.memory[um.regs[b as usize]as usize][um.regs[c as usize] as usize];
    um.regs[a as usize] = val;
    um.prog_count += 1;
}

//a function that perfroms the Segment Store operation
pub fn seg_store(um: &mut Um, a: u32, b: u32, c: u32){
    um.memory[um.regs[a as usize]as usize][um.regs[b as usize] as usize] = um.regs[c as usize];
    um.prog_count += 1;
}

//a function that perfroms the Addition operation with values in a given registers position and puts the value in register A's position
pub fn addition(um: &mut Um, a: u32, b: u32, c: u32){
    let result = (um.regs[b as usize] as u64 +(um.regs[c as usize]) as u64) % (2_u64.pow(32)) as u64; 
    um.regs[a as usize] = result as u32; 
    um.prog_count +=1;
}

//a function that perfroms the Multiplication operation with values in a given registers position and puts the value in register A's position
pub fn mult(um: &mut Um, a: u32, b: u32, c: u32){
    let result = (um.regs[b as usize] as u64 * (um.regs[c as usize]) as u64) % (2_u64.pow(32)) as u64; 
    um.regs[a as usize] = result as u32; 
    um.prog_count +=1;
}

//a function that perfroms the Division operation with values in a given registers position and puts the value in register A's position
pub fn division(um: &mut Um, a: u32, b: u32, c: u32){
    let result = um.regs[b as usize] as u64 / (um.regs[c as usize]) as u64;
    um.regs[a as usize] = result as u32; 
    um.prog_count +=1;
}

//a function that perfroms the Bitwise NAND operation with values in a given registers position and puts the value in register A's position
pub fn nand(um: &mut Um, a: u32, b: u32, c: u32){
    um.regs[a as usize] = !((um.regs[b as usize] as u32) & (um.regs[c as usize] as u32));
    um.prog_count +=1;
}

//a function that perfroms the Map Segemnt operation Here a new segment is created with a number of words equal to the value in register c
pub fn map_seg(um: &mut Um, b: u32, c: u32){ 
    if um.queue.len() > 0 {
        let index = um.queue.pop().unwrap();
        um.memory[index as usize] = vec![0; um.regs[c as usize]as usize];
        um.regs[b as usize] = index;
    } else {
        um.memory.push(vec![0; um.regs[c as usize]as usize]);
        um.regs[b as usize] = (um.memory.len() - 1) as u32;
    }
    um.prog_count +=1;
}

//a function that perfroms the Unmap Segment operation A segment in memory is unmapped
pub fn unmap_seg(um: &mut Um, c: u32){
    let index = um.regs[c as usize] as usize;
    um.queue.push(index as u32);
    um.prog_count += 1;
}

//a function that perfroms the output operation; the value in register C is displayed to the I/O device
pub fn output(um: &mut Um, c: u32){
   let out = um.regs[c as usize] as u8;
   std::io::stdout().write(&[out]).unwrap();
   um.prog_count+=1;
}

//a function that perfroms the input operation; The UM waits for an input on the I/O device and Loads the value in register C
pub fn input(um: &mut Um, c: u32){
    match std::io::stdin().bytes().next() {
        Some(byte) => {
            // Got a byte successfully
            um.regs[c as usize] = byte.unwrap() as u32;
        },
        None => {
            // End of input
            um.regs[c as usize] = u32::MAX;
        },
    }
    um.prog_count+=1;
}

//a function that perfroms the Load Program operation that loads a new program into memory
pub fn load_prog(um: &mut Um, b: u32, c: u32){
    um.memory[0] = um.memory[um.regs[b as usize] as usize].clone();
    um.prog_count = um.regs[c as usize];
}

//a function that perfroms the Load Value operation and loads a value into a specified register.
pub fn load_val(um: &mut Um, a: u32, b: u32){
    um.regs[a as usize] = b;
    um.prog_count+=1;
}
