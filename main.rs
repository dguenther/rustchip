extern mod rsfml;

use rsfml::window::{ContextSettings, VideoMode, event};
use rsfml::graphics::{RenderWindow, sfClose};

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
    let mut window = match RenderWindow::new(VideoMode::new_init(320, 160, 32), ~"rustchip", sfClose, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    window.set_framerate_limit(60);
	let mut system = ::cpu::Cpu::new();

	system.load(arg_list[1]);

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => { window.close()}
                event::NoEvent => { break }
                _ => {}
            }
        }
		system.cycle();
        system.draw(&mut window);
        system.update_keys();
    }

}