extern crate minifb;

use minifb::{Key, Window, WindowOptions};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    //open rom
    let romName = "../Roms/tetris.gb";
    let path = Path::new(romName);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,why.description()),
        Ok(file) => file,
    };
    let mut rom = Vec::new();
    file.read_to_end(&mut rom);
    println!("{:?}", rom);
    //I'm not entirely sure if what's opened is correct^
    //but we should probably upload this into mem afterwards


   //create window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rusty Boi ;)",
        WIDTH,
        HEIGHT,
        minifb::WindowOptions {
                resize: true, // TODO allow resize
                scale: minifb::Scale::X4,
                ..minifb::WindowOptions::default()
            }, //WindowOptions::Default
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() {
        
        for x in 0..WIDTH{
            for y in 0..HEIGHT{
                buffer[x*HEIGHT + y] = 0xff0000;
            }
        }
        
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT);
    }
}