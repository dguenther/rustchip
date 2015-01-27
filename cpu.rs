extern crate rsfml;

use rsfml::graphics::{Color, Image, RenderWindow, RenderTarget, Sprite, Texture};
use rsfml::window::keyboard;

use std::io::fs::File;
use std::path::Path;
use std::rand;

pub struct Cpu {
	// These fields are public so they can be set directly in tests, not sure
	// if there's a better way to do that

	// Current opcode
	opcode: u16,

	// Memory map
	// 0x000 - 0x1FF Interpreter
	// 0x050 - 0x0A0 Fonts
	// 0x200 - 0xFFF ROM and RAM
	pub memory: [u8; 4096],

	// Registers
	pub v: [u8; 16],

	// Index register
	pub index: u16,

	// Program counter
	pub pc: u16,

	// Black and white, 64 x 32 screen
	pub graphics: [u8; 64 * 32],

	// Timer registers
	pub delay_timer: u8,
	pub sound_timer: u8,

	// Stack
	pub stack: [u16; 16],
	pub sp: u16,

	// Keys
	pub keys: [u8; 16],

	// emulator flags and values
	pub draw_flag: bool,
	pub wait_flag: bool,
	pub wait_register: u8
}

impl Cpu {
	pub fn new() -> Cpu {
		let mut init_cpu = Cpu {
			// Initialize registers and memory

			// Program is loaded into memory at 0x200
			pc: 0x200,
			opcode: 0,
			index: 0,
			sp: 0,

			graphics: [0, ..64 * 32],
			stack: [0, ..16],
			v: [0, ..16],
			memory: [0, ..4096],

			delay_timer: 0,
			sound_timer: 0,

			keys: [0, ..16],

			draw_flag: false,
			wait_flag: false,
			wait_register: 0,
		};
		init_cpu.load_fontset();
		init_cpu
	}

	fn load_fontset(&mut self) {
		let fonts: [u8; 80] =
		[
		  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
		  0x20, 0x60, 0x20, 0x20, 0x70, // 1
		  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
		  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
		  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
		  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
		  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
		  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
		  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
		  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
		  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
		  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
		  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
		  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
		  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
		  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
		];
		for i in range(0, 79u) {
			self.memory[0x050 + i] = fonts[i];
		}
	}

	pub fn load(&mut self, filename: &String) {
		let f = &Path::new(filename.to_string());
		let mut r = match File::open(f) {
			Ok(s) => s,
			Err(_) => panic!("Couldn't open file")
		};
		let mut i = 0x200;
		let bytes = match r.read_to_end() {
			Ok(s) => s,
			Err(_) => panic!("Couldn't read file")
		};
		for byte in bytes.iter() {
			self.memory[i] = *byte;
			i += 1;
		}
	}

	pub fn is_waiting(&mut self) -> bool {
		return self.wait_flag;
	}

	pub fn set_wait_register(&mut self, value: u8) {
		self.v[self.wait_register as uint] = value;
		self.wait_flag = false;
	}

	pub fn draw(&mut self, window: &mut RenderWindow) {
		let mut gfx: Vec<u8> = Vec::with_capacity(64 * 32 * 4);
		for i in range(0u, 64 * 32) {
			let value = match self.graphics[i] {
				0 => 0u8,
				_ => 0xFFu8
			};
			// SFML takes RGBA, but we only want to display black or white,
			// (all 255 or all 0) so we'll just repeat the same value 4 times
			gfx.grow(4, value);
		}

		if self.draw_flag {
			let img = match Image::create_from_pixels(64, 32, gfx.as_slice()) {
				Some(s) => s,
				None => panic!("Couldn't create image from pixel array")
			};
			let tex = match Texture::new_from_image(&img) {
				Some(s) => s,
				None => panic!("Couldn't create texture from image")
			};
			let mut sprite = match Sprite::new_with_texture(&tex) {
				Some(s) => s,
				None => panic!("Couldn't create sprite from texture")
			};
			sprite.scale2f(10f32, 10f32);
			window.clear(&Color::black());
			window.draw(&sprite);
			window.display();

			// reset draw flag
			self.draw_flag = false;
		}
	}

