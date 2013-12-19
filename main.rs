extern mod rsfml;

use rsfml::window::{ContextSettings, VideoMode, event};
use rsfml::graphics::{RenderWindow, sfDefaultStyle};
use rsfml::window::keyboard;

use std::os::args;

mod cpu;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    let arg_list = args();
    if (arg_list.len() <= 1) {
        fail!("You must pass in a ROM for rustchip to read.");
    }

    // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(640, 320, 32), "rustchip", sfDefaultStyle, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    window.set_framerate_limit(60);
	let mut c8 = cpu::Cpu::new();

	c8.load(arg_list[1]);

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => { window.close()}
                event::NoEvent => { break }
                event::KeyPressed{code, alt, ..} => { 
                    if (code == keyboard::R && alt) {
                        c8 = cpu::Cpu::new();
                        c8.load(arg_list[1]);
                    }
                    if (c8.is_waiting()) {
                        let val = match code {
                            keyboard::Num1 => 1,
                            keyboard::Num2 => 2,
                            keyboard::Num3 => 3,
                            keyboard::Num4 => 0xC,
                            keyboard::Q => 4,
                            keyboard::W => 5,
                            keyboard::E => 6,
                            keyboard::R => 0xD,
                            keyboard::A => 7,
                            keyboard::S => 8,
                            keyboard::D => 9,
                            keyboard::F => 0xE,
                            keyboard::Z => 0xA,
                            keyboard::X => 0,
                            keyboard::C => 0xB,
                            keyboard::V => 0xF,
                            _ => -1
                        };
                        if (val != -1) {
                            c8.set_wait_register(val);
                        }
                    }
                }
                _ => {}
            }
        }
        
        if (!c8.is_waiting()) {
    		c8.cycle();
            c8.draw(&mut window);
            c8.update_keys();
        }
    }

}