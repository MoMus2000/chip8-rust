use crate::ram::Ram;
pub const PROGRAM_START : u16 = 0x200;

pub struct Cpu{
    vx: [u8; 16],
    pc: u16,
    i: u16
}

impl Cpu {
    pub fn new() -> Cpu{
        let cpu = Cpu{
            vx : [0; 16],
            pc: PROGRAM_START,
            i: 0,
        };
        return cpu
    }

    pub fn run_instruction(&mut self, memory: &mut Ram){
        // We are trying to create a 16 bit instruction
        // For that we fetch two bytes and then combine the two together using a bitwise |

        let hi = memory.read_bytes(self.pc) as u16;
        let lo = memory.read_bytes(self.pc+1) as u16;

        let instruction = (hi << 8) | lo; // Creating a 16 bit instruction
        println!("Instruction Read: {:#x?}, hi: {:#x?}, lo: {:#x?}", instruction, hi, lo);

        println!("Binary Read for hi: {:016b}", hi);
        println!("Binary Read for hi with 8 bits shifted to left: {:016b}", hi << 8);
        println!("Binary Read for lo: {:016b}", lo);
        println!("Binary When We OR hi and lo : {:016b}", instruction); // We get back a 16 bit (2 byte) instruction.
        
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

        let nnn = (instruction & 0x0FFF);
        let n = (instruction & 0x000F) as u8;
        let x = (instruction & 0x0F00) >> 8 as u8; // we shift 8 bits to the right as right is LSB. 
        let y = (instruction & 0x00F0)  >> 4 as u8; // we shift 4 bits to the right as right is LSB.

        println!("Instruction: {:016b}", instruction);
        println!("MASK: 0xFFF0 {:016b}", 0xFFF0);
        println!("NNN:         {:016b}", nnn);
        // println!("N: {:b}", n);
        println!("x: {:08b}", x);
        println!("y: {:08b}", y);

        println!("OpCode {:b}", instruction & 0xF000);

        // Both print statements below will yield the same outcome.
        println!("OpCode shifted right by 12: {:08b}" , (instruction & 0xF000) >> 12);
        println!("OpCode (HEX): {:#x}" , (instruction >> 12));

        if hi == 0 && lo == 0{
            panic!("STOP");
        }

        // match {

        //     _ => panic!("Instruction not found !")
        // }

        // panic!("STOP EXECUTION");
        self.pc += 2;

    }
}