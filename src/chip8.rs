use crate::{ram::Ram, cpu::{Cpu, PROGRAM_START}, cpu};

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