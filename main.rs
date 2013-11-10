extern mod rsfml;

use rsfml::window::{ContextSettings, VideoMode};
use rsfml::graphics::{RenderWindow, sfClose};

mod cpu;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(320, 160, 32), ~"SFML Example", sfClose, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

	let mut system = ::cpu::Cpu::new();

	system.load("/Users/derekguenther/Downloads/c8games/TETRIS");
	//system.print_mem();

    loop {
		system.cycle();
        system.draw(&mut window);
	}

}