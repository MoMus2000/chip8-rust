use crate::ram::Ram;
use std::thread::sleep;
use std::time::Duration;
pub const PROGRAM_START : u16 = 0x200;

pub struct Cpu{
    pub vx: [u8; 16],
    pub pc: u16,
    pub i: u16,
    pub prev_pc: u16
}

impl Cpu {
    pub fn new() -> Cpu{
        let cpu = Cpu{
            vx : [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0
        };
        return cpu
    }

    pub fn read_vx_register(&mut self, index: usize) -> u8 {
        return self.vx[index]
    }

    pub fn write_vx_register(&mut self, index: usize, value: u8){
        // println!("AT INDEX {}",index);
        // println!("WRITING {}",value);
        self.vx[index] = value
    }

    pub fn run_instruction(&mut self, memory: &mut Ram){
        // We are trying to create a 16 bit instruction
        // For that we fetch two bytes and then combine the two together using a bitwise |

        let hi = memory.read_bytes(self.pc) as u16;
        let lo = memory.read_bytes(self.pc+1) as u16;

        let instruction = (hi << 8) | lo; // Creating a 16 bit instruction
        // println!("Instruction Read: {:#x?}, hi: {:#x?}, lo: {:#x?}", instruction, hi, lo);

        // println!("Binary Read for hi: {:016b}", hi);
        // println!("Binary Read for hi with 8 bits shifted to left: {:016b}", hi << 8);
        // println!("Binary Read for lo: {:016b}", lo);
        // println!("Binary When We OR hi and lo : {:016b}", instruction); // We get back a 16 bit (2 byte) instruction.
        
        /*
            nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
            n or nibble - A 4-bit value, the lowest 4 bits of the instruction
            x - A 4-bit value, the lower 4 bits of the high byte of the instruction
            y - A 4-bit value, the upper 4 bits of the low byte of the instruction
            kk or byte - An 8-bit value, the lowest 8 bits of the instruction  
            
            <---hi--> <---lo-->
            0000 0000 0000 0000

            Chip 8 is big endian -> most significant to least significant
        */

        let nnn = instruction & 0x0fff;
        let nn = instruction & 0x00ff;
        let n = (instruction & 0x000F) as u8;
        let x = (instruction & 0x0F00) >> 8 as u8; // we shift 8 bits to the right as right is LSB. 
        let y = (instruction & 0x00F0)  >> 4 as u8; // we shift 4 bits to the right as right is LSB.

        // println!("Instruction: {:016b}", instruction);
        // println!("MASK: 0xFFF0 {:016b}", 0xFFF0);
        // println!("NNN:         {:016b}", nnn);
        // // println!("N: {:b}", n);
        // println!("x: {:08b}", x);
        // println!("y: {:08b}", y);

        // println!("OpCode {:b}", instruction & 0xF000);

        // Both print statements below will yield the same outcome.
        // println!("OpCode shifted right by 12: {:08b}" , (instruction & 0xF000) >> 12);
        // println!("OpCode (HEX): {:#x}" , (instruction >> 12));

        if self.prev_pc == self.pc {
            panic!("Executing same counter twice !")
        }

        self.prev_pc = self.pc;

        if hi == 0 && lo == 0{
            panic!("STOP");
        }

        let code = instruction >> 12; 
        
        // sleep(Duration::new(2, 0));

        match code {
            0x1 => {
                self.pc = nnn
            }
            0x3 => {
                let x_value = self.read_vx_register(x as usize);
                if x_value == nn as u8{
                    self.pc += 4;
                }
                else{
                    self.pc += 2;
                }
            }
            0x6 => {
                self.write_vx_register(x as usize, nn as u8);
                self.pc += 2
            }
            0x7 => {
                let x_value = self.read_vx_register(x as usize);
                self.write_vx_register(x as usize, x_value.wrapping_add(nn as u8));
                self.pc += 2
            }
            0xA => {
                self.i = nnn;
                self.pc += 2
            }
            0xD => {
                self.draw(memory, x as u8, y as u8, n);
                self.pc += 2
            }
            0xF => {
                let x_value = self.read_vx_register(x as usize);
                self.i += x_value as u16;
                self.pc += 2
            }
            _ => panic!("Instruction not found ! {:#x?}", code)
        }


    }

    pub fn draw(&mut self, ram: &mut Ram, x_coordinate: u8, y_coordinate: u8, height: u8){
        for h in 0 .. height{
            let mut b = ram.read_bytes(self.i+h as u16);
            for _ in 0 .. 8{
                let bit_value = (b & 0b1000_0000) >> 7;
                match bit_value {
                    0=>{
                        print!("_");
                    }
                    1 =>{
                        print!("#");
                    }
                    _ => unreachable!()
                }
                b = b << 1;
            }
            print!("\n");
        }
        print!("\n")
    }
}