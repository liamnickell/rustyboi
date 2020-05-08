#![allow(dead_code)]

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

mod regs;
mod clock;
mod mmu;
mod cpu;
mod gpu;

use cpu::CPU;
use gpu::GPU;
use mmu::MMU;
use std::num::Wrapping;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
// only 160*144 out of 256*256 pixels are displayed controlled by scrollx and scroly

const CYCLES_PER_UPDATE: u32 = 69833;

fn main() {


    //open rom
    let rom_file = "../Roms/tetris.gb";
    let boot_file = "../Roms/DMR_ROM.bin";
    //rom::openRom(romName);

    //set up cpu?
    //let cpu = CPU:init();


   //create window
    let mut frame: Vec<u32> = vec![0; WIDTH * HEIGHT];

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

    // construct cpu, mmu and gpu
    let mut mmu = MMU::init(rom_file, boot_file);
    let mut gpu = GPU::init(&mut mmu);
    let mut cpu = CPU::init(rom_file, &mut mmu);

    //first run cpu such that total cycles is approximately 1/60 second, then update buffer
    let mut total_cycles : u32 = 0;
    while window.is_open() {

        //run cpu
        let mut cycles_passed: u32 = 0;
        while ((cycles_passed < CYCLES_PER_UPDATE) & (total_cycles < 300)) {
            let i = cpu.cpu_cycle();
            cycles_passed += i;
            total_cycles += i;
            gpu.step(cycles_passed as u32);
        }

        //udpate window buffer with 
        for x in 0..WIDTH{
            for y in 0..HEIGHT{
                frame[x*HEIGHT + y] = gpu.output()[x*HEIGHT + y];
            }
        }
        
        window.update_with_buffer(&frame, WIDTH, HEIGHT);
    }
}