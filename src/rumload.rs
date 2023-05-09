use std::convert::TryInto;

//a function load that take an optional reference to a string as input
//returns a vector of 32-bit unsigned integers

pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        //match statement checks if the input parameter is none or contains a filename
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        //if there is no input the bufreader object reads from standard input
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };
    //mutable vector 'buf' of type 'Vec<u8>'
    //'read_to_end' method reads all the remaining bytes from the input
    
    let mut buf = Vec::<u8>::new(); 
    raw_reader.read_to_end(&mut buf).unwrap();

    // vector 'instructions' of type 'Vec<u32>'
    let instructions: Vec<u32> = buf

    //'chunks_exact' with a chunk size of 4bytes. 
    //creates an iterator that yields immutable refernences to equally sized chunks (remaining gets ignored)
    .chunks_exact(4)

    //This converts the chunk from a slice of u8 bytes into a u32 integer in big-endian byte order
    // for example if the chunk is not exactly 4 bytes long, the program panics.
    .map(|x| u32::from_be_bytes(x.try_into().unwrap())) .collect();
        instructions
}