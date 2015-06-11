#[macro_use]
extern crate log;
extern crate sfml;

use sfml::window::{ContextSettings, VideoMode, event, DefaultStyle};
use sfml::graphics::RenderWindow;
use sfml::window::keyboard;

use std::env::args;

mod cpu;

fn main() {
    if args().len() <= 1 {
        panic!("You must pass in a ROM for rustchip to read.");
    }
    let file_path = args().nth(1).unwrap();

    // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(640, 320, 32), "rustchip", DefaultStyle, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_framerate_limit(60);
	let mut c8 = cpu::Cpu::new();

	c8.load(&file_path);

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => { window.close()}
                event::NoEvent => { break }
                event::KeyPressed{code, alt, ..} => {
                    if code == keyboard::Key::R && alt {
                        c8 = cpu::Cpu::new();
                        c8.load(&file_path);
                    }
                    if c8.is_waiting() {
                        let val = match code {
                            keyboard::Key::Num1 => 1,
                            keyboard::Key::Num2 => 2,
                            keyboard::Key::Num3 => 3,
                            keyboard::Key::Num4 => 0xC,
                            keyboard::Key::Q => 4,
                            keyboard::Key::W => 5,
                            keyboard::Key::E => 6,
                            keyboard::Key::R => 0xD,
                            keyboard::Key::A => 7,
                            keyboard::Key::S => 8,
                            keyboard::Key::D => 9,
                            keyboard::Key::F => 0xE,
                            keyboard::Key::Z => 0xA,
                            keyboard::Key::X => 0,
                            keyboard::Key::C => 0xB,
                            keyboard::Key::V => 0xF,
                            _ => -1
                        };
                        if val != -1 {
                            c8.set_wait_register(val);
                        }
                    }
                }
                _ => {}
            }
        }

        if !c8.is_waiting() {
    		c8.run_cycle();
            c8.draw(&mut window);
            c8.update_keys();
        }
    }

}
