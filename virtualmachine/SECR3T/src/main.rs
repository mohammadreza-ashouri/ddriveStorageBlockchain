use std::fs::File;
use std::io::prelude::*; 
use std::error::Error;
use std::num::ParseIntError;
use std::ptr::addr_of;





            fn run() -> Result<(), Box<Error>> {
              

            let filename:&str = "Addition.bin-runtime";
            let mut vm= Vm::new_from_file(&filename) ?;

            loop {
                match vm.next() {
                    Some(Opcode::EOF) => break,
                    Some(x) => x.describe(),
                    None => {}
                    
                }
            }

            Ok(())


            }


 

  fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len()-1))
    .step_by(2)
    .map(|i| u8::from_str_radix(&s[i..i+2], 16))
    .collect()
    // note : Iterators are lazy in Rust, so we need to call collect() at the end to consume them
  }








fn main() {
    println!("---------------------------");
    run().unwrap_or(0);

    
}



struct Vm{
    code:Vec<u8>, // contract code 
    pc:usize, // current instruction 
    stack:Vec<U256>,// for making the stack-machine in a way that EVM works but with different class of features
    hashcode:usize
    // ***** USIZE 
    // --> unsign integer: mostly used to hold CPU addresses like 32/64 bits

}



impl Vm{

    fn new_from_file(filename:&str) ->Result<Vec<u8>,Box<Error>> {
        let mut f=File::open(filename)?;
        let mut buffer =String::new();
        f.read_to_string(&mut buffer)?;

        let code=decode(&buffer)?;
        Ok(Vm {code: code, pc: 0,stack: Vec::new()})
    }

    //The "next" function will return the Opcode at the current pc, and then advance pc to the next instruction.

    fn next(&mut self) -> Option<Opcode> { // Option returns Some or None

    // ******** OPTION ***********
    /*
   Note 1 -> Options are commonly paired with pattern matching to query the presence of a value and take action

    */
    let addr = self.pc;

    match self.code[self.pc] {
        0x00 => {
            self.pc +=1;
            Some(Opcode::STOP)
        },

        0x01 => {
            self.pc +=1;
            Some(Opcode::ADD)
        },

        0x02 => {
            self.pc +=1;
            Some(Opcode::MUL)
        },

        0x60 => {
            let value = self.code[self.pc+1];
            self.pc +=2;
            Some(Opcode::PUSH1(addr,value))
        },

        0x61 => {

            let value0 = self.code[self.pc+1];
            let value1= self.code[self.pc+2];
            self.pc +=3;
            Some(Opcode::PUSH2(addr,value0,value1))
        },

        _ => {self.pc +=1; None}


    }

    }

//******** &mut self  *******/
// &mut self -> means a mutable reference to self.
// This reference type allows you to modify self without taking
//  ownership of it.

//********  self  ***********/
// note => A self referential data structure
fn interpret(&mut self){
    let maybe_op=self.next();

    //only for debugging 
    match &maybe_op {
        Some(x) => x.describe(),
        None =>{}
    }

    // the real execution

    match &maybe_op {
        Some(x) => {
            match x {
                Opcode::PUSH1(addr, value) => {
                  // Valu is u8 but we need to a 256-bit on stack...
                    self.stack.push(U256::from(*value));

                },
                Opcode::ADD(addr) => {

                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    self.stack.push(v1 + v2);

                },

                _ => {
                    // will implement later
                }
            }
        },
        None => {}
        }
    }

}



}

//################ No we can create a function that describes the opcode [important ]

impl Opcode {
    fn describe(&self) {
        match self{

            Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHALTs execution",line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::MUL(line)=> println!(("0x{:x}\tMUL\tMultiplication operation", line),

            Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
            Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),


            _ => println!("Unknown opcode!")
        }
    }
}

//################ end of the function that describes the opcodes



/// ---------- OPCODE ENUM 
enum Opcode {
    
    STOP, //0x00
    ADD, //0x01 
    MUL, //0x02


    PUSH1(u8), // 0x60
    PUSH2(u8,u8), //0x61 

    PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),// 0x7f  
    //PUSH1(u8) means that the instruction is made of 2 bytes. First byte is the instruction value, second byte is the value to push to the stack. We are going to store this value in the enumeration.
   EOF,
}

