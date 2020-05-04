use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//right not this is strictly for testing purposes

pub fn openRom(name: &str){
    //let romName = *name;
    let path = Path::new(name);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,why.description()),
        Ok(file) => file,
    };

    let mut romData = vec![];
    file.read_to_end(&mut romData);

    for i in 0x0100..romData.len(){
        print!("{:X}, ", romData[i]);
    }
    
    println!("\nDesired instruction: {:X}\n", romData[0x104])
;
}