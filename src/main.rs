mod ram;
mod chip8;
mod cpu;

use std::fs::File;
use std::io::Read;
use std::env;

use chip8::Chip8;

// youtube: https://www.youtube.com/watch?v=cWRopgPRC4M -> Stopped at 1:37:00
// Need to figure out why he moves by 8 bits and does a logical OR between the high and lo

fn main() {
    let mut file = File::open("./data/Invaders.ch8").expect("File not found !");
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer).expect("Error reading to the buffer");
    let args: Vec<String> = env::args().collect();

    let mut debug = false;

    if args.len() >= 2{
        if args[1] == "debug"{
            debug = true;
        }
    }

    let mut console = Chip8::new();
    console.load_rom(&buffer);

    loop{
        console.run_instruction();
        if debug == true{
            println!("{:?}", console);
        }
    }

}