	pub fn update_keys(&mut self) {
		self.keys[1] = keyboard::is_key_pressed(keyboard::Key::Num1) as u8;
		self.keys[2] = keyboard::is_key_pressed(keyboard::Key::Num2) as u8;
		self.keys[3] = keyboard::is_key_pressed(keyboard::Key::Num3) as u8;
		self.keys[0xC] = keyboard::is_key_pressed(keyboard::Key::Num4) as u8;
		self.keys[4] = keyboard::is_key_pressed(keyboard::Key::Q) as u8;
		self.keys[5] = keyboard::is_key_pressed(keyboard::Key::W) as u8;
		self.keys[6] = keyboard::is_key_pressed(keyboard::Key::E) as u8;
		self.keys[0xD] = keyboard::is_key_pressed(keyboard::Key::R) as u8;
		self.keys[7] = keyboard::is_key_pressed(keyboard::Key::A) as u8;
		self.keys[8] = keyboard::is_key_pressed(keyboard::Key::S) as u8;
		self.keys[9] = keyboard::is_key_pressed(keyboard::Key::D) as u8;
		self.keys[0xE] = keyboard::is_key_pressed(keyboard::Key::F) as u8;
		self.keys[0xA] = keyboard::is_key_pressed(keyboard::Key::Z) as u8;
		self.keys[0] = keyboard::is_key_pressed(keyboard::Key::X) as u8;
		self.keys[0xB] = keyboard::is_key_pressed(keyboard::Key::C) as u8;
		self.keys[0xF] = keyboard::is_key_pressed(keyboard::Key::V) as u8;
	}

