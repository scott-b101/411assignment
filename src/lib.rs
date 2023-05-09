
pub mod rumload;
pub mod operations;
pub mod rumstate;


// Below are hardcoded tests used to check if our functions work properly before using them in our Rumstate file
#[cfg(test)]
mod tests {
    use crate::{ rumstate, operations};

    
    #[test]

    fn check_cmove1(){

        let mut um = rumstate::Um::new();
        let a = 5;
        let b = 7;
        let c = 2;
            
        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 10;
        
       println!("{},{},{}", um.regs[a as usize], um.regs[b as usize],um.regs[c as usize]);

        let result = 100;
       
        operations::cond_move(&mut um,a,b,c);

        println!("{},{},{}", um.regs[a as usize], um.regs[b as usize],um.regs[c as usize]);

        assert_eq!(um.regs[a as usize], result)

    }

    #[test]
    fn check_cmove2(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 0;
       
        let result = 223;
       
        operations::cond_move(&mut um,a,b,c);
        
        assert_eq!(um.regs[a as usize], result)
    }
    #[test]
    fn check_segload1(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        // um.regs[a as usize] = 20;
        // um.regs[b as usize] = 10;
        // um.regs[c as usize]= 15;

        um.memory = vec![vec![222,25,77,9,10], vec![23,66,81,2], vec![0,1,2,3,4,45,6,78], vec![7,8,9,6,4,2,4,6]];
        let temp:Vec<u32> = vec![12,34,56,7,89];
        um.memory.push(temp);  
          
        //println!("{:?}", um.memory.len());

        let result = um.memory[um.regs[b as usize]as usize][um.regs[c as usize] as usize];
        //println!("{:?}", result);
        operations::seg_load(&mut um,a,b,c);
        assert_eq!(um.regs[a as usize], result)
    }
    #[test]
    fn check_segstore(){
        let mut um = rumstate::Um::new();
        let a = 7; 
        let b = 2;
        let c = 4;



        um.memory = vec![vec![222,25,77,9,10], vec![23,66,81,2],vec![7,8,9,6,4,2,4,6]];
        let temp:Vec<u32> = vec![12,34,56,7,89];
        um.memory.push(temp);  

        let result = um.regs[c as usize];
    
        operations::seg_store(&mut um,a,b,c);
    
        assert_eq!(um.memory[um.regs[a as usize]as usize][um.regs[b as usize] as usize], result)
    }
    #[test]
    fn check_add1(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 10;
       
        let result = (um.regs[b as usize] as u64 + (um.regs[c as usize]) as u64) % (2_u64.pow(32)) as u64;
       
        operations::addition(&mut um,a,b,c);
        
        assert_eq!(um.regs[a as usize], result as u32)
    }
    #[test]
    fn check_mult1(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 10;
       
        let result = (um.regs[b as usize] as u64 * (um.regs[c as usize]) as u64) % (2_u64.pow(32)) as u64;
       
        operations::mult(&mut um,a,b,c);
        
        assert_eq!(um.regs[a as usize], result as u32)
    }
    #[test]
    fn check_div1(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 10;
       
        let result = um.regs[b as usize] as u64 / (um.regs[c as usize]) as u64;
       
        operations::division(&mut um,a,b,c);
        
        assert_eq!(um.regs[a as usize], result as u32)
    }

    #[test]
    fn check_nand1(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 10;
       
        let result = !(um.regs[b as usize] & um.regs[c as usize]);
       
        operations::nand(&mut um,a,b,c);
        
        assert_eq!(um.regs[a as usize], result)
    }

    #[test]
    fn check_nand2(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 500;
        um.regs[b as usize] =500;
        um.regs[c as usize]= 10;
       
        let result = !(um.regs[b as usize] & um.regs[c as usize]);
       
        operations::nand(&mut um,a,b,c);
        
        assert_eq!(um.regs[a as usize], result)
    }
    #[test]
    fn check_output1(){
        let mut um = rumstate::Um::new();
        let a = 5; 
        let b = 7;
        let c = 2;

        um.regs[a as usize] = 223;
        um.regs[b as usize] =100;
        um.regs[c as usize]= 10;
       
        let result = um.regs[c as usize];
       
        operations::output(&mut um,c);
        
        assert_eq!(um.regs[c as usize], result)
    }

}
