use crate::operations;

//a public struct Field 
pub struct Field { 
    width: u32, //width represents the number of bits in the field
    lsb: u32, //lsb represents the least significant bit position of the field within the instruction word.
}

//these registers are represented as a field struct and are used to extract the reqister values from instructions as well as the Opcode 
static RA: Field = Field {width: 3, lsb: 6}; 
static RB: Field = Field {width: 3, lsb: 3}; 
static RC: Field = Field {width: 3, lsb: 0}; 
static RL: Field = Field {width: 3, lsb: 25}; 
static VL: Field = Field {width: 25, lsb: 0}; 
static OP: Field = Field {width: 4, lsb: 28};

// Define a struct for holding the state of the Universal Machine
pub struct Um{
    pub regs: Vec<u32>, // Vector of registers
    pub memory: Vec<Vec<u32>>, // Vector of vectors to represent the memory
    pub prog_count:u32, // Program counter to keep track of current instruction being executes
    pub queue: Vec<u32> // a vector that is used to hold mapped segments and unmapped segment IDs.
}

impl Um{
    // Constructor for the `Um` struct
    pub fn new() -> Self{
        Um {
            regs: vec![0; 8] , //Initialize registers to zero with a size of 8 that represents the machine's eight registers
            memory: vec![], // Memory is initialized
            prog_count: 0, // the counter is initialized to 0
            queue:vec![],// The queue vector is initialized
        }
    }

    /// Helper function to generate a bit mask with the specified number of bits set to 1.
    fn mask(&self, bits: u32) -> u32 { 
        (1 << bits) - 1 
    }
    
    //The 'get' function extracts a value from an instruction using a 'Field"
    pub fn get(&self, field: &Field, instruction: u32) -> u32 {
        (instruction >> field.lsb) & self.mask(field.width)
    }

    // A public function used to the instructions
    pub fn execute_instruction(&mut self, inst: u32){
        //there are 14 Opcodes
        pub enum Opcode{
            CMov,
            SegLoa,
            SegSto,
            Add,
            Mult,
            Div,
            Nand,
            Halt,
            MapSeg,
            UnmapSeg,
            Output,
            Input,
            LoadProg,
            Loadval,
        }
        // Match the opcode of the instruction and call the corresponding operations
        match self.get(&OP, inst){
            o if o == Opcode::CMov as u32 => {
                operations::cond_move(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));
            },
            o if o == Opcode::SegLoa as u32 => {
                operations::seg_load(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));
            },
            o if o == Opcode::SegSto as u32 => {
                operations::seg_store(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));
            },
            o if o == Opcode::Add as u32 => {
                operations::addition(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));
            },
            o if o == Opcode::Mult as u32 => {
                operations::mult(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));   
            },
            o if o == Opcode::Div as u32 => {
                operations::division(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));
            },
            o if o == Opcode::Nand as u32 => {
                operations::nand(self, self.get(&RA, inst), self.get(&RB, inst), self.get(&RC, inst));
            },
            o if o == Opcode::Halt as u32 => {
               std::process::exit(0) //the program is terminated
            },
            o if o == Opcode::MapSeg as u32 => {
                operations::map_seg(self, self.get(&RB,inst), self.get(&RC, inst));
            },
            o if o == Opcode::UnmapSeg as u32 => {
                operations::unmap_seg(self,self.get(&RC, inst))
            },
            o if o == Opcode::Output as u32 => {
                operations::output(self,self.get(&RC, inst));
            },
            o if o == Opcode::Input as u32 => {
                operations::input(self, self.get(&RC, inst));
            },
            o if o == Opcode::LoadProg as u32 => {
                operations::load_prog(self, self.get(&RB, inst),self.get(&RC, inst));
            },
            o if o == Opcode::Loadval as u32 => {
              operations::load_val(self, self.get(&RL, inst), self.get(&VL, inst));
            },
            _ =>{
                std::process::exit(0)
            }
        }
    }
}
   
