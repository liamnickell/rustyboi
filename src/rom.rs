use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn openRom(name: &str){
    //let romName = *name;
    let path = Path::new(name);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,why.description()),
        Ok(file) => file,
    };

    let mut romData = Vec::new();
    file.read_to_end(&mut romData);
    //println!("{:?}", romData);
    //I'm not entirely sure if what's opened is correct^
    //but we should probably upload this into mem afterwards
}