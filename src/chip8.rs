use crate::{ram::Ram, cpu::{Cpu, PROGRAM_START}, cpu};
use std::fmt;

pub struct Chip8{
    memory: Ram,
    cpu: Cpu,
}

impl Chip8{
    pub fn new() -> Chip8 {
        let console = Chip8 {
           memory: Ram::new(),
           cpu: Cpu::new(),
        };
        return console 
    }

    pub fn load_rom(&mut self, data: &Vec::<u8>){
        for i in 0 .. data.len(){
            // println!("DATA WE GOT {}", data[i]);
            self.memory.write_bytes(PROGRAM_START + i as u16, data[i]);
        }

        // println!("{:?}", self.memory);
    }

    pub fn run_instruction(&mut self){
        self.cpu.run_instruction(&mut self.memory)
    }
}

impl fmt::Debug for Chip8{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "===Chip8 Cpu State===\n", ).expect("ERROR: Writing to screen");
        write!(f, "PROGRAM_COUNTER: {:?}\n", self.cpu.pc).expect("ERROR: Printing PROGRAM_COUNTER");
        write!(f, "VX: {:?}\n", self.cpu.vx).expect("ERROR: Printing cpu register");
        write!(f, "I: {:?}\n", self.cpu.i).expect("ERROR: Printing the i register");
        write!(f, "=====================\n", ).expect("ERROR: Writing to screen");
        write!(f, "\n", ).unwrap_or_default();
        Ok(())
    }
}