	pub fn run_cycle(&mut self) {
		// Fetch Opcode
		self.opcode = self.memory[self.pc as uint] as u16 << 8 | self.memory[self.pc as uint + 1] as u16;

		// Decode/Execute Opcode
		let op_tuple = (((self.opcode & 0xF000) >> 12) as uint, ((self.opcode & 0x0F00) >> 8) as uint,
						((self.opcode & 0x00F0) >> 4) as uint, (self.opcode & 0x000F) as uint);

		debug!("{}", op_tuple);

		match op_tuple {
			(0, 0, 0xE, 0) => {
				/* Clear screen */
				self.graphics = [0, ..64 * 32];
				self.draw_flag = true;
				self.pc += 2;
			}
			(0, 0, 0xE, 0xE) => {
				/* Return from subroutine */
				self.sp -= 1;
				self.pc = self.stack[self.sp as uint];
				debug!("Return to {}", self.pc);
			}
			(0, _, _, _) => { /* Calls RCA 1802 program at address abc */ panic!("Opcode 0NNN not implemented") }
			(1, _, _, _) => {
				/* Jumps to address NNN */
				self.pc = self.opcode & 0x0FFF;
				debug!("Jump to {}", self.pc);
			}
			(2, _, _, _) => {
				/* Calls subroutine at NNN */
				self.stack[self.sp as uint] = self.pc + 2;
				self.sp += 1;
				self.pc = self.opcode & 0x0FFF;
				debug!("Call {}", self.pc);
			}
			(3, x, _, _) => {
				/* Skips next instruction if Vx is NN */
				if self.v[x] == (self.opcode & 0x00FF) as u8 {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
				debug!("Skips instruction if V{} ({}) is {}", x, self.v[x], (self.opcode & 0x00FF));
			}
			(4, x, _, _) => { /* Skips next instruction if Vx isn't NN */ if self.v[x] != (self.opcode & 0x00FF) as u8 {self.pc += 4} else {self.pc += 2} }
			(5, x, y, 0) => { /* Skips next instruction if Vx is Vy */ if self.v[x] == self.v[y] {self.pc += 4} else {self.pc += 2} }
			(6, x, _, _) => { /* Sets Vx to NN */ self.v[x] = (self.opcode & 0x00FF) as u8; self.pc += 2 }
			(7, x, _, _) => { /* Adds NN to Vx (need to set carry?) */ self.v[x] += (self.opcode & 0x00FF) as u8; self.pc += 2 }
			(8, x, y, 0) => {
				/* Sets Vx to Vy */
				self.v[x] = self.v[y];
				self.pc += 2;
				debug!("Set V{} to V{} ({})", x, y, self.v[y]);
			}
			(8, x, y, 1) => { /* Sets Vx to Vx OR Vy */ self.v[x] = self.v[x] | self.v[y]; self.pc += 2 }
			(8, x, y, 2) => { /* Sets Vx to Vx AND Vy */ self.v[x] = self.v[x] & self.v[y]; self.pc += 2 }
			(8, x, y, 3) => { /* Sets Vx to Vx XOR Vy */ self.v[x] = self.v[x] ^ self.v[y]; self.pc += 2 }
			(8, x, y, 4) => {
				/* Adds Vy to Vx */
				if self.v[y] > 0xFF - self.v[x] {
					// set carry
					self.v[0xF] = 1;
				} else {
					self.v[0xF] = 0;
				}
				self.v[x] += self.v[y];
				self.pc += 2;
			}
			(8, x, y, 5) => {
				/* Subtracts Vy from Vx */
				if self.v[x] > self.v[y] {
					// set borrow
					self.v[0xF] = 1;
				} else {
					self.v[0xF] = 0;
				}
				self.v[x] -= self.v[y];
				self.pc += 2;
			}
			(8, x, _, 6) => {
				/* Shifts Vx right by one */
				// set VF to least significant bit
				self.v[0xF] = self.v[x] & 1;
				self.v[x] = self.v[x] >> 1;
				self.pc += 2;
			}
			(8, x, y, 7) => { /* Sets Vx to Vy minus Vx */
				if self.v[y] > self.v[x] {
					// set borrow
					self.v[0xF] = 1;
				} else {
					self.v[0xF] = 0;
				}
				self.v[x] = self.v[y] - self.v[x];
				self.pc += 2;
			}
			(8, x, _, 0xE) => {
				/* Shifts Vx left by one */
				// set VF to most significant bit
		        self.v[0xF] = self.v[x] >> 7;
		        self.v[x] = self.v[x] << 1;
		        self.pc += 2;
			}
			(9, x, y, 0) => {
				/* Skips next instruction if Vx isn't Vy */
				if self.v[x] != self.v[y] { self.pc += 4 } else { self.pc += 2 }
			}
			(0xA, _, _, _) => {
				/* Sets index register to NNN */
				self.index = self.opcode & 0x0FFF;
				self.pc += 2;
				debug!("Set I to {}", self.index);
			}
			(0xB, _, _, _) => { /* Jumps to NNN plus V0 */ self.pc = (self.opcode & 0x0FFF) + (self.v[0] as u16) }
			(0xC, x, _, _) => {
				/* Sets Vx to a random number and NN */
				let rand_num: u8 = rand::random();
				self.v[x] =  rand_num & ((self.opcode & 0x00FF) as u8);
				self.pc += 2
			}
			(0xD, x, y, h) => {
				/* Draws a sprite at (Vx, Vy) with width of 8 and height of N pixels */
				let mut pixel: u8;
				self.v[0xF] = 0;
				for y_draw in range(0, h) {
					pixel = self.memory[(self.index as uint + y_draw)];
					for x_draw in range(0, 8) {
						if pixel & (0x80 >> x_draw) != 0 {
							let calc: uint = (((self.v[x] as int + x_draw as int) % 64) + (((self.v[y] as int + y_draw as int) % 32) * 64)) as uint;
							if self.graphics[calc] == 1 {
								// Collision detection
								self.v[0xF] = 1;
							}
							self.graphics[calc] ^= 1;
						}
					}
				}
			  	self.draw_flag = true;
			  	self.pc += 2;
			}
			(0xE, x, 9, 0xE) => {
				/* Skips next instruction if key in Vx is pressed */
				if self.keys[self.v[x] as uint] == 1 {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			}
			(0xE, x, 0xA, 1) => {
				/* Skips next instruction if key in Vx isn't pressed */
				if self.keys[self.v[x] as uint] == 0 {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			}
			(0xF, x, 0, 7) => { /* Sets Vx to the value of the delay timer */ self.v[x] = self.delay_timer; self.pc += 2 }
			(0xF, x, 0, 0xA) => {
				/* Wait for a key press, store the value of the key in Vx */
				self.wait_flag = true;
				self.wait_register = x as u8;
				self.pc += 2;
			}
			(0xF, x, 1, 5) => { /* Sets delay timer to Vx */ self.delay_timer = self.v[x]; self.pc += 2 }
			(0xF, x, 1, 8) => { /* Sets sound timer to Vx */ self.sound_timer = self.v[x]; self.pc += 2 }
			(0xF, x, 1, 0xE) => {
				/* Adds Vx to I */
				if self.v[x] as u16 > 0xFFF - self.index {
					// set carry (undocumented)
					self.v[0xF] = 1;
				} else {
					self.v[0xF] = 0;
				}
				self.index += self.v[x] as u16;
				self.pc += 2;
			}
			(0xF, x, 2, 9) => {
				/* Sets I to the location of the fontset sprite for the character in Vx */
				self.index = 0x050 + (self.v[x] as u16 * 5);
				self.pc += 2;
			}
			(0xF, x, 3, 3) => {
				// Stores binary-coded decimal representation of Vx at I, I+1, and I+2
				// In other words, each digit of the number in a separate memory location
				self.memory[self.index as uint] = self.v[x] / 100;
				self.memory[self.index as uint + 1] = (self.v[x] / 10) % 10;
				self.memory[self.index as uint + 2] = self.v[x] % 10;
				self.pc += 2;
			}
			(0xF, x, 5, 5) => {
				/* Stores V0 to Vx in memory starting at address I */
				for i in range(0, x + 1) {
					self.memory[(self.index as uint + i)] = self.v[i];
				}
				self.pc += 2;
			}
			(0xF, x, 6, 5) => {
				/* Fills V0 to Vx from memory starting at address I */
				for i in range(0, x + 1) {
					self.v[i] = self.memory[(self.index as uint + i)];
				}
				self.pc += 2;
			}
			_ => panic!("Unknown instruction")
		}

		// Update timers
		if self.delay_timer > 0 {
			self.delay_timer -= 1;
		}

		if self.sound_timer > 0 {
			if self.sound_timer == 1 {
				// TODO: BEEP
			}
			self.sound_timer -= 1;
		}
	}

}